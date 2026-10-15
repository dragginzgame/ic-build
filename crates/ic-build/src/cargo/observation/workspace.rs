use ic_host_process::tool::{ExecutionFailure, OutputLimits};
use std::{
    io,
    path::{Path, PathBuf},
    process::Command,
};

/// Ask Cargo for the workspace owning a crate manifest directory.
///
/// Cargo owns membership, explicit `package.workspace`, exclusions and nested
/// independent workspaces. This performs an offline `locate-project`, without
/// resolving dependencies or installing a toolchain. It never guesses from the
/// directory layout or falls back to the supplied directory.
/// The caller supplies the Cargo command, environment and capture limits, as
/// for other Cargo observations. Existing launcher arguments are preserved.
/// Output must be complete before parsing. No elapsed-time deadline is added;
/// toolchain auto-installation is disabled for this observation.
///
/// # Errors
/// Returns filesystem, Cargo invocation or invalid Cargo response errors.
/// Invocation failures retain typed Host status, output and cleanup evidence
/// as the I/O error's inner source; nonzero exits use `InvalidInput`.
pub fn workspace_root_for(
    crate_manifest_dir: impl AsRef<Path>,
    limits: OutputLimits,
    mut command: Command,
) -> io::Result<PathBuf> {
    let manifest = crate_manifest_dir
        .as_ref()
        .canonicalize()?
        .join("Cargo.toml");
    command
        .env("RUSTUP_AUTO_INSTALL", "0")
        .args([
            "--offline",
            "locate-project",
            "--workspace",
            "--message-format=json",
        ])
        .arg("--manifest-path")
        .arg(manifest);
    let output = super::capture(&mut command, limits).map_err(|error| {
        let kind = match error.execution_error().map(|execution| &execution.failure) {
            Some(ExecutionFailure::Io { source, .. }) => source.kind(),
            Some(ExecutionFailure::ExitStatus) => io::ErrorKind::InvalidInput,
            _ => io::ErrorKind::Other,
        };
        io::Error::new(kind, error)
    })?;
    let response: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let manifest = response
        .get("root")
        .and_then(serde_json::Value::as_str)
        .map(Path::new)
        .filter(|path| {
            path.is_absolute() && path.file_name() == Some(std::ffi::OsStr::new("Cargo.toml"))
        })
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Cargo returned no absolute workspace manifest",
            )
        })?;
    manifest.parent().map(Path::to_owned).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Cargo workspace manifest has no parent",
        )
    })
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use ic_host_process::tool::{OutputLimit, ToolError};
    use std::fs;

    #[test]
    fn discovery_refuses_oversized_complete_json_and_preserves_invocation_errors() {
        let root = crate::artifacts::test_support::unique_temp_directory("discovery-capture");
        let limits = OutputLimits {
            stdout: OutputLimit::Terminate(64 * 1024),
            stderr: OutputLimit::Terminate(64 * 1024),
            timeout: None,
        };
        let mut response = br#"{"root":"/tmp/Cargo.toml"}"#.to_vec();
        response.resize(64 * 1024, b' ');
        fs::write(root.join("response"), &response).unwrap();
        let mut command = Command::new("/bin/sh");
        command.current_dir(&root).env("SELECTED_RECIPE", "workspace discovery").args([
            "-c",
            "test \"$SELECTED_RECIPE\" = 'workspace discovery' && test \"$RUSTUP_AUTO_INSTALL\" = 0 && cat response",
        ]);
        assert_eq!(
            workspace_root_for(&root, limits, command).unwrap(),
            Path::new("/tmp")
        );
        response.push(b' ');
        fs::write(root.join("response"), &response).unwrap();
        let mut command = Command::new("/bin/sh");
        command.current_dir(&root).args(["-c", "cat response"]);
        let error = workspace_root_for(&root, limits, command).unwrap_err();
        assert!(
            error
                .get_ref()
                .unwrap()
                .downcast_ref::<ToolError>()
                .is_some()
        );
        let mut failed = Command::new("/bin/sh");
        failed.args(["-c", "printf refused >&2; exit 23"]);
        let error = workspace_root_for(&root, limits, failed).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        let host = error
            .get_ref()
            .unwrap()
            .downcast_ref::<ToolError>()
            .unwrap();
        let evidence = host.evidence().unwrap();
        assert_eq!(evidence.stderr, b"refused");
        assert_eq!(evidence.status.unwrap().code(), Some(23));
        let missing = Command::new(root.join("absent"));
        let error = workspace_root_for(&root, limits, missing).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        assert!(
            error
                .get_ref()
                .unwrap()
                .downcast_ref::<ToolError>()
                .is_some()
        );
        fs::remove_dir_all(root).unwrap();
    }
}
