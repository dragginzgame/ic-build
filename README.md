# IC Build

IC Build provides common Cargo/Wasm builds and retained artifact caches, moved
from IC Testkit, and feature-safe Cargo grouping from Canic.
The [published 0.12 family is available on crates.io](https://crates.io/crates/ic-build/0.12.0).
The owning
[extraction issue](https://github.com/dragginzgame/ic-build/issues/1) tracks
producer and consumer acceptance.

`ic_build::artifacts` owns exact Cargo input resolution, transactional artifact
publication, retained records, Wasm caches, sessions, batch results and explicit
maintenance. `ic_build::cargo` groups packages only when their resolved normal
and build dependency trees match isolated resolution. Consumers own recipes,
tools, output budgets, source custody, release identities and lifecycle.
PocketIC stays in Testkit; complete-release admission stays in Canic.

Build 0.12 moves native Candid extraction from Host Tools into Unix-only
`artifacts::candid::extract`, over the registry Host 0.24 family. It retains raw
bounded process evidence and source/tool identities; callers own tool admission,
limits, environment, normalization, grammar and publication. Before/after hashes
cannot detect transient writes: callers must retain their source lease. See
[#51](https://github.com/dragginzgame/ic-build/issues/51) and
[PR #52](https://github.com/dragginzgame/ic-build/pull/52).

Start with [Build ownership and extraction](docs/design/build-ownership.md).
It records the inspected build paths, duplicated responsibilities, intended
owners, retained safety boundaries and extraction sequence. Read the
[current handoff](docs/status/current.md) before implementation.
The [artifact API guide](docs/artifacts.md) covers recipes, retained records,
freshness, batching and explicit maintenance.
Pending 0.12.1 exposes `cargo::packages::verify_external_lock_subset` for bounded
in-memory admission of already selected external package identities
([API guide](docs/artifacts.md#external-lock-selection-admission)). It shares
archive qualification's reader/comparison without adopting its nonempty-graph
policy or executing Cargo; callers retain local package, feature and release checks.
Version 0.1.3 also provides temporary staging and bounded capture of producer
outputs, with caller-owned qualification and publication.

The dependency direction is application adapters → IC Build → IC Host
Tooling. IC Build must not depend on Canic, IcyDB, PocketIC or IC Testkit.

## Use

The examples below target pending 0.14. Once published, add the registry
dependency to your host-side package:

```toml
[dependencies]
ic-build = "0.14"
```

Rust 1.91.0 or newer is required. Import the engine directly and keep the returned
record alive while using its artifacts. With a prepared Cargo/Wasm toolchain and a
caller-held source lease:

```rust,no_run
use ic_build::artifacts::{WasmBuildSpec, build_wasm_canisters_cached};
use ic_build::cargo::selection::CargoSelection;
use std::path::Path;

fn build(workspace: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let target = workspace.join("target/canister-builds");
    let spec = WasmBuildSpec::new(workspace, &target, &["counter_canister"])
        .with_cargo_selection(CargoSelection {
            profile: Some("release".into()),
            locked: true,
            ..Default::default()
        });
    let outcome = build_wasm_canisters_cached(&spec)?;
    let wasm = std::fs::read(&outcome.record().artifacts()[0])?;
    Ok(wasm)
}
```

Consumers select packages, profiles, environment and maintenance policy.
IC Build 0.10 and newer select Wasm through Cargo's exact package/target messages,
including custom cdylib names. Existing recipes keep their cache paths and keys.
The engine owns `--message-format=json`; wrappers must report artifact records.
`with_cargo_message_bytes` adjusts the default 16 MiB per-message allowance.
Overflow refuses publication after Cargo completes while diagnostics keep flowing.
This adds no elapsed-time deadline. Metadata-derived declared paths remain
available for caller-owned external builds.
Pending 0.14 replaces raw Cargo arguments with `cargo::selection::CargoSelection`
and `with_cargo_selection`. Set `profile`, the existing
`cargo::selection::CargoFeatureSelection`, and explicit lock/network flags.
The feature selection owns `all_features`, explicit feature names and default-feature
policy; observations and builds share its encoder. Native executable
specifications use the same value in `selection`. Arbitrary CLI flags and target
selectors have no recipe escape hatch; choose packages, targets and output paths
through their structural fields. Cargo output events carry Host's
`ic_host_process::tool::OutputStream` directly. Progress and failure details use
`artifacts::WasmBuildPhase`, including separate Cargo/rustc identity phases.
Artifact batches return `batch::BatchLabelError`; Wasm contract errors retain it
in `WasmBuildBatchContractError::Labels`.
Published 0.6.1 adds `ic_build::cargo::environment` for read-only configuration
discovery, include traversal, raw-byte digests and compiler-override facts. Consumers select
effective or conservative input paths and retain compiler admission, receipt
framing and source custody. Configuration contents are excluded from returned
facts and failure diagnostics. Observation executes no Cargo commands and does
not establish build or release validity.

IC Build 0.12 uses the registry IC Host 0.24 family at the public grouping,
probe, capture and Candid boundary; release 0.12.0 selects Host 0.24.0. Align direct
Host dependencies before passing `OutputLimits`, `CleanupPolicy` or typed Host
failures across that boundary. Upgrading Build alone while retaining a different
Host minor family leaves nominally incompatible Rust types.
The [deployable-Wasm LTO guidance](docs/artifacts.md#deployable-wasm-lto) records
whole-canister measurements and shows how a common adapter can apply full LTO
through the existing API, with complete-application qualification before adopting
a shared release default.
The [0.12.0 release CI](https://github.com/dragginzgame/ic-build/actions/runs/38947150972)
passes Linux and native macOS 15 Intel/Apple Silicon. These producer results do
not establish consumer adoption or application qualification;
[#1](https://github.com/dragginzgame/ic-build/issues/1) tracks remaining acceptance
and Canic production convergence. See the artifact guide for source custody and
retained-cache contracts.

## Native executables

IC Build 0.10 and newer provide `cargo::native::build_native_executable` on Unix.
The example below uses the pending 0.14 typed recipe and caller-selected process policy.
It builds an explicit package's binary or example and returns Cargo's reported
executable and target root, without reconstructing paths for custom profiles or
target directories.
The caller supplies its Cargo command factory, profile/features, diagnostics writer
and per-message JSON allowance. Set shared manifest/configuration/target paths in
the factory so metadata and build use the same recipe. The specification owns
package/target selection; no raw build-argument field remains. Compiler diagnostics stream without a default deadline.

```rust,no_run
use ic_build::cargo::native::{NativeExecutableSpec, build_native_executable};
use ic_host_process::child::OwnedChild;
use std::process::Command;

let spec = NativeExecutableSpec {
    package: "my-package",
    target_name: "worker",
    target_kind: "example",
    selection: Default::default(),
    max_message_bytes: 16 * 1024 * 1024,
};
let built = build_native_executable(&spec, || {
    let mut cargo = Command::new("cargo");
    cargo.current_dir("/checkout");
    cargo
}, std::io::stderr(), || Ok(()), OwnedChild::spawn_direct, || false)?;
# Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
```

Admission runs before each dispatch. The caller selects child/group custody and
cleanup with the spawning callback; the interruption callback is polled during
metadata and compilation. Signal handlers, source/configuration/tool exclusion
and artifact lifetime stay with the caller. This result does not establish
immutable artifact custody or release admission.

## Development

Shared Tooling 0.15.2 is pinned in
[.shared-tooling.snapshot](.shared-tooling.snapshot); commands use this reviewed
local snapshot. The [Make interface](docs/make-interface.md) owns common setup,
formatting, hook installation and release entrypoints.

The virtual root [Cargo manifest](Cargo.toml) owns the workspace version, edition 2024,
MSRV 1.91.0, common metadata and lints. The
[ic-build package](crates/ic-build/Cargo.toml) inherits them and permits registry
publication through the explicit tagged-release command. The [library root](crates/ic-build/src/lib.rs)
exports the generic engine. The root [lockfile](Cargo.lock) records its selected
dependencies; none depend on Testkit, PocketIC or Canic.

```text
Cargo.toml                  # Virtual workspace and shared metadata
Cargo.lock                  # One selected graph
crates/ic-build/Cargo.toml   # Inherited package metadata; publish = true
crates/ic-build/src/lib.rs   # Generic engine modules
```

Prepare the declared development compiler and common tools explicitly:

```bash
rustup toolchain install --profile minimal --component rustfmt --target wasm32-unknown-unknown --no-self-update
make install-tools
cargo fetch --locked
make install-hooks
```

Release preparation uses the shared workspace-version renderer and does not
require an additional Cargo editor. Global Cargo plugins are not setup
prerequisites.

[rust-toolchain.toml](rust-toolchain.toml) selects the existing Rust 1.99.0
development compiler, rustfmt, Clippy and its Wasm target independently of the qualified Linux
library MSRV. Explicit setup reads that target from the toolchain file. The complete
common toolset is prepared and checked on Linux x86-64. Interactive shells use
the PATH selection in [local developer setup](docs/local-setup.md).
Fetch the selected workspace dependencies before the offline validation gate;
installing developer tools does not populate this project cache. CI performs the
same locked fetch on each host and retains its preparation log on failure.
`make test` resolves the repository-selected compiler and passes its absolute
path as `RUSTC` to the test command. Independent Cargo/Wasm fixtures therefore
use the prepared compiler even outside this repository's toolchain selection.

| Command | Purpose |
| --- | --- |
| `make help` | List the repository commands. |
| `make version` | Read the current workspace version. |
| `make tasks` | Read the shared maintenance catalog; scheduling remains explicit. |
| `make tools-check` | Verify all 12 common tools offline. |
| `make fmt` / `make fmt-check` | Sort Cargo manifests and format / check all workspace Rust source. |
| `make install-hooks` | Activate the shared pre-commit formatter for selected staged files. |
| `make check-package` / `make docs-check` | Compile the engine and build rustdoc with warnings denied. |
| `make clippy` | Check the library, tests and examples with all features and warnings denied. |
| `make test` | Run behavioral, cross-process retention/locking and documentation tests. |
| `make msrv` | Compile the engine using the minimum compiler declared in Cargo.toml. |
| `make check` / `make ci` | Run the full gate: snapshot, tools, links, metadata, dependency/pin checks, formatting, compilation/Clippy/rustdoc/tests/MSRV and isolated release routing. |
| `make install-validation-tools` | Prepare pinned Linux x86-64 ShellCheck, actionlint and Gitleaks. |
| `make lint check-secrets` | Check shell/workflow sources and scan committed history for secrets. |
| `make check-release-adapters` | Test release metadata, recovery and publication routing in isolated fixtures. |
| `make release-patch` / `make release-minor` / `make release-major` | Validate and deliver the selected release through Shared Tooling. |
| `make publish-check` / `make publish` | Dry-run / upload the clean, tagged release to crates.io. |

The full local delivery suite is `make ci` plus `make lint check-secrets` on
Linux after explicit tool setup. [CI](.github/workflows/ci.yml) runs lint/security
on Linux and the package gate on Linux x86-64, macOS 15 Apple Silicon and
macOS 15 Intel. It explicitly prepares the minimum compiler from Cargo.toml.
Failed native jobs use the shared evidence action. The exact 0.12.0 release source
passes all three hosts and lint/security in the release CI linked above; unpublished
changes still require their own checks.

Keep Cargo.lock tracked: the full shared pin checker enforces lockfile custody
as well as root/member dependency inheritance. Each dependency graph is checked
with its selected lockfile; validation never changes package versions.

`make cloc` reports workspace Rust LOC. The transferred tests execute real
filesystem and process boundaries, Cargo metadata and Wasm compilation, alongside
synthetic producer fixtures for failure and concurrency cases. The 0.12.0 release
gate passes on Linux and both required native macOS hosts. The
[release and publication guide](docs/publishing.md) describes the consumer
adapters, exact tagged-source admission and retained recovery evidence.
The adapter fixtures qualify command behavior; they do not establish publication
or native macOS support. Published-version custody is recorded in the handoff.

Existing Testkit v1 stamp bytes, digest domains and `.ic-testkit` cache/lock names
are retained unchanged during the ownership move. There is one reader and no
automatic cache migration or cleanup. Returned records must remain alive until
their artifact paths have finished being consumed.

The [transactional transform example](crates/ic-build/examples/transactional_artifact_cache.rs)
composes a caller-selected, admitted tool with the generic cache; it chooses its
own recipe and maintenance policy. Follow [AGENTS.md](AGENTS.md) and the ownership
design for further convergence of Canic's complete-build mechanics.

## Coordination

The design owns architectural decisions, not issue status. GitHub issues in the
owning [GitHub repository](https://github.com/dragginzgame/ic-build) remain the
sole follow-up tracker.
Changes in Canic, Testkit, IcyDB or Shared Tooling require authorization for that
repository. Consumer migrations and cleanup require their own scoped authorization.
