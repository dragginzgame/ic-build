use super::*;

const BEFORE: &str = r#"version = 4
[[package]]
name = "app"
version = "0.1.0"
dependencies = ["dependency"]
[[package]]
name = "app"
version = "9.0.0"
source = "registry+https://example.invalid"
checksum = "fixed"
[[package]]
name = "dependency"
version = "1.0.0"
source = "registry+https://example.invalid"
checksum = "dependency-checksum"
"#;
fn projection() -> WorkspaceVersionProjection<'static> {
    WorkspaceVersionProjection {
        package: "app",
        previous: "0.1.0",
        next: "0.1.1",
    }
}
fn after() -> String {
    BEFORE.replacen("0.1.0", "0.1.1", 1)
}

#[test]
fn projects_only_workspace_versions_and_accepts_comments() {
    let after = format!("# retained source comment\n{}", after());
    verify_workspace_version_projection(BEFORE, &after, &[projection()]).unwrap();
    assert!(matches!(
        verify_workspace_version_projection(BEFORE, BEFORE, &[projection()]),
        Err(LockProjectionError::ChangedInputs)
    ));
}

#[test]
fn dependency_source_checksum_and_other_fields_cannot_change() {
    for after in [
        after().replace("dependency-checksum", "changed"),
        after().replace("example.invalid", "different.invalid"),
        after().replace("9.0.0", "9.0.1"),
        after().replace("[\"dependency\"]", "[]"),
        after().replace("version = 4", "version = 3"),
    ] {
        assert!(matches!(
            verify_workspace_version_projection(BEFORE, &after, &[projection()]),
            Err(LockProjectionError::ChangedInputs)
        ));
    }
}

#[test]
fn missing_duplicate_and_wrong_prior_workspace_records_refuse() {
    assert!(matches!(
        verify_workspace_version_projection(
            BEFORE,
            &after(),
            &[WorkspaceVersionProjection {
                package: "missing",
                ..projection()
            }]
        ),
        Err(LockProjectionError::PackageSelection { matches: 0, .. })
    ));
    let duplicate = format!("{BEFORE}\n[[package]]\nname = \"app\"\nversion = \"0.1.0\"\n");
    assert!(matches!(
        verify_workspace_version_projection(&duplicate, &after(), &[projection()]),
        Err(LockProjectionError::PackageSelection { matches: 2, .. })
    ));
    assert!(matches!(
        verify_workspace_version_projection(
            BEFORE,
            &after(),
            &[WorkspaceVersionProjection {
                previous: "0.0.9",
                ..projection()
            }]
        ),
        Err(LockProjectionError::PreviousVersion { .. })
    ));
    assert!(matches!(
        verify_workspace_version_projection(BEFORE, &after(), &[]),
        Err(LockProjectionError::InvalidSelection)
    ));
    assert!(matches!(
        verify_workspace_version_projection(BEFORE, &after(), &[projection(), projection()]),
        Err(LockProjectionError::InvalidSelection)
    ));
    assert!(matches!(
        verify_workspace_version_projection("not valid [", &after(), &[projection()]),
        Err(LockProjectionError::Parse {
            input: "before",
            ..
        })
    ));
    assert!(matches!(
        verify_workspace_version_projection(BEFORE, "not valid [", &[projection()]),
        Err(LockProjectionError::Parse { input: "after", .. })
    ));
}

#[test]
fn projects_a_coherent_multi_package_family() {
    let before = "version=4\n[[package]]\nname=\"one\"\nversion=\"0.1.0\"\n[[package]]\nname=\"two\"\nversion=\"0.1.0\"\n";
    let changes = [
        WorkspaceVersionProjection {
            package: "one",
            ..projection()
        },
        WorkspaceVersionProjection {
            package: "two",
            ..projection()
        },
    ];
    verify_workspace_version_projection(before, &before.replace("0.1.0", "0.1.1"), &changes)
        .unwrap();
}
