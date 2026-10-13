//! Strict file-dependency discovery; callers supply directories and output policy.

use crate::artifacts::cache_fs::canonicalize_allow_missing;
use std::{
    collections::BTreeSet,
    fs, io,
    path::{Path, PathBuf},
};

/// Parse Cargo/rustc Make depfiles without interpreting unsupported Make syntax.
///
/// Supports escaped space/tab/backslash/hash/colon, `$$`, line continuations,
/// multiple rules and empty phony rules. Full-line comments are ignored; their
/// environment facts must be represented in caller identity. Relative paths
/// remain relative until discovery binds the caller's explicit working directory.
///
/// # Errors
/// Returns `InvalidData` for malformed or unsupported syntax, or an empty input set.
pub fn parse_cargo_depfile(text: &str) -> io::Result<Vec<PathBuf>> {
    let mut paths = BTreeSet::new();
    let mut targets = Vec::new();
    let mut dependencies = Vec::new();
    let mut token = String::new();
    let mut separated = false;
    let mut chars = text.chars();
    while let Some(character) = chars.next() {
        match character {
            '\\' => match chars.next() {
                Some('\n') => flush_token(&mut token, separated, &mut targets, &mut dependencies),
                Some('\r') if chars.next() == Some('\n') => {
                    flush_token(&mut token, separated, &mut targets, &mut dependencies);
                }
                Some(escaped @ (' ' | '\t' | '\\' | '#' | ':')) => token.push(escaped),
                _ => return Err(invalid("unsupported depfile escape")),
            },
            '$' => {
                if chars.next() != Some('$') {
                    return Err(invalid("unsupported depfile expansion"));
                }
                token.push('$');
            }
            ':' => {
                if separated {
                    return Err(invalid("unescaped colon in depfile dependency"));
                }
                flush_token(&mut token, false, &mut targets, &mut dependencies);
                if targets.is_empty() {
                    return Err(invalid("depfile target is empty"));
                }
                separated = true;
            }
            '#' if !separated && targets.is_empty() && token.is_empty() => {
                for comment in chars.by_ref() {
                    if comment == '\n' {
                        break;
                    }
                }
            }
            '\n' => {
                finish_rule(
                    &mut token,
                    &mut targets,
                    &mut dependencies,
                    &mut separated,
                    &mut paths,
                )?;
            }
            ' ' | '\t' | '\r' => {
                flush_token(&mut token, separated, &mut targets, &mut dependencies);
            }
            '#' | ';' | '|' | '*' | '?' | '[' | ']' | '=' | '%' => {
                return Err(invalid("unsupported Make depfile syntax"));
            }
            control if control.is_control() => return Err(invalid("control byte in depfile")),
            ordinary => token.push(ordinary),
        }
    }
    finish_rule(
        &mut token,
        &mut targets,
        &mut dependencies,
        &mut separated,
        &mut paths,
    )?;
    if paths.is_empty() {
        return Err(invalid("depfile has no source dependencies"));
    }
    Ok(paths.into_iter().collect())
}

/// Discover source inputs from `.d` records directly in caller-selected directories.
///
/// Missing directories are an empty cold-build inventory. Records use bounded,
/// regular-file/no-follow reads; any malformed record refuses the entire result.
/// Existing inputs resolve aliases before generated-root exclusion. Missing paths
/// with parent traversal are never classified as generated merely by prefix.
/// This reads evidence only: no Cargo invocation, cache, cleanup or deadline.
///
/// # Errors
/// Returns non-absolute selections, record overflow, unsupported syntax or I/O.
pub fn discover_cargo_depfile_inputs(
    current_dir: &Path,
    directories: &[PathBuf],
    excluded_roots: &[PathBuf],
    maximum_record_bytes: usize,
) -> io::Result<Vec<PathBuf>> {
    if !current_dir.is_absolute()
        || directories
            .iter()
            .chain(excluded_roots)
            .any(|path| !path.is_absolute())
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "depfile selections must be absolute",
        ));
    }
    let excluded = excluded_roots
        .iter()
        .map(|path| canonicalize_allow_missing(path))
        .collect::<io::Result<Vec<_>>>()?;
    let mut inputs = BTreeSet::new();
    for directory in directories {
        let entries = match fs::read_dir(directory) {
            Ok(entries) => entries,
            Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
            Err(error) => return Err(error),
        };
        for entry in entries {
            let record = entry?.path();
            if record.extension().is_none_or(|extension| extension != "d") {
                continue;
            }
            let bytes = ic_host_fs::read::read_file_no_follow(&record, maximum_record_bytes)
                .map_err(io::Error::from)?;
            let text = std::str::from_utf8(&bytes)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
            for path in parse_cargo_depfile(text)? {
                let path = if path.is_absolute() {
                    path
                } else {
                    current_dir.join(path)
                };
                let path = match path.canonicalize() {
                    Ok(path) => path,
                    Err(error) if error.kind() == io::ErrorKind::NotFound => path,
                    Err(error) => return Err(error),
                };
                if !path
                    .components()
                    .any(|component| component == std::path::Component::ParentDir)
                    && excluded.iter().any(|root| path.starts_with(root))
                {
                    continue;
                }
                inputs.insert(path);
            }
        }
    }
    Ok(inputs.into_iter().collect())
}

fn flush_token(
    token: &mut String,
    separated: bool,
    targets: &mut Vec<String>,
    dependencies: &mut Vec<String>,
) {
    if !token.is_empty() {
        if separated {
            dependencies.push(std::mem::take(token));
        } else {
            targets.push(std::mem::take(token));
        }
    }
}

fn finish_rule(
    token: &mut String,
    targets: &mut Vec<String>,
    dependencies: &mut Vec<String>,
    separated: &mut bool,
    paths: &mut BTreeSet<PathBuf>,
) -> io::Result<()> {
    flush_token(token, *separated, targets, dependencies);
    if !targets.is_empty() && !*separated {
        return Err(invalid("depfile rule has no target separator"));
    }
    paths.extend(dependencies.drain(..).map(PathBuf::from));
    targets.clear();
    *separated = false;
    Ok(())
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
