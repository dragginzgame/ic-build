use super::{ArtifactStagingDirectory, CapturedArtifact};
use crate::artifacts::test_support::unique_temp_directory;
use std::{fs, io};

// Moved from Canic's capture tests; producer paths and public publication remain distinct.
#[test]
fn captured_cargo_input_survives_replacement_and_cleans_up() {
    let root = unique_temp_directory("captured-cargo-output");
    let source = root.join("cargo.wasm");
    let destination = root.join("published.wasm");
    fs::write(&source, b"compiled first role").unwrap();
    fs::write(&destination, b"previous artifact").unwrap();
    let captured = CapturedArtifact::capture(&source, &root, 1024).unwrap();
    let staging = captured.path().parent().unwrap().to_owned();

    fs::write(&source, b"later compilation").unwrap();
    assert_eq!(fs::read(captured.path()).unwrap(), b"compiled first role");
    assert_eq!(fs::read(&destination).unwrap(), b"previous artifact");
    fs::remove_file(&source).unwrap();
    assert_eq!(fs::read(captured.path()).unwrap(), b"compiled first role");

    drop(captured);
    assert!(!staging.exists());
    assert_eq!(fs::read(&destination).unwrap(), b"previous artifact");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn failed_cargo_input_capture_leaves_no_stage_or_published_changes() {
    let root = unique_temp_directory("failed-cargo-capture");
    let destination = root.join("published.wasm");
    fs::write(&destination, b"previous artifact").unwrap();
    let error = CapturedArtifact::capture(&root.join("missing.wasm"), &root, 1024).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
    assert_eq!(fs::read(&destination).unwrap(), b"previous artifact");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn capture_honors_the_selected_byte_budget_and_preserves_its_typed_error() {
    let root = unique_temp_directory("bounded-capture");
    let source = root.join("input");
    fs::write(&source, b"four").unwrap();
    let error = CapturedArtifact::capture(&source, &root, 3).unwrap_err();
    assert!(matches!(
        error
            .get_ref()
            .unwrap()
            .downcast_ref::<ic_host_artifacts::artifact::ArtifactError>(),
        Some(ic_host_artifacts::artifact::ArtifactError::LimitExceeded { limit: 3 })
    ));
    assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
    let captured = CapturedArtifact::capture(&source, &root, 4).unwrap();
    assert_eq!(fs::read(captured.path()).unwrap(), b"four");
    let staging = captured.path().parent().unwrap().to_owned();
    captured.close().unwrap();
    assert!(!staging.exists());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn simultaneous_captures_retain_distinct_versions_without_blocking() {
    let root = unique_temp_directory("distinct-captures");
    let source = root.join("input");
    fs::write(&source, b"first").unwrap();
    let first = CapturedArtifact::capture(&source, &root, 1024).unwrap();
    let duplicate = CapturedArtifact::capture(&source, &root, 1024).unwrap();
    fs::write(&source, b"second").unwrap();
    let second = CapturedArtifact::capture(&source, &root, 1024).unwrap();
    assert_ne!(first.path(), duplicate.path());
    assert_eq!(fs::read(first.path()).unwrap(), b"first");
    assert_eq!(fs::read(second.path()).unwrap(), b"second");
    first.close().unwrap();
    assert_eq!(fs::read(duplicate.path()).unwrap(), b"first");
    duplicate.close().unwrap();
    second.close().unwrap();
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn staging_preserves_unowned_children_and_requires_an_existing_parent() {
    let root = unique_temp_directory("staging-custody");
    let foreign = root.join("foreign");
    fs::create_dir(&foreign).unwrap();
    fs::write(foreign.join("evidence"), b"preserve").unwrap();
    let missing = root.join("missing");
    assert_eq!(
        ArtifactStagingDirectory::new(&missing).unwrap_err().kind(),
        io::ErrorKind::NotFound
    );
    assert!(!missing.exists());
    let staging = ArtifactStagingDirectory::new(&root).unwrap();
    fs::write(staging.path().join("candidate"), b"owned").unwrap();
    let staged_path = staging.path().to_owned();
    staging.close().unwrap();
    assert!(!staged_path.exists());
    assert_eq!(fs::read(foreign.join("evidence")).unwrap(), b"preserve");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn retained_capture_survives_owner_and_source_without_retaining_other_captures() {
    let root = unique_temp_directory("retained-capture");
    let source = root.join("producer.wasm");
    fs::write(&source, b"exact evidence").unwrap();
    let retained = CapturedArtifact::capture(&source, &root, 32).unwrap();
    let temporary = CapturedArtifact::capture(&source, &root, 32).unwrap();
    let temporary_path = temporary.path().to_owned();
    let path = retained.retain();
    fs::remove_file(&source).unwrap();
    drop(temporary);
    assert!(!temporary_path.exists());
    assert_eq!(fs::read(&path).unwrap(), b"exact evidence");
    fs::remove_dir_all(path.parent().unwrap()).unwrap();
    assert!(!path.exists());
    fs::remove_dir_all(root).unwrap();
}
