//! Bounded Cargo observations and feature-safe package grouping.
//!
//! Responsibility: observe complete Cargo evidence and preserve isolated dependency trees.
//! Does not own: feature selection, role contracts, or runtime compilation.
//! Boundary: shared dependency feature unification must not change a role's isolated build.

#[cfg(test)]
mod tests;

pub mod environment;
pub mod flags;
pub mod observation;
pub mod packages;
pub(crate) mod selection;

use crate::batch::{BatchLabelError, validate_labels};
use ic_host_process::tool::OutputLimits;
use std::{collections::BTreeMap, path::Path, process::Command};

/// Partition packages without changing their resolved normal/build dependency trees.
///
/// The caller supplies the selected Cargo command, complete environment, target,
/// source custody and output/deadline policy. This does not publish artifacts or
/// establish a consumer's release validity.
/// Package names must be nonempty and unique; invalid inputs are rejected before
/// invoking the command factory, with zero-based indices in the error. An empty
/// package list returns an empty plan without invoking the factory.
///
/// # Errors
/// Returns invalid package input, the original typed Host process/capture failure
/// (including status and bounded diagnostics), or invalid dependency-tree evidence.
pub fn compatible_package_groups(
    workspace: &Path,
    packages: &[String],
    target: &str,
    limits: OutputLimits,
    mut cargo_command: impl FnMut() -> Command,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    validate_labels(packages.iter().map(String::as_str)).map_err(|error| match error {
        BatchLabelError::Empty { index } => {
            format!("empty Cargo package input at index {index}")
        }
        BatchLabelError::Duplicate { label, first_index, duplicate_index } => {
            format!("duplicate Cargo package input {label:?} at indices {first_index} and {duplicate_index}")
        }
    })?;
    let mut isolated = BTreeMap::new();
    let mut groups: Vec<Vec<String>> = Vec::new();
    for package in packages {
        let tree = resolved_trees(
            workspace,
            std::slice::from_ref(package),
            target,
            limits,
            &mut cargo_command,
        )?;
        if tree.len() != 1 {
            return Err("Cargo returned additional isolated package roots".into());
        }
        let selected = tree
            .get(package)
            .ok_or("Cargo omitted the selected package tree")?;
        isolated.insert(package.clone(), selected.clone());
        let mut selected_group = None;
        for (index, group) in groups.iter().enumerate() {
            let mut candidate = group.clone();
            candidate.push(package.clone());
            let combined =
                resolved_trees(workspace, &candidate, target, limits, &mut cargo_command)?;
            if preserves_individual_trees(&isolated, &combined, &candidate) {
                selected_group = Some(index);
                break;
            }
        }
        if let Some(index) = selected_group {
            groups[index].push(package.clone());
        } else {
            groups.push(vec![package.clone()]);
        }
    }
    Ok(groups)
}

fn resolved_trees(
    workspace: &Path,
    packages: &[String],
    target: &str,
    limits: OutputLimits,
    cargo_command: &mut impl FnMut() -> Command,
) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let manifest = workspace.join("Cargo.toml");
    let packages = packages.iter().map(String::as_str).collect::<Vec<_>>();
    let features = observation::CargoFeatureSelection::default();
    let tree = observation::observe_tree(
        &observation::CargoTreeSpec {
            manifest_path: &manifest,
            packages: &packages,
            target,
            edges: "normal,build",
            format: "{p}\t{f}",
            locked: true,
            offline: false,
            features: &features,
        },
        limits,
        cargo_command(),
    )?;
    parse_trees(&tree)
}

fn parse_trees(text: &str) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let mut trees = BTreeMap::new();
    let mut root = None;
    for line in text.lines().filter(|line| !line.is_empty()) {
        if let Some(package) = line.strip_prefix('0') {
            let name = package
                .split_whitespace()
                .next()
                .ok_or("empty Cargo tree root")?;
            if trees.insert(name.to_string(), String::new()).is_some() {
                return Err("duplicate Cargo tree root".into());
            }
            root = Some(name.to_string());
        }
        let tree = root
            .as_ref()
            .and_then(|name| trees.get_mut(name))
            .ok_or("Cargo dependency has no selected root")?;
        tree.push_str(line);
        tree.push('\n');
    }
    if trees.is_empty() {
        return Err("empty Cargo dependency forest".into());
    }
    Ok(trees)
}

fn preserves_individual_trees(
    isolated: &BTreeMap<String, String>,
    combined: &BTreeMap<String, String>,
    packages: &[String],
) -> bool {
    combined.len() == packages.len()
        && packages.iter().all(|name| {
            isolated
                .get(name)
                .is_some_and(|expected| combined.get(name) == Some(expected))
        })
}
