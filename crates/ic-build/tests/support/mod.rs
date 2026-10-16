// Each integration target uses only the fixtures it needs.
#![allow(dead_code)]

#[path = "../../test-support/fs/mod.rs"]
pub mod fs_fixture;

#[path = "../../test-support/cargo/mod.rs"]
pub mod cargo;

#[path = "../../test-support/cargo/wasm.rs"]
pub mod wasm;

use std::path::{Path, PathBuf};

pub fn discover_workspace(manifest_dir: impl AsRef<Path>) -> std::io::Result<PathBuf> {
    use ic_host_process::tool::{OutputLimit, OutputLimits};

    ic_build::cargo::observation::workspace_root_for(
        manifest_dir,
        OutputLimits {
            stdout: OutputLimit::Terminate(64 * 1024),
            stderr: OutputLimit::Terminate(64 * 1024),
            timeout: None,
        },
        std::process::Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into())),
    )
}

pub fn wasm_path(target_dir: &Path, crate_name: &str, profile_target_dir: &str) -> PathBuf {
    target_dir
        .join("wasm32-unknown-unknown")
        .join(profile_target_dir)
        .join(format!("{crate_name}.wasm"))
}
