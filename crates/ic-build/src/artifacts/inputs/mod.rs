//! Standalone filesystem evidence for caller-owned build inventories.

mod depfiles;
#[cfg(test)]
mod tests;

pub use depfiles::{discover_cargo_depfile_inputs, parse_cargo_depfile, read_cargo_depfile_inputs};

use crate::artifacts::{
    cache_fs::canonicalize_allow_missing,
    digest::{InputDigest, InputPathError, InputTreeEntry, InputTreeWalk},
};
use sha2::{Digest as _, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fmt, io,
    path::{Component, Path, PathBuf},
};

/// Caller-selected filesystem roots, exclusions and opaque non-file identity.
///
/// Paths must be absolute. No source, output or tool policy is selected implicitly.
/// Excluded entry names apply to children, not to explicitly declared roots.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputEvidenceSpec {
    identity: Vec<u8>,
    roots: Vec<PathBuf>,
    excluded_roots: Vec<PathBuf>,
    excluded_names: Vec<OsString>,
}

/// Per-path bytes, directory membership or explicit absence observed before work.
///
/// Capture/validation requires caller custody over sources and parent directories;
/// this value does not lock files, admit a release or retain output artifacts.
#[derive(Clone, Debug)]
pub struct InputEvidence {
    spec: InputEvidenceSpec,
    entries: BTreeMap<PathBuf, InputEvidenceValue>,
}

/// Exact filesystem fact recorded for an input path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputEvidenceValue {
    /// The selected path did not exist.
    Missing,
    /// A directory whose non-excluded children were traversed.
    Directory,
    /// Raw SHA-256 of a regular file, without a digest-domain prefix.
    File {
        /// Digest of the complete file bytes.
        sha256: InputDigest,
    },
}

/// Failure to capture or validate standalone input authority.
#[derive(Debug)]
pub enum InputEvidenceError {
    /// The caller's non-file identity changed.
    IdentityChanged,
    /// Exclusion policy changed between observations.
    ExclusionsChanged,
    /// Previously observed bytes, absence or membership changed.
    ChangedInput {
        /// Changed input path.
        path: PathBuf,
    },
    /// An after-inventory path had no pre-build authority.
    UnobservedInput {
        /// Newly discovered input path.
        path: PathBuf,
    },
    /// Filesystem or input-admission failure, retaining its original cause.
    Io {
        /// Path being inspected.
        path: PathBuf,
        /// Original error.
        source: io::Error,
    },
}

impl InputEvidenceSpec {
    /// Start an empty recipe with caller-derived environment/tool/profile identity.
    #[must_use]
    pub fn new(identity: impl Into<Vec<u8>>) -> Self {
        Self {
            identity: identity.into(),
            roots: Vec::new(),
            excluded_roots: Vec::new(),
            excluded_names: Vec::new(),
        }
    }

    /// Declare a file, recursive directory, or not-yet-existing input.
    #[must_use]
    pub fn with_input(mut self, path: impl Into<PathBuf>) -> Self {
        self.roots.push(path.into());
        self
    }

    /// Exclude one caller-owned generated/output tree.
    #[must_use]
    pub fn with_excluded_root(mut self, path: impl Into<PathBuf>) -> Self {
        self.excluded_roots.push(path.into());
        self
    }

    /// Skip this exact child name at every directory level.
    #[must_use]
    pub fn with_excluded_entry_name(mut self, name: impl Into<OsString>) -> Self {
        self.excluded_names.push(name.into());
        self
    }
}

