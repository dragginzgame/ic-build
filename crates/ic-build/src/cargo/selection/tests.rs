use super::*;

#[test]
fn clustered_features_keep_build_bytes_and_metadata_resolution_in_step() {
    let arguments = [
        "-qrFextra",
        "--features",
        "alpha,beta",
        "--offline",
        "--lib",
    ]
    .map(OsString::from)
    .to_vec();
    let selected = CargoSelection::parse(arguments.clone());
    assert_eq!(selected.arguments, arguments);
    assert_eq!(selected.profile, Ok("release".to_owned()));
    assert_eq!(
        selected.metadata_arguments,
        ["-Fextra", "--features", "alpha,beta", "--offline"].map(OsString::from)
    );
    assert_eq!(selected.flags, BTreeSet::from([SelectionFlag::Library]));
}

#[test]
fn malformed_and_option_shaped_values_keep_conservative_admission() {
    let selected = CargoSelection::parse(
        ["--features", "--profile=fast"]
            .map(OsString::from)
            .to_vec(),
    );
    assert_eq!(selected.profile, Ok("fast".to_owned()));
    assert_eq!(selected.metadata_arguments, selected.arguments);

    let selected = CargoSelection::parse(
        ["--features", "--help", "-F", "--target-dir", "--profile"]
            .map(OsString::from)
            .to_vec(),
    );
    assert_eq!(
        selected.profile,
        Err("Cargo --profile requires a UTF-8 profile name")
    );
    assert_eq!(
        selected.flags,
        BTreeSet::from([SelectionFlag::NonBuilding, SelectionFlag::TargetDirectory])
    );
    assert_eq!(
        selected.metadata_arguments,
        ["--features", "--help", "-F", "--target-dir"].map(OsString::from)
    );
}

#[cfg(unix)]
#[test]
fn malformed_native_feature_bytes_reach_cargo_unchanged() {
    use std::os::unix::ffi::OsStringExt as _;
    for argument in [b"-qF\xff".as_slice(), b"--features=\xff".as_slice()] {
        let raw = OsString::from_vec(argument.to_vec());
        let selected = CargoSelection::parse(vec![raw.clone()]);
        assert_eq!(selected.arguments, std::slice::from_ref(&raw));
        assert_eq!(selected.metadata_arguments, [raw]);
        assert_eq!(selected.profile, Ok("dev".to_owned()));
    }
}
