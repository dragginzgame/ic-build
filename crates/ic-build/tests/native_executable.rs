//! Real native Cargo selection, environment preservation and failed-build evidence.
#![cfg(unix)]

mod support;

use ic_build::cargo::native::{NativeExecutableSpec, build_native_executable};
use ic_host_process::tool::ToolError;
use std::{cell::Cell, ffi::OsString, fs, process::Command};

#[test]
fn actual_cargo_reports_custom_profile_executable_and_streams_failed_build_diagnostics() {
    let root = support::unique_temp_directory("native-executable");
    fs::create_dir(root.join("examples")).unwrap();
    fs::write(root.join("Cargo.toml"), "[package]\nname = \"native-fixture\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[workspace]\n[profile.custom]\ninherits = \"dev\"\n").unwrap();
    fs::write(
        root.join("examples/worker.rs"),
        "fn unused_fixture() {}\nfn main() { println!(\"{}\", env!(\"NATIVE_FIXTURE_VALUE\")); }\n",
    )
    .unwrap();
    let cargo_program = std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    assert!(
        Command::new(&cargo_program)
            .current_dir(&root)
            .args(["generate-lockfile", "--offline"])
            .status()
            .unwrap()
            .success()
    );
    let target = root.join("output space");
    let calls = Cell::new(0);
    let factory = || {
        calls.set(calls.get() + 1);
        let mut command = Command::new(&cargo_program);
        command
            .current_dir(&root)
            .env("CARGO_TARGET_DIR", &target)
            .env("CARGO_NET_OFFLINE", "true")
            .env("NATIVE_FIXTURE_VALUE", "factory-preserved");
        command
    };
    let args = [OsString::from("--profile"), OsString::from("custom")];
    let spec = NativeExecutableSpec {
        package: "native-fixture",
        target_name: "worker",
        target_kind: "example",
        build_args: &args,
        max_message_bytes: 1024 * 1024,
    };
    let mut diagnostics = Vec::new();
    let built = build_native_executable(&spec, factory, &mut diagnostics, || Ok(())).unwrap();
    assert_eq!(calls.get(), 2);
    assert_eq!(built.target_directory, target);
    assert!(built.executable.is_file());
    assert!(built.executable.starts_with(&target));
    assert!(String::from_utf8_lossy(&diagnostics).contains("never used"));
    assert_eq!(
        Command::new(&built.executable).output().unwrap().stdout,
        b"factory-preserved\n"
    );
    calls.set(0);
    let admissions = Cell::new(0);
    let error = build_native_executable(&spec, factory, &mut diagnostics, || {
        admissions.set(admissions.get() + 1);
        if admissions.get() == 2 {
            Err(std::io::ErrorKind::Interrupted.into())
        } else {
            Ok(())
        }
    })
    .unwrap_err();
    assert_eq!(
        error.downcast_ref::<std::io::Error>().unwrap().kind(),
        std::io::ErrorKind::Interrupted
    );
    assert_eq!(calls.get(), 1);
    fs::write(
        root.join("examples/worker.rs"),
        "fn main() { this_does_not_compile(); }\n",
    )
    .unwrap();
    diagnostics.clear();
    let error = build_native_executable(&spec, factory, &mut diagnostics, || Ok(())).unwrap_err();
    let execution = error
        .downcast_ref::<ToolError>()
        .expect("original Host failure stays typed");
    assert_eq!(
        execution.evidence().unwrap().status.unwrap().code(),
        Some(101)
    );
    assert!(String::from_utf8_lossy(&diagnostics).contains("this_does_not_compile"));
}
