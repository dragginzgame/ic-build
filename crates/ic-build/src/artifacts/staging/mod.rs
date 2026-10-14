//! Invocation-owned artifact staging without cache publication or reuse.
//!
//! Callers own the parent directory, recipes and publication boundary. This
//! module allocates only its own child and removes it after the last owner is
//! dropped. Explicit retention transfers cleanup to the caller.
//! It never prunes existing artifacts or publishes a product release.

use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use ic_host_fs::{durable::write_bytes, read::read_file};

use super::cache_fs::remove_path_if_present;

const STAGE_ATTEMPTS: usize = 64;
static STAGE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub(super) struct OwnedDirectory {
    path: PathBuf,
    armed: bool,
}

impl OwnedDirectory {
    pub(super) const fn new(path: PathBuf) -> Self {
        Self { path, armed: true }
    }

    pub(super) fn path(&self) -> &Path {
        &self.path
    }

    pub(super) const fn is_armed(&self) -> bool {
        self.armed
    }

    pub(super) const fn disarm(&mut self) {
        self.armed = false;
    }

    pub(super) fn preserve(mut self) {
        self.disarm();
    }

    pub(super) fn try_cleanup(&mut self) -> io::Result<()> {
        remove_path_if_present(&self.path)?;
        self.disarm();
        Ok(())
    }

    pub(super) fn cleanup(mut self) -> io::Result<()> {
        self.try_cleanup()
    }

    pub(super) fn cleanup_on_drop(&mut self) {
        if self.armed {
            let _ = remove_path_if_present(&self.path);
            self.disarm();
        }
    }
}

impl Drop for OwnedDirectory {
    fn drop(&mut self) {
        self.cleanup_on_drop();
    }
}

/// Temporary artifact directory retained until its owner is closed or dropped.
///
/// The caller keeps custody of the parent and allocated child against unrelated
/// writers. Drop performs best-effort cleanup of this invocation-owned child;
/// use [`Self::close`] when cleanup failures must reach the caller. No shared
/// cache entry is created, reused or removed.
#[derive(Debug)]
pub struct ArtifactStagingDirectory {
    directory: OwnedDirectory,
}

impl ArtifactStagingDirectory {
    /// Allocate a distinct staging child beneath an existing caller-owned directory.
    ///
    /// The parent is not created. Existing children are never reused or removed.
    ///
    /// # Errors
    /// Returns the filesystem error for a missing or unwritable parent, or
    /// `AlreadyExists` when every allocation attempt collides.
    pub fn new(parent: &Path) -> io::Result<Self> {
        for _ in 0..STAGE_ATTEMPTS {
            let sequence = STAGE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let root = parent.join(format!(
                ".ic-build-artifact-stage-{}-{sequence}",
                std::process::id()
            ));
            match fs::create_dir(&root) {
                Ok(()) => {
                    return Ok(Self {
                        directory: OwnedDirectory::new(root),
                    });
                }
                Err(source) if source.kind() == io::ErrorKind::AlreadyExists => {}
                Err(source) => return Err(source),
            }
        }
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!(
                "failed to allocate artifact staging beneath {}",
                parent.display()
            ),
        ))
    }

    /// Borrow the owned staging directory through artifact production and consumption.
    #[must_use]
    pub fn path(&self) -> &Path {
        self.directory.path()
    }

    /// Remove this owned staging child, reporting cleanup failure without a hidden retry.
    ///
    /// On failure, the caller owns recovery of the remaining staging child.
    /// It does not remove the parent or any other artifact directory.
    ///
    /// # Errors
    /// Returns the filesystem error if removing the owned child fails.
    pub fn close(mut self) -> io::Result<()> {
        self.directory.disarm();
        remove_path_if_present(self.directory.path())
    }
}

impl Drop for ArtifactStagingDirectory {
    fn drop(&mut self) {
        self.directory.cleanup_on_drop();
    }
}

/// Owned copy of one completed producer output, independent of later source replacement.
///
/// Keep this record alive through every use of [`Self::path`]. The copy is
/// temporary rather than a reusable cache entry; ordinary Drop removes it.
/// [`Self::retain`] explicitly transfers cleanup to the caller.
#[derive(Debug)]
pub struct CapturedArtifact {
    staging: ArtifactStagingDirectory,
    path: PathBuf,
}

impl CapturedArtifact {
    /// Capture an output synchronously before another producer can overwrite it.
    ///
    /// The caller keeps the source stable during capture and selects the maximum
    /// readable byte count. Read/publication failures retain their typed causes
    /// inside the returned I/O error. Existing public destinations are untouched.
    ///
    /// # Errors
    /// Returns the source read/byte-limit, staging allocation or durable-write
    /// error. An unsuccessful capture performs best-effort staging cleanup.
    pub fn capture(source: &Path, parent: &Path, maximum_bytes: usize) -> io::Result<Self> {
        let bytes = read_file(source, maximum_bytes).map_err(io::Error::from)?;
        let staging = ArtifactStagingDirectory::new(parent)?;
        let path = staging.path().join("input.artifact");
        write_bytes(&path, &bytes).map_err(io::Error::other)?;
        Ok(Self { staging, path })
    }

    /// Borrow the exact captured bytes while this record owns their lifetime.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Transfer the captured file and its staging child to caller-owned retention.
    ///
    /// Unlike close or Drop, this keeps the exact captured bytes after this owner
    /// is consumed. The caller owns later cleanup of the returned path's parent.
    /// Keep custody of those paths; this grants no cache or release admission.
    #[must_use]
    pub fn retain(mut self) -> PathBuf {
        self.staging.directory.disarm();
        self.path
    }

    /// Remove the captured output and its owned staging child, reporting cleanup failure.
    ///
    /// # Errors
    /// Returns the filesystem error if removing the owned child fails.
    pub fn close(self) -> io::Result<()> {
        self.staging.close()
    }
}

#[cfg(test)]
mod tests;
