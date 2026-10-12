# Current handoff

## Planning and package scaffold

The maintainer authorized creation of the local `ic-build` repository and saving
[the build ownership audit](../design/build-ownership.md) here for coordination.
The maintainer subsequently requested the standard `crates/` layout. The
repository now includes an unpublished, dependency-free `crates/ic-build/`
library scaffold alongside the ownership split, source evidence and safety
boundaries. No production implementation has been copied, extracted or retired.

Shared Tooling 0.9.1 is canonically adopted at
`e8a9ea6b660f216166eb00d0af6a32462bdbda4a`. Source was an isolated clean checkout;
dirty sibling Shared changes were not adopted. The Make entrypoint provides common
explicit setup and release commands; release admission refuses until engine
extraction and a reviewed release adapter exist. The package sets `publish = false`.

`make check` (also `make ci`) is the complete bootstrap gate: snapshot integrity,
offline common tools, all Markdown links, workspace metadata, the full shared
dependency/pin and inheritance checks, manifest/source formatting, scaffold
compilation and rustdoc, the declared MSRV and isolated release routing.
Linux delivery also runs `make lint check-secrets` with explicitly prepared
validation tools. Shared helpers remain unchanged in the recorded snapshot.
The virtual root Cargo workspace reserves version `0.1.0`, edition 2024 and MSRV
target 1.88.0 at the maintainer's request. It declares `crates/ic-build` as its
single member. The child inherits version, edition, Rust floor, repository,
description, README and lints; the root Cargo.lock records the dependency-free
graph. The library root documents the scaffold and exposes no public build API.
There is no build-engine or native macOS qualification. Extraction must add the
real implementation and behavioral tests and requalify the resulting dependency
graph; scaffold checks establish no runtime behavior or release identity.

The complete common toolset (four host, five IC and three Cargo executables) was
prepared with `make install-tools` and verified offline with `make tools-check`
on Linux x86-64 on 2026-10-12. Setup reused verified shared-store bundles and used
the existing Rust/Cargo 1.99.0 toolchain. No compiler upgrade was performed. This
tool setup does not qualify the planned package's MSRV or native macOS support.
The existing development compiler is now explicitly selected in
`rust-toolchain.toml`; its exact tool selections were prepared again after that
declaration. Earlier failed validation logs remain under `target/validation-failures/`.

The Makefile exposes the shared formatting and hook targets, workspace version,
maintenance catalog and pinned Linux validation-tool setup. Formatting now uses
the shared defaults: workspace manifest sorting followed by `cargo fmt --all`.
The empty-workspace pin adapter is retired in favor of the full shared checker
with `--cargo-inheritance`. `make cloc` reports the scaffold's zero implementation
and test LOC. The compilation, rustdoc and MSRV targets use the selected lockfile;
MSRV is read from root Cargo.toml rather than duplicated in CI.

The earlier manifest-only bootstrap passed Linux validation and isolated hook
preservation checks. The shared hook remains activated through local
`core.hooksPath=.githooks`. Release routing uses the shared substitute runner;
no release operation is part of this layout change.

The new Cargo.lock remains unstaged under the user-owned Git rules. The full
shared checker requires a tracked lockfile and refuses the working checkout
until it is staged. Layout preparation uses an isolated Git checkout with the
reviewed files tracked, preserving the real index. Linux scaffold compilation,
rustdoc and actual Rust/Cargo 1.88.0 checks passed in the working checkout;
Cargo also built and verified its local archive. The full `make ci` suite,
Linux lint/history scan and the shared source-formatting/hook preservation
check passed in the isolated checkout; its evidence is retained at
`/tmp/ic-build-crate-layout.ei2N1v/`. The real index was unchanged. Packaging currently warns that
no license has been selected, and registry publication remains disabled.

CI is configured for Linux x86-64, macOS 15 Apple Silicon and macOS 15 Intel,
with explicit preparation of the root-declared MSRV, Linux shell/workflow lint
and history secret scanning. It cancels older
runs for the same workflow/ref and retains failed native validation/installer
evidence through the shared action. Configuration is not a passing hosted run;
native macOS scaffold execution remains unqualified. The prior
[bootstrap run](https://github.com/dragginzgame/ic-build/actions/runs/38262914748)
is bound to the earlier empty-workspace commit, not this new package layout.

The public [GitHub repository](https://github.com/dragginzgame/ic-build) is
established as `origin`. Bootstrap commit `6e562d1` was pushed to `main` under
the maintainer's explicit authorization. The package-layout changes remain
working-tree edits. Version `0.1.0` remains the pending initial release because
there is no released consumer contract; no package release or registry
publication is established. The next implementation work is governed by the
design; sibling
changes retain their own target authorization and coordination requirements.

GitHub issues remain the sole follow-up tracker. This handoff is setup evidence,
not an issue backlog. No sibling source or existing build artifacts were changed.
