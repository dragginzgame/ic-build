//! Selected native executables from Cargo's own package and artifact messages.

use ic_host_process::{
    child::OwnedChild,
    tool::{
        OutputLimit, OutputLimits, OutputStream, SuccessfulExit, communicate_child_with_observer,
    },
};
use serde_json::Value;
use std::{
    error::Error,
    ffi::OsString,
    io::{self, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

#[cfg(test)]
mod tests;

/// Exact native target and caller-selected build arguments and decoding allowance.
pub struct NativeExecutableSpec<'a> {
    /// Unique package name in the selected Cargo metadata graph.
    pub package: &'a str,
    /// Cargo target name, without guessing an output filename.
    pub target_name: &'a str,
    /// `bin` or `example`, selecting Cargo's corresponding build target.
    pub target_kind: &'a str,
    /// Additional build arguments, such as profile, features and target selection.
    pub build_args: &'a [OsString],
    /// Maximum metadata bytes or bytes in one Cargo JSON message, including LF.
    /// Total streamed diagnostic output and build duration have no default limit.
    pub max_message_bytes: usize,
}

/// Cargo-reported paths; mutable build-directory custody remains caller-owned.
#[derive(Debug)]
pub struct NativeExecutable {
    /// Absolute executable reported for the exact selected package and target.
    pub executable: PathBuf,
    /// Absolute target root reported by the same Cargo command's metadata.
    pub target_directory: PathBuf,
}

/// Build a selected native target and return Cargo's reported executable.
///
/// The factory supplies the same Cargo prefix, working directory, environment
/// and network/tool policy for metadata and build. Commands add `--locked`, exact
/// package/target selection and Cargo's JSON diagnostics format. Stdout is a
/// bounded-per-message protocol; rendered compiler diagnostics and piped stderr
/// stream to `diagnostics`. Other stderr configurations remain intact. Stdin is
/// null. This owns direct-child completion, not process-group or signal policy.
/// `admit` runs before each command factory, so caller interruption can refuse
/// another dispatch after metadata. In-flight commands still run to completion.
///
/// Callers exclude source/config/tool mutation and retain artifact custody. No
/// cache, installation, release admission, retry or elapsed-time default is added.
/// Interrupted or rejected capture uses Host's direct-child cleanup evidence.
///
/// # Errors
/// Refuses invalid selection before command effects, missing/ambiguous metadata
/// or artifacts, malformed/oversized messages, sink failures and command failures.
/// Original Host execution/status/cleanup causes remain downcastable.
pub fn build_native_executable(
    spec: &NativeExecutableSpec<'_>,
    mut cargo: impl FnMut() -> Command,
    mut diagnostics: impl Write,
    mut admit: impl FnMut() -> io::Result<()>,
) -> Result<NativeExecutable, Box<dyn Error + Send + Sync>> {
    let selector = match spec.target_kind {
        "bin" => "--bin",
        "example" => "--example",
        _ => return Err(protocol("native target kind must be bin or example").into()),
    };
    if spec.package.is_empty() || spec.target_name.is_empty() || spec.max_message_bytes == 0 {
        return Err(protocol("empty native target selection or message allowance").into());
    }
    admit()?;
    let mut command = cargo();
    command.args(["metadata", "--locked", "--no-deps", "--format-version", "1"]);
    let mut bytes = Vec::new();
    execute(&mut command, |stream, chunk| match stream {
        OutputStream::Stdout => append(&mut bytes, chunk, spec.max_message_bytes),
        OutputStream::Stderr => diagnostics.write_all(chunk),
    })?;
    let (package_id, target_directory) = selection(&serde_json::from_slice(&bytes)?, spec)?;
    admit()?;
    let mut command = cargo();
    command.args([
        "build",
        "--locked",
        "--message-format=json",
        "-p",
        spec.package,
        selector,
        spec.target_name,
    ]);
    command.args(spec.build_args);
    let mut line = Vec::new();
    let mut executable = None;
    execute(&mut command, |stream, chunk| {
        if stream == OutputStream::Stderr {
            return diagnostics.write_all(chunk);
        }
        for fragment in chunk.split_inclusive(|byte| *byte == b'\n') {
            append(&mut line, fragment, spec.max_message_bytes)?;
            if line.ends_with(b"\n") {
                message(&line, &package_id, spec, &mut executable, &mut diagnostics)?;
                line.clear();
            }
        }
        Ok(())
    })?;
    if !line.is_empty() {
        message(&line, &package_id, spec, &mut executable, &mut diagnostics)?;
    }
    Ok(NativeExecutable {
        executable: executable.ok_or_else(|| protocol("Cargo omitted the selected executable"))?,
        target_directory,
    })
}

fn selection(metadata: &Value, spec: &NativeExecutableSpec<'_>) -> io::Result<(String, PathBuf)> {
    let packages = metadata["packages"]
        .as_array()
        .ok_or_else(|| protocol("Cargo omitted packages"))?;
    let mut matching = packages
        .iter()
        .filter(|package| package["name"] == spec.package);
    let package = matching
        .next()
        .ok_or_else(|| protocol("Cargo omitted selected package"))?;
    if matching.next().is_some() {
        return Err(protocol("Cargo package name is ambiguous"));
    }
    let targets = package["targets"]
        .as_array()
        .ok_or_else(|| protocol("Cargo omitted package targets"))?;
    if targets
        .iter()
        .filter(|target| matches_target(target, spec))
        .count()
        != 1
    {
        return Err(protocol("Cargo target is missing or ambiguous"));
    }
    let id = package["id"]
        .as_str()
        .filter(|id| !id.is_empty())
        .ok_or_else(|| protocol("Cargo omitted package identity"))?;
    let root = metadata["target_directory"]
        .as_str()
        .map(PathBuf::from)
        .filter(|root| root.is_absolute())
        .ok_or_else(|| protocol("Cargo omitted absolute target directory"))?;
    Ok((id.to_owned(), root))
}

fn matches_target(target: &Value, spec: &NativeExecutableSpec<'_>) -> bool {
    target["name"] == spec.target_name
        && target["kind"]
            .as_array()
            .is_some_and(|kinds| kinds.iter().any(|kind| kind == spec.target_kind))
}

fn message(
    bytes: &[u8],
    package_id: &str,
    spec: &NativeExecutableSpec<'_>,
    executable: &mut Option<PathBuf>,
    diagnostics: &mut impl Write,
) -> io::Result<()> {
    let message: Value = serde_json::from_slice(bytes).map_err(io::Error::other)?;
    match message["reason"]
        .as_str()
        .ok_or_else(|| protocol("invalid Cargo message reason"))?
    {
        "compiler-message" => {
            if let Some(rendered) = message["message"]["rendered"].as_str() {
                diagnostics.write_all(rendered.as_bytes())?;
            }
        }
        "compiler-artifact"
            if message["package_id"] == package_id && matches_target(&message["target"], spec) =>
        {
            let path = message["executable"]
                .as_str()
                .map(PathBuf::from)
                .filter(|path| path.is_absolute())
                .ok_or_else(|| protocol("Cargo omitted absolute executable path"))?;
            if executable.replace(path).is_some() {
                return Err(protocol("Cargo reported multiple selected executables"));
            }
        }
        _ => {}
    }
    Ok(())
}

fn execute(
    command: &mut Command,
    output: impl FnMut(OutputStream, &[u8]) -> io::Result<()>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    command.stdin(Stdio::null()).stdout(Stdio::piped());
    let mut child = OwnedChild::spawn_direct(command)?;
    communicate_child_with_observer(
        &mut child,
        None,
        OutputLimits {
            stdout: OutputLimit::Truncate(0),
            stderr: OutputLimit::Truncate(0),
            timeout: None,
        },
        SuccessfulExit::Cleanup,
        || false,
        output,
    )?
    .require_success()?;
    Ok(())
}

fn append(buffer: &mut Vec<u8>, bytes: &[u8], maximum: usize) -> io::Result<()> {
    if bytes.len() > maximum.saturating_sub(buffer.len()) {
        return Err(protocol("Cargo JSON message exceeds caller allowance"));
    }
    buffer.try_reserve(bytes.len()).map_err(io::Error::other)?;
    buffer.extend_from_slice(bytes);
    Ok(())
}

fn protocol(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
