use super::*;
use serde_json::json;

fn spec() -> NativeExecutableSpec<'static> {
    NativeExecutableSpec {
        package: "fixture",
        target_name: "worker",
        target_kind: "example",
        build_args: &[],
        max_message_bytes: 4096,
    }
}

fn metadata() -> schema::Metadata {
    serde_json::from_value(json!({
        "packages": [{
            "name": "fixture", "version": "0.0.0",
            "id": "path+file:///source#fixture@0.0.0",
            "dependencies": [], "features": {}, "manifest_path": "/source/Cargo.toml",
            "targets": [{"name": "worker", "kind": ["example"], "src_path": "/source/examples/worker.rs"}]
        }],
        "workspace_members": [], "workspace_root": "/source",
        "target_directory": "/selected-target", "version": 1
    }))
    .unwrap()
}

#[test]
fn metadata_selection_refuses_missing_ambiguous_and_relative_roots() {
    let spec = spec();
    let original = metadata();
    let (id, root) = selection(original.clone(), &spec).unwrap();
    assert_eq!(id, "path+file:///source#fixture@0.0.0");
    assert_eq!(root, PathBuf::from("/selected-target"));
    let mut ambiguous = original.clone();
    ambiguous.packages.push(original.packages[0].clone());
    assert!(selection(ambiguous, &spec).is_err());
    let mut missing = original.clone();
    missing.packages[0].targets.clear();
    assert!(selection(missing, &spec).is_err());
    let mut relative = original.clone();
    relative.target_directory = "relative-target".into();
    assert!(selection(relative, &spec).is_err());
    let mut wrong_kind = original;
    wrong_kind.packages[0].targets[0].kind = vec![schema::TargetKind::Lib];
    assert!(selection(wrong_kind, &spec).is_err());
    let mut missing_package = metadata();
    missing_package.packages.clear();
    assert!(selection(missing_package, &spec).is_err());
    let mut duplicate_target = metadata();
    let target = duplicate_target.packages[0].targets[0].clone();
    duplicate_target.packages[0].targets.push(target);
    assert!(selection(duplicate_target, &spec).is_err());
    let mut missing_identity = metadata();
    missing_identity.packages[0].id.repr.clear();
    assert_eq!(
        selection(missing_identity, &spec).unwrap_err().to_string(),
        "Cargo omitted package identity"
    );
}

#[test]
fn messages_keep_exact_package_target_diagnostics_and_ambiguity() {
    let spec = spec();
    let (id, _) = selection(metadata(), &spec).unwrap();
    let target = serde_json::to_value(&metadata().packages[0].targets[0]).unwrap();
    let artifact = json!({
        "reason": "compiler-artifact", "package_id": id, "target": target,
        "profile": {"opt_level": "0", "debug_assertions": true, "overflow_checks": true, "test": false},
        "features": [], "filenames": [], "fresh": false,
        "executable": "/custom-target/custom-profile/examples/worker"
    });
    let mut output = Vec::new();
    let mut selected = None;
    let mut unrelated = artifact.clone();
    unrelated["package_id"] = json!("path+file:///other#fixture@0.0.0");
    message(
        &serde_json::to_vec(&unrelated).unwrap(),
        &id,
        &spec,
        &mut selected,
        &mut output,
    )
    .unwrap();
    assert!(selected.is_none());
    let mut other_target = artifact.clone();
    other_target["target"]["name"] = json!("another-worker");
    message(
        &serde_json::to_vec(&other_target).unwrap(),
        &id,
        &spec,
        &mut selected,
        &mut output,
    )
    .unwrap();
    assert!(selected.is_none());
    let diagnostic = json!({
        "reason": "compiler-message", "package_id": id, "target": target,
        "message": {"message": "café", "level": "warning", "spans": [], "children": [], "rendered": "warning: café\n"}
    });
    message(
        &serde_json::to_vec(&diagnostic).unwrap(),
        &id,
        &spec,
        &mut selected,
        &mut output,
    )
    .unwrap();
    assert_eq!(output, "warning: café\n".as_bytes());
    let bytes = serde_json::to_vec(&artifact).unwrap();
    message(&bytes, &id, &spec, &mut selected, &mut output).unwrap();
    assert_eq!(
        selected,
        Some(PathBuf::from(
            "/custom-target/custom-profile/examples/worker"
        ))
    );
    assert!(message(&bytes, &id, &spec, &mut selected, &mut output).is_err());
    for invalid_path in [json!(null), json!("relative")] {
        let mut invalid = artifact.clone();
        invalid["executable"] = invalid_path;
        assert!(
            message(
                &serde_json::to_vec(&invalid).unwrap(),
                &id,
                &spec,
                &mut None,
                &mut output,
            )
            .is_err()
        );
    }
}

#[test]
fn message_allowance_and_protocol_refusals_preserve_boundaries() {
    let mut bytes = Vec::new();
    append(&mut bytes, b"123", 4).unwrap();
    append(&mut bytes, b"4", 4).unwrap();
    assert!(append(&mut bytes, b"5", 4).is_err());
    assert_eq!(bytes, b"1234");
    let spec = spec();
    for invalid in [b"not JSON".as_slice(), b"{}", br#"{"reason":"compiler-artifact","package_id":"selected","target":{"name":"worker","kind":["example"]},"executable":"relative"}"#] {
        assert!(message(invalid, "selected", &spec, &mut None, &mut Vec::new()).is_err());
    }
}

#[test]
fn invalid_selection_refuses_before_the_command_factory() {
    for invalid in [
        NativeExecutableSpec {
            target_kind: "lib",
            ..spec()
        },
        NativeExecutableSpec {
            package: "",
            ..spec()
        },
        NativeExecutableSpec {
            max_message_bytes: 0,
            ..spec()
        },
    ] {
        assert!(
            build_native_executable(
                &invalid,
                || panic!("invalid selection has no command effects"),
                Vec::new(),
                || Ok(())
            )
            .is_err()
        );
    }
    for arguments in [
        &["--target-dir", "elsewhere"][..],
        &["--manifest-path=elsewhere/Cargo.toml"],
        &["-pother"],
        &["--example=other"],
        &["--target=wasm32-unknown-unknown"],
        &["--config=elsewhere.toml"],
        &["--message-format=human"],
        &["--lib"],
        &["--help"],
        &["--profile", ""],
    ] {
        let arguments = arguments.iter().map(OsString::from).collect::<Vec<_>>();
        let invalid = NativeExecutableSpec {
            build_args: &arguments,
            ..spec()
        };
        assert!(
            build_native_executable(
                &invalid,
                || panic!("recipe overrides have no command effects"),
                Vec::new(),
                || Ok(()),
            )
            .is_err()
        );
    }
}

fn message(
    bytes: &[u8],
    package: &str,
    spec: &NativeExecutableSpec<'_>,
    executable: &mut Option<PathBuf>,
    diagnostics: &mut impl Write,
) -> io::Result<()> {
    selected_message(
        serde_json::from_slice(bytes).map_err(io::Error::other)?,
        package,
        spec,
        executable,
        diagnostics,
    )
}