impl InputEvidence {
    /// Capture input authority without building or publishing an artifact cache.
    ///
    /// Directories record membership, including empty directories. Missing roots
    /// are explicit evidence; links and special files refuse. Keep a genuine
    /// source write-exclusion boundary through capture, work and validation.
    ///
    /// # Errors
    /// Returns non-absolute paths, invalid exclusions, unsupported files or I/O.
    pub fn capture(spec: &InputEvidenceSpec) -> Result<Self, InputEvidenceError> {
        let mut spec = spec.clone();
        for path in spec.roots.iter().chain(&spec.excluded_roots) {
            if !path.is_absolute() {
                return Err(input_io(
                    path,
                    io::Error::new(io::ErrorKind::InvalidInput, "input path must be absolute"),
                ));
            }
        }
        for name in &spec.excluded_names {
            let mut components = Path::new(name).components();
            if !matches!(components.next(), Some(Component::Normal(_)))
                || components.next().is_some()
            {
                return Err(input_io(
                    Path::new(name),
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "excluded entry must be one normal name",
                    ),
                ));
            }
        }
        spec.roots.sort();
        spec.roots.dedup();
        spec.excluded_roots = spec
            .excluded_roots
            .iter()
            .map(|path| canonicalize_allow_missing(path).map_err(|source| input_io(path, source)))
            .collect::<Result<_, _>>()?;
        spec.excluded_roots.sort();
        spec.excluded_roots.dedup();
        spec.excluded_names.sort();
        spec.excluded_names.dedup();
        let mut entries = BTreeMap::new();
        let mut visited = BTreeSet::new();
        let mut walk =
            InputTreeWalk::evidence(&spec.excluded_roots, &spec.excluded_names, &mut visited);
        for root in &spec.roots {
            walk.visit(root, root, true, &mut |entry| {
                let (path, value) = match entry {
                    InputTreeEntry::File { path, .. } => {
                        let hash = ic_host_fs::read::hash_file_no_follow(path, u64::MAX)
                            .map_err(io::Error::from)?;
                        (
                            path,
                            InputEvidenceValue::File {
                                sha256: InputDigest::from_bytes(*hash.sha256.as_bytes()),
                            },
                        )
                    }
                    InputTreeEntry::Directory { path, .. } => (path, InputEvidenceValue::Directory),
                    InputTreeEntry::Missing { path } => (path, InputEvidenceValue::Missing),
                };
                if entries
                    .insert(path.to_owned(), value)
                    .is_some_and(|first| first != value)
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "input changed across overlapping observations",
                    ));
                }
                Ok(())
            })
            .map_err(|source| input_io(root, source))?;
        }
        Ok(Self { spec, entries })
    }

    /// Borrow the immutable per-path observations for consumer-owned projection.
    #[must_use]
    pub const fn entries(&self) -> &BTreeMap<PathBuf, InputEvidenceValue> {
        &self.entries
    }

    /// Hash the caller identity and UTF-8 path/value map using big-endian lengths.
    ///
    /// Fields are identity, then lexically sorted path/value pairs. Values are
    /// `absent`, `directory`, or lowercase raw SHA-256 hexadecimal. Each field
    /// starts with its u64 byte length in big-endian order. This preserves the
    /// existing length-framed consumer evidence encoding without selecting a
    /// cache namespace or adding a domain/version prefix.
    ///
    /// # Errors
    /// Returns non-UTF-8 paths that this evidence encoding cannot represent.
    pub fn digest(&self) -> Result<InputDigest, InputEvidenceError> {
        let mut ordered = self
            .entries
            .iter()
            .map(|(path, value)| {
                path.to_str().map(|key| (key, value)).ok_or_else(|| {
                    input_io(
                        path,
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "input digest requires UTF-8 paths",
                        ),
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        ordered.sort_unstable_by_key(|(path, _)| *path);
        let mut digest = Sha256::new();
        framed_field(&mut digest, &self.spec.identity);
        for (path, value) in ordered {
            framed_field(&mut digest, path.as_bytes());
            match value {
                InputEvidenceValue::Missing => framed_field(&mut digest, b"absent"),
                InputEvidenceValue::Directory => framed_field(&mut digest, b"directory"),
                InputEvidenceValue::File { sha256 } => {
                    framed_field(&mut digest, sha256.to_hex().as_bytes());
                }
            }
        }
        Ok(InputDigest::from_bytes(digest.finalize().into()))
    }

    /// Validate an expanded/replaced inventory against pre-work authority.
    ///
    /// Rechecks every original root even when a changed/disappearing depfile no
    /// longer names it. Late bytes cannot become pre-build authority. A newly
    /// named missing path is admissible only when a complete earlier directory
    /// scan proved its first child absent and that child was not excluded.
    ///
    /// # Errors
    /// Returns changed identity/policy/inputs, unobserved inputs or I/O failures.
    pub fn validate_after(&self, after: &Self) -> Result<(), InputEvidenceError> {
        if self.spec.identity != after.spec.identity {
            return Err(InputEvidenceError::IdentityChanged);
        }
        if self.spec.excluded_roots != after.spec.excluded_roots
            || self.spec.excluded_names != after.spec.excluded_names
        {
            return Err(InputEvidenceError::ExclusionsChanged);
        }
        let retained = Self::capture(&self.spec)?;
        for (path, before) in &self.entries {
            if retained.entries.get(path) != Some(before)
                || after.entries.get(path).is_some_and(|value| value != before)
            {
                return Err(InputEvidenceError::ChangedInput { path: path.clone() });
            }
        }
        for (path, value) in retained.entries.iter().chain(&after.entries) {
            if self.entries.contains_key(path) {
                continue;
            }
            if self.observed_absence(path) {
                if *value == InputEvidenceValue::Missing {
                    continue;
                }
                return Err(InputEvidenceError::ChangedInput { path: path.clone() });
            }
            return Err(InputEvidenceError::UnobservedInput { path: path.clone() });
        }
        Ok(())
    }

    fn observed_absence(&self, path: &Path) -> bool {
        let Some(directory) = path
            .ancestors()
            .skip(1)
            .find(|ancestor| self.entries.get(*ancestor) == Some(&InputEvidenceValue::Directory))
        else {
            return false;
        };
        let Ok(suffix) = path.strip_prefix(directory) else {
            return false;
        };
        if !suffix
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
        {
            return false;
        }
        let Some(Component::Normal(name)) = suffix.components().next() else {
            return false;
        };
        let first = directory.join(name);
        !self
            .spec
            .excluded_names
            .iter()
            .any(|excluded| excluded == name)
            && !self
                .spec
                .excluded_roots
                .iter()
                .any(|root| first.starts_with(root))
            && !self.entries.contains_key(&first)
    }
}

fn framed_field(digest: &mut Sha256, bytes: &[u8]) {
    digest.update(
        u64::try_from(bytes.len())
            .expect("field length fits u64")
            .to_be_bytes(),
    );
    digest.update(bytes);
}

fn input_io(path: &Path, source: io::Error) -> InputEvidenceError {
    let context_path = source
        .get_ref()
        .and_then(|error| error.downcast_ref::<InputPathError>())
        .map_or_else(|| path.to_owned(), |error| error.path.clone());
    InputEvidenceError::Io {
        path: context_path,
        source,
    }
}

impl fmt::Display for InputEvidenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdentityChanged => formatter.write_str("non-file input identity changed"),
            Self::ExclusionsChanged => formatter.write_str("input exclusion policy changed"),
            Self::ChangedInput { path } => write!(formatter, "input changed: {}", path.display()),
            Self::UnobservedInput { path } => write!(
                formatter,
                "input was not observed before work: {}",
                path.display()
            ),
            Self::Io { path, source } => write!(
                formatter,
                "input evidence failed for {}: {source}",
                path.display()
            ),
        }
    }
}

impl std::error::Error for InputEvidenceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}
