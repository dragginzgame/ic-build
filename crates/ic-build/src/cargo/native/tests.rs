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

fn metadata() -> Value {
    json!({"packages": [{"name": "fixture", "id": "path+file:///source#fixture@0.0.0", "targets": [{"name": "worker", "kind": ["example"]}]}], "target_directory": "/selected-target"})
}

#[test]
fn metadata_selection_refuses_missing_ambiguous_and_relative_roots() {
    let spec = spec();
    let original = metadata();
    let (id, root) = selection(&original, &spec).unwrap();
    assert_eq!(id, "path+file:///source#fixture@0.0.0");
    assert_eq!(root, PathBuf::from("/selected-target"));
    let mut ambiguous = original.clone();
    ambiguous["packages"]
        .as_array_mut()
        .unwrap()
        .push(original["packages"][0].clone());
    assert!(selection(&ambiguous, &spec).is_err());
    let mut missing = original.clone();
    missing["packages"][0]["targets"] = json!([]);
    assert!(selection(&missing, &spec).is_err());
    let mut relative = original.clone();
    relative["target_directory"] = json!("relative-target");
    assert!(selection(&relative, &spec).is_err());
    let mut wrong_kind = original;
    wrong_kind["packages"][0]["targets"][0]["kind"] = json!(["lib"]);
    assert!(selection(&wrong_kind, &spec).is_err());
}

#[test]
fn messages_keep_exact_package_target_diagnostics_and_ambiguity() {
    let spec = spec();
    let (id, _) = selection(&metadata(), &spec).unwrap();
    let artifact = json!({"reason":"compiler-artifact", "package_id": id, "target":{"name":"worker", "kind":["example"]}, "executable":"/custom-target/custom-profile/examples/worker"});
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
    let diagnostic = json!({"reason":"compiler-message", "message":{"rendered":"warning: café\n"}});
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
                Vec::new()
            )
            .is_err()
        );
    }
}
