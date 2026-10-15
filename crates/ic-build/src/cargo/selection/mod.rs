//! One interpretation of recipe arguments, retaining their original identity bytes.

use std::{
    collections::BTreeSet,
    ffi::{OsStr, OsString},
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SelectionFlag {
    NonBuilding,
    TargetDirectory,
    InputSelection,
    TargetSelection,
    Library,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CargoSelection {
    pub(crate) arguments: Vec<OsString>,
    pub(crate) metadata_arguments: Vec<OsString>,
    pub(crate) profile: Result<String, &'static str>,
    pub(crate) flags: BTreeSet<SelectionFlag>,
}

impl CargoSelection {
    pub(crate) fn parse(arguments: Vec<OsString>) -> Self {
        let mut metadata_arguments = Vec::new();
        let mut flags = BTreeSet::new();
        let mut profile = Ok(None);
        let mut metadata_next = 0;
        let mut profile_next = 0;
        for (index, argument) in arguments.iter().enumerate() {
            let bytes = argument.as_encoded_bytes();
            let short = short_cargo_option(argument);
            if let Some(flag) = selection_flag(argument, short) {
                flags.insert(flag);
            }

            if index >= profile_next && profile.is_ok() {
                let selected = if argument == "--profile" {
                    profile_next = index + 2;
                    arguments
                        .get(index + 1)
                        .and_then(|value| value.to_str())
                        .map(Some)
                        .ok_or("Cargo --profile requires a UTF-8 profile name")
                } else if let Some(name) = argument
                    .to_str()
                    .and_then(|text| text.strip_prefix("--profile="))
                {
                    Ok(Some(name))
                } else {
                    let flags_end = short.map_or(bytes.len(), |(_, offset)| offset);
                    let release = argument == "--release"
                        || (bytes.starts_with(b"-")
                            && !bytes.starts_with(b"--")
                            && bytes[..flags_end].contains(&b'r'));
                    if matches!(short, Some((_, offset)) if offset + 1 == bytes.len()) {
                        profile_next = index + 2;
                    }
                    Ok(release.then_some("release"))
                };
                match selected {
                    Err(error) => profile = Err(error),
                    Ok(Some(name)) => {
                        if name.is_empty() || profile.as_mut().unwrap().replace(name).is_some() {
                            profile = Err("select one nonempty Cargo profile");
                        }
                    }
                    Ok(None) => {}
                }
            }

            if index < metadata_next {
                continue;
            }
            if let Some((b'F', offset)) = short {
                metadata_arguments.push(argument.to_str().map_or_else(
                    || argument.clone(),
                    |text| OsString::from(format!("-F{}", &text[offset + 1..])),
                ));
                if offset + 1 == bytes.len()
                    && let Some(value) = arguments.get(index + 1)
                {
                    metadata_arguments.push(value.clone());
                    metadata_next = index + 2;
                }
                continue;
            }
            let text = argument.to_string_lossy();
            match text.as_ref() {
                "--all-features"
                | "--no-default-features"
                | "--locked"
                | "--offline"
                | "--frozen" => metadata_arguments.push(argument.clone()),
                "--features" | "--filter-platform" => {
                    metadata_arguments.push(argument.clone());
                    if let Some(value) = arguments.get(index + 1) {
                        metadata_arguments.push(value.clone());
                        metadata_next = index + 2;
                    }
                }
                _ if text.starts_with("--features=") || text.starts_with("--filter-platform=") => {
                    metadata_arguments.push(argument.clone());
                }
                _ => {}
            }
        }
        let profile = profile.map(|name| name.unwrap_or("dev").to_owned());
        Self {
            arguments,
            metadata_arguments,
            profile,
            flags,
        }
    }
}

fn selection_flag(argument: &OsStr, short: Option<(u8, usize)>) -> Option<SelectionFlag> {
    let bytes = argument.as_encoded_bytes();
    let option = bytes.split(|byte| *byte == b'=').next().unwrap_or_default();
    // Refusals historically inspect every raw token, including option
    // values. Keep that conservative admission independent of projection.
    if matches!(argument.to_str(), Some("--help" | "--unit-graph"))
        || matches!(short, Some((b'h', _)))
    {
        return Some(SelectionFlag::NonBuilding);
    }
    if argument == "--target-dir" || bytes.starts_with(b"--target-dir=") {
        return Some(SelectionFlag::TargetDirectory);
    }
    if matches!(
        option,
        b"--manifest-path"
            | b"--target"
            | b"--package"
            | b"--workspace"
            | b"--all"
            | b"--exclude"
            | b"--config"
    ) || matches!(short, Some((b'm' | b'p' | b'C', _)))
    {
        return Some(SelectionFlag::InputSelection);
    }
    if matches!(
        option,
        b"--bin"
            | b"--bins"
            | b"--example"
            | b"--examples"
            | b"--test"
            | b"--tests"
            | b"--bench"
            | b"--benches"
            | b"--all-targets"
    ) {
        return Some(SelectionFlag::TargetSelection);
    }
    if argument == "--lib" {
        return Some(SelectionFlag::Library);
    }

    None
}

// Return the first help or value-taking short option and its byte position.
// Bytes after a value-taking option are its value, not more switches.
// Unknown options remain Cargo's responsibility to reject.
fn short_cargo_option(argument: &OsStr) -> Option<(u8, usize)> {
    let bytes = argument.as_encoded_bytes();
    if !bytes.starts_with(b"-") || bytes.starts_with(b"--") {
        return None;
    }
    for (index, byte) in bytes.iter().copied().enumerate().skip(1) {
        match byte {
            b'v' | b'q' | b'r' => {}
            b'h' | b'F' | b'j' | b'Z' | b'm' | b'p' | b'C' => return Some((byte, index)),
            _ => return None,
        }
    }
    None
}

#[cfg(test)]
mod tests;
