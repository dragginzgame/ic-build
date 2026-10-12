# Current handoff

## Planning bootstrap

The maintainer authorized creation of the local `ic-build` repository and saving
[the build ownership audit](../design/build-ownership.md) here for coordination.
The repository contains the proposed ownership split, findings, extraction order,
source evidence and required safety boundaries. No production implementation has
been copied, extracted or retired.

Shared Tooling 0.9.1 is canonically adopted at
`e8a9ea6b660f216166eb00d0af6a32462bdbda4a`. Source was an isolated clean checkout;
dirty sibling Shared changes were not adopted. The Make entrypoint provides common
explicit setup and release commands; release metadata admission refuses while
this repository has no package or release adapter.

`make check` (also `make ci`) is the complete bootstrap gate: snapshot integrity,
offline common tools, all Markdown links, workspace metadata, declared
Cargo/workflow/action pins, manifest formatting and isolated release routing.
Linux delivery also runs `make lint check-secrets` with explicitly prepared
validation tools. Shared helpers remain unchanged in the recorded snapshot.
The virtual root Cargo workspace reserves version `0.1.0`, edition 2024 and MSRV
target 1.88.0 at the maintainer's request. It has no members, dependency graph or
executable yet. There is no build-engine or native macOS qualification. Add the
real `crates/ic-build/` package and meaningful package gates when extraction
implementation is explicitly started; do not manufacture a placeholder crate,
successful runtime test result or release identity for the planning stage.

The complete common toolset (four host, five IC and three Cargo executables) was
prepared with `make install-tools` and verified offline with `make tools-check`
on Linux x86-64 on 2026-10-12. Setup reused verified shared-store bundles and used
the existing Rust/Cargo 1.99.0 toolchain. No compiler upgrade was performed. This
tool setup does not qualify the planned package's MSRV or native macOS support.
The existing development compiler is now explicitly selected in
`rust-toolchain.toml`; its exact tool selections were prepared again after that
declaration. Earlier failed validation logs remain under `target/validation-failures/`.

The Makefile exposes the shared formatting and hook targets, workspace version,
maintenance catalog and pinned Linux validation-tool setup. Formatting covers
the empty workspace's manifest only. The bootstrap pin adapter delegates
declaration rules to the shared jq module and refuses a populated workspace;
extraction must replace it with the full shared dependency checker and a tracked
lockfile, add rustfmt to both formatter commands, and add behavioral/MSRV gates.
The shared LOC command requires packages and is not a bootstrap gate.

Linux validation passed with `make ci` and `make lint check-secrets`. The shared
hook is activated through local `core.hooksPath=.githooks`. An isolated
manifest-only adoption check verified sorting and selected-file refresh,
partial-staging rejection, preservation of dirty/untracked files and refusal
to overwrite another hook configuration. The real checkout's index was
unchanged. Release routing was checked with the shared substitute runner;
no release operation was executed.

CI is configured for Linux x86-64, macOS 15 Apple Silicon and macOS 15 Intel,
with Linux shell/workflow lint and history secret scanning. It cancels older
runs for the same workflow/ref and retains failed native validation/installer
evidence through the shared action. Configuration is not a passing hosted run;
native macOS execution remains unqualified.

The public [GitHub repository](https://github.com/dragginzgame/ic-build) is
established as `origin`. The maintainer explicitly authorized committing and
pushing the validated bootstrap to `main`. GitHub commits and workflow runs
provide delivery and native-CI evidence. The workspace version remains the
reserved initial `0.1.0`; no package release or registry publication is
established. The next implementation work is governed by the design; sibling
changes retain their own target authorization and coordination requirements.

GitHub issues remain the sole follow-up tracker. This handoff is setup evidence,
not an issue backlog. No sibling source or existing build artifacts were changed.
