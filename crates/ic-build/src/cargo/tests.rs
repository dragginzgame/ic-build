use super::*;
use ic_host_process::tool::OutputLimit;

#[test]
#[cfg(unix)]
fn incomplete_tree_capture_and_spawn_failures_preserve_typed_host_errors() {
    let root = crate::artifacts::test_support::unique_temp_directory("batch-capture-failure");
    let packages = ["role".to_string()];
    let limits = OutputLimits {
        stdout: OutputLimit::Truncate(20),
        stderr: OutputLimit::Terminate(4096),
        timeout: None,
    };
    let truncated =
        compatible_package_groups(&root, &packages, "wasm32-unknown-unknown", limits, || {
            let mut command = Command::new("/bin/sh");
            command.args(["-c", "printf '0role v0.1.0\\t\\n1shared v0.1.0\\tbase\\n'"]);
            command
        })
        .unwrap_err();
    assert!(
        truncated
            .downcast_ref::<ic_host_process::tool::ToolError>()
            .is_some()
    );
    let missing =
        compatible_package_groups(&root, &packages, "wasm32-unknown-unknown", limits, || {
            Command::new(root.join("missing-cargo"))
        })
        .unwrap_err();
    assert!(
        missing
            .downcast_ref::<ic_host_process::tool::ToolError>()
            .is_some()
    );
    fs::remove_dir_all(root).unwrap();
}
use std::fs;

#[test]
fn real_cargo_resolution_batches_peers_and_splits_feature_growth() {
    let root = crate::artifacts::test_support::unique_temp_directory("build-feature-batches");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("Cargo.toml"), "[workspace]\nmembers=[\"shared\",\"exclusive\",\"left\",\"peer\",\"right\"]\nresolver=\"3\"\n").unwrap();
    for (name, extra) in [
        (
            "shared",
            "[features]\ndefault=[\"base\"]\nbase=[]\nextra=[]\n",
        ),
        ("exclusive", ""),
        ("left", "[dependencies]\nshared={path=\"../shared\"}\n"),
        (
            "peer",
            "[dependencies]\nshared={path=\"../shared\"}\nexclusive={path=\"../exclusive\"}\n",
        ),
        (
            "right",
            "[dependencies]\nshared={path=\"../shared\",features=[\"extra\"]}\n",
        ),
    ] {
        let package = root.join(name);
        fs::create_dir_all(package.join("src")).unwrap();
        fs::write(
            package.join("Cargo.toml"),
            format!("[package]\nname=\"{name}\"\nversion=\"0.1.0\"\nedition=\"2024\"\n{extra}"),
        )
        .unwrap();
        fs::write(package.join("src/lib.rs"), "pub fn fixture() {}\n").unwrap();
    }
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    assert!(
        Command::new(&cargo)
            .current_dir(&root)
            .args(["generate-lockfile", "--offline"])
            .status()
            .unwrap()
            .success()
    );
    let limits = OutputLimits {
        stdout: OutputLimit::Terminate(1024 * 1024),
        stderr: OutputLimit::Terminate(64 * 1024),
        timeout: None,
    };
    let packages = ["left", "peer", "right"].map(str::to_string);
    let groups =
        compatible_package_groups(&root, &packages, "wasm32-unknown-unknown", limits, || {
            let mut command = Command::new(&cargo);
            command
                .env("RUSTUP_AUTO_INSTALL", "0")
                .env("CARGO_NET_OFFLINE", "true");
            command
        })
        .unwrap();
    assert_eq!(groups, [vec!["left", "peer"], vec!["right"]]);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn shared_feature_growth_rejects_a_batch_but_disjoint_dependencies_do_not() {
    let left = "0left v1.0.0\t\n1shared v1.0.0\tbase\n";
    let right = "0right v1.0.0\t\n1shared v1.0.0\tbase\n1extra v1.0.0\tunique\n";
    let isolated = parse_trees(&format!("{left}\n{right}")).unwrap();
    let packages = ["left".to_string(), "right".to_string()];
    assert!(preserves_individual_trees(&isolated, &isolated, &packages));
    let changed = parse_trees(&format!(
        "{}\n{right}",
        left.replace("\tbase", "\tbase,extra")
    ))
    .unwrap();
    assert!(!preserves_individual_trees(&isolated, &changed, &packages));
    let omitted = parse_trees(left).unwrap();
    assert!(!preserves_individual_trees(&isolated, &omitted, &packages));
    assert!(parse_trees(&format!("{left}{left}")).is_err());
    assert!(parse_trees("1shared v1.0.0\tbase\n").is_err());
}

#[test]
fn feature_assignment_order_is_retained_for_host_and_wasm_instances() {
    let isolated =
        parse_trees("0role v1.0.0\t\n1shared v1.0.0\twasm\n1shared v1.0.0\thost\n").unwrap();
    let swapped =
        parse_trees("0role v1.0.0\t\n1shared v1.0.0\thost\n1shared v1.0.0\twasm\n").unwrap();
    assert!(!preserves_individual_trees(
        &isolated,
        &swapped,
        &["role".to_string()]
    ));
}
