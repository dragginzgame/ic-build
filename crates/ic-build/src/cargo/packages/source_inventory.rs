//! Qualification budgets and byte/mode projection; Host owns Unix tree admission.

use super::{Cause, PackageArchiveSpec, PackageQualificationLimits};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use ic_host_fs::read::tree::{TreeVisitor, walk_tree};
#[cfg(unix)]
use rustix::{
    fd::OwnedFd,
    fs::{FileType, Mode, OFlags, open},
};

pub(super) fn sources(
    root: &Path,
    packages: &[PackageArchiveSpec<'_>],
    limits: PackageQualificationLimits,
) -> Result<BTreeMap<PathBuf, Value>, Cause> {
    let mut files = BTreeMap::new();
    let mut remaining = limits.source_entries;
    for spec in packages {
        let path = root.join(format!("{}-{}", spec.name, spec.version));
        #[cfg(unix)]
        {
            let fd = open(
                &path,
                OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                Mode::empty(),
            )?;
            let mut visitor = SourceInventory {
                prefix: path.strip_prefix(root)?,
                limits,
                remaining: &mut remaining,
                files: &mut files,
            };
            visitor.enter_directory(Path::new(""), &path)?;
            walk_tree(&fd, Path::new(""), &path, &mut visitor)?;
        }
        #[cfg(not(unix))]
        inventory(root, &path, limits, 0, &mut remaining, &mut files)?;
    }
    Ok(files)
}

#[cfg(unix)]
struct SourceInventory<'a> {
    prefix: &'a Path,
    limits: PackageQualificationLimits,
    remaining: &'a mut usize,
    files: &'a mut BTreeMap<PathBuf, Value>,
}

#[cfg(unix)]
impl SourceInventory<'_> {
    fn admit(&mut self, relative: &Path) -> Result<(), Cause> {
        if relative.components().count() > self.limits.source_depth || *self.remaining == 0 {
            return Err("extracted source traversal budget exceeded".into());
        }
        *self.remaining -= 1;
        Ok(())
    }
}

#[cfg(unix)]
impl TreeVisitor for SourceInventory<'_> {
    type Error = Cause;

    fn file(&mut self, fd: &OwnedFd, relative: &Path, _display: &Path) -> Result<(), Cause> {
        use std::os::unix::fs::MetadataExt as _;
        self.admit(relative)?;
        let file = std::fs::File::from(fd.try_clone()?);
        let mode = file.metadata()?.mode() & 0o777;
        let sha256 = ic_host_artifacts::artifact::hash_reader(file, self.limits.file_bytes as u64)?
            .sha256
            .to_string();
        self.files.insert(
            self.prefix.join(relative),
            serde_json::json!({"sha256": sha256, "mode": mode}),
        );
        Ok(())
    }

    fn enter_directory(&mut self, relative: &Path, _display: &Path) -> Result<(), Cause> {
        self.admit(relative)
    }

    fn leave_directory(
        &mut self,
        _fd: &OwnedFd,
        _relative: &Path,
        _display: &Path,
    ) -> Result<(), Cause> {
        Ok(())
    }

    fn unsupported(&self, display: &Path, _kind: FileType) -> Cause {
        format!("unsupported extracted source entry: {}", display.display()).into()
    }

    fn non_utf8(&self, display: &Path) -> Cause {
        format!("non-UTF-8 extracted source entry: {}", display.display()).into()
    }
}

#[cfg(not(unix))]
fn inventory(
    root: &Path,
    path: &Path,
    limits: PackageQualificationLimits,
    depth: usize,
    remaining: &mut usize,
    files: &mut BTreeMap<PathBuf, Value>,
) -> Result<(), Cause> {
    if depth > limits.source_depth || *remaining == 0 {
        return Err("extracted source traversal budget exceeded".into());
    }
    *remaining -= 1;
    let metadata = std::fs::symlink_metadata(path)?;
    if metadata.is_dir() {
        for entry in std::fs::read_dir(path)? {
            inventory(root, &entry?.path(), limits, depth + 1, remaining, files)?;
        }
    } else if metadata.is_file() {
        let mode = metadata.permissions().readonly();
        files.insert(
            path.strip_prefix(root)?.to_owned(),
            serde_json::json!({"sha256": super::hash(path, limits.file_bytes)?.to_string(), "mode": mode}),
        );
    } else {
        return Err(format!("unsupported extracted source entry: {}", path.display()).into());
    }
    Ok(())
}
