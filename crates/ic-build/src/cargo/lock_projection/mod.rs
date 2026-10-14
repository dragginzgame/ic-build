//! Strict semantic lockfile comparison after caller-selected workspace version changes.

use std::fmt;

///
/// `WorkspaceVersionProjection`
///
/// One unsourced workspace package whose version may change; all other parsed
/// lockfile content must remain identical. Release version policy is caller-owned.
///

#[derive(Clone, Copy, Debug)]
pub struct WorkspaceVersionProjection<'a> {
    /// Exact package name, selecting only records with no source field.
    pub package: &'a str,
    /// Expected version in the saved input.
    pub previous: &'a str,
    /// Only permitted replacement version.
    pub next: &'a str,
}

///
/// `LockProjectionError`
///
/// Strict version projection failed without changing either input.
///

#[derive(Debug)]
#[non_exhaustive]
pub enum LockProjectionError {
    /// No changes selected, or a name/version is empty or a name repeated.
    InvalidSelection,
    /// One selected input is not valid TOML.
    Parse {
        /// `before` or `after` input role.
        input: &'static str,
        /// Original parser error.
        source: toml::de::Error,
    },
    /// The saved lockfile has no package array.
    MissingPackages,
    /// A name must identify exactly one unsourced package.
    PackageSelection {
        /// Requested workspace package.
        package: String,
        /// Number of matching unsourced records.
        matches: usize,
    },
    /// The saved selected package does not have its expected prior version.
    PreviousVersion {
        /// Requested workspace package.
        package: String,
    },
    /// The after input differs from the exact permitted projection.
    ChangedInputs,
}

impl fmt::Display for LockProjectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSelection => f.write_str("invalid workspace version selection"),
            Self::Parse { input, source } => write!(f, "invalid {input} lockfile: {source}"),
            Self::MissingPackages => f.write_str("lockfile has no package array"),
            Self::PackageSelection { package, matches } => write!(
                f,
                "workspace package {package} has {matches} unsourced records; expected one"
            ),
            Self::PreviousVersion { package } => {
                write!(f, "unexpected previous workspace version for {package}")
            }
            Self::ChangedInputs => {
                f.write_str("lockfile differs from the selected workspace version projection")
            }
        }
    }
}
impl std::error::Error for LockProjectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Verify that only explicitly selected unsourced package versions changed.
///
/// This is independent of any renderer. Parsed equality permits TOML comments
/// and table-field ordering differences, but preserves array order, dependencies,
/// source/checksum records and every other value. Registry records with the same
/// name are never projected. No file IO, dependency resolution, version selection
/// or release effect occurs. Callers own byte limits and saved-input custody.
///
/// # Errors
/// Refuses invalid/duplicate selections, parse failures, missing/ambiguous workspace
/// packages, unexpected prior versions and any other parsed difference.
pub fn verify_workspace_version_projection(
    before: &str,
    after: &str,
    projections: &[WorkspaceVersionProjection<'_>],
) -> Result<(), LockProjectionError> {
    if projections.is_empty()
        || projections.iter().enumerate().any(|(index, item)| {
            item.package.is_empty()
                || item.previous.is_empty()
                || item.next.is_empty()
                || projections[..index]
                    .iter()
                    .any(|other| other.package == item.package)
        })
    {
        return Err(LockProjectionError::InvalidSelection);
    }
    let mut expected: toml::Value =
        toml::from_str(before).map_err(|source| LockProjectionError::Parse {
            input: "before",
            source,
        })?;
    let packages = expected
        .get_mut("package")
        .and_then(toml::Value::as_array_mut)
        .ok_or(LockProjectionError::MissingPackages)?;
    for projection in projections {
        let mut matches = 0;
        for package in packages.iter_mut().filter(|package| {
            package.get("name").and_then(toml::Value::as_str) == Some(projection.package)
                && package.get("source").is_none()
        }) {
            matches += 1;
            if package.get("version").and_then(toml::Value::as_str) != Some(projection.previous) {
                return Err(LockProjectionError::PreviousVersion {
                    package: projection.package.to_owned(),
                });
            }
            package["version"] = toml::Value::String(projection.next.to_owned());
        }
        if matches != 1 {
            return Err(LockProjectionError::PackageSelection {
                package: projection.package.to_owned(),
                matches,
            });
        }
    }
    let actual: toml::Value =
        toml::from_str(after).map_err(|source| LockProjectionError::Parse {
            input: "after",
            source,
        })?;
    if expected != actual {
        return Err(LockProjectionError::ChangedInputs);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
