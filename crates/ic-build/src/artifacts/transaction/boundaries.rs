//! Lazy filesystem facts scoped to one transaction's pre-effect boundary checks.

use super::{
    ArtifactCacheError, ResolvedCargoBuildInputs, canonicalize_allow_missing, canonicalize_path,
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Default)]
pub(super) struct CargoInputBoundaries {
    exclusions: BTreeMap<PathBuf, PathBuf>,
    inputs: BTreeMap<PathBuf, (PathBuf, bool)>,
}

impl CargoInputBoundaries {
    pub(super) fn watches(
        &mut self,
        resolved: &ResolvedCargoBuildInputs,
        candidate: &Path,
    ) -> Result<bool, ArtifactCacheError> {
        for exclusion in resolved.exclusions() {
            let exclusion = match self.exclusions.entry(exclusion.clone()) {
                std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
                std::collections::btree_map::Entry::Vacant(entry) => {
                    let path = canonicalize_allow_missing(exclusion).map_err(|source| {
                        ArtifactCacheError::Io {
                            operation: "resolve Cargo build input exclusion",
                            path: exclusion.clone(),
                            source,
                        }
                    })?;
                    entry.insert(path)
                }
            };
            if candidate.starts_with(exclusion) {
                return Ok(false);
            }
        }
        for input in resolved.inputs() {
            let (path, is_directory) = match self.inputs.entry(input.path().to_owned()) {
                std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
                std::collections::btree_map::Entry::Vacant(entry) => {
                    let path =
                        canonicalize_path(input.path(), "canonicalize resolved Cargo build input")?;
                    let is_directory = fs::metadata(&path)
                        .map_err(|source| ArtifactCacheError::Io {
                            operation: "inspect resolved Cargo build input",
                            path: path.clone(),
                            source,
                        })?
                        .is_dir();
                    entry.insert((path, is_directory))
                }
            };
            if candidate == *path || (*is_directory && candidate.starts_with(path)) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
