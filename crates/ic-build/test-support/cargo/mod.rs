//! Private Cargo fixture scaffolding shared by unit and integration targets.

use std::{fs, io, path::Path};

pub fn write_workspace(root: &Path, members: &[&str]) -> io::Result<()> {
    fs::create_dir_all(root)?;
    let members = toml::Value::Array(
        members
            .iter()
            .map(|member| toml::Value::String((*member).to_owned()))
            .collect(),
    );
    fs::write(
        root.join("Cargo.toml"),
        format!("[workspace]\nmembers = {members}\nresolver = \"2\"\n"),
    )
}

pub fn write_fixture_package(
    directory: &Path,
    manifest_tail: &str,
    source: &str,
) -> io::Result<()> {
    fs::create_dir_all(directory.join("src"))?;
    fs::write(
        directory.join("Cargo.toml"),
        format!(
            "[package]\nname = \"fixture\"\nversion = \"0.0.0\"\nedition = \"2024\"\n{manifest_tail}"
        ),
    )?;
    fs::write(directory.join("src/lib.rs"), source)
}
