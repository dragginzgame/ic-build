# IC Build

IC Build provides common Cargo/Wasm builds and retained artifact caches, moved
from IC Testkit, and feature-safe Cargo grouping from Canic.
The [published 0.9 family is available on crates.io](https://crates.io/crates/ic-build/0.9.0).
The owning
[extraction issue](https://github.com/dragginzgame/ic-build/issues/1) tracks
producer and consumer acceptance.

`ic_build::artifacts` owns exact Cargo input resolution, transactional artifact
publication, retained records, Wasm caches, sessions, batch results and explicit
maintenance. `ic_build::cargo` groups packages only when their resolved normal
and build dependency trees match isolated resolution. Consumers own recipes,
tools, output budgets, source custody, release identities and lifecycle.
PocketIC stays in Testkit; complete-release admission stays in Canic.

Start with [Build ownership and extraction](docs/design/build-ownership.md).
It records the inspected build paths, duplicated responsibilities, intended
owners, retained safety boundaries and extraction sequence. Read the
[current handoff](docs/status/current.md) before implementation.
The [artifact API guide](docs/artifacts.md) covers recipes, retained records,
freshness, batching and explicit maintenance.
Version 0.1.3 also provides temporary staging and bounded capture of producer
outputs, with caller-owned qualification and publication.

The dependency direction is application adapters → IC Build → IC Host
Tooling. IC Build must not depend on Canic, IcyDB, PocketIC or IC Testkit.

## Use

The examples below use the published 0.9 API. Add the registry dependency to
your host-side package:

```toml
[dependencies]
ic-build = "0.9"
```

Rust 1.91.0 or newer is required. Import the engine directly and keep the returned
record alive while using its artifacts. With a prepared Cargo/Wasm toolchain and a
caller-held source lease:

```rust,no_run
use ic_build::artifacts::{WasmBuildSpec, build_wasm_canisters_cached};
use std::path::Path;

fn build(workspace: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let target = workspace.join("target/canister-builds");
    let spec = WasmBuildSpec::new(workspace, &target, &["counter_canister"])
        .with_cargo_profile_args(["--release", "--locked"]);
    let outcome = build_wasm_canisters_cached(&spec)?;
    let wasm = std::fs::read(&outcome.record().artifacts()[0])?;
    Ok(wasm)
}
```

Consumers select packages, profiles, environment and maintenance policy.
To adopt 0.9, remove the fourth argument from `WasmBuildSpec::new` and retain
profile selection through `with_cargo_profile_args`. Output directories follow
those arguments automatically, including when a later call replaces them.
Published 0.6.1 adds `ic_build::cargo::environment` for read-only configuration
discovery, include traversal, raw-byte digests and compiler-override facts. Consumers select
effective or conservative input paths and retain compiler admission, receipt
framing and source custody. Configuration contents are excluded from returned
facts and failure diagnostics. Observation executes no Cargo commands and does
not establish build or release validity.

IC Build 0.8 and 0.9 use IC Host 0.21 at the public grouping/capture boundary;
align direct Host dependencies before passing
`OutputLimits` or typed Host failures across that boundary.
The [deployable-Wasm LTO guidance](docs/artifacts.md#deployable-wasm-lto) records
whole-canister measurements and shows how a common adapter can apply full LTO
through the existing API, with complete-application qualification before adopting
a shared release default.
Publication makes the engine available; [#1](https://github.com/dragginzgame/ic-build/issues/1)
still tracks consumer acceptance, Canic production convergence and required native
macOS qualification. See the artifact guide for source custody and retained-cache
contracts.

## Native executables

Pending 0.9.1 adds `cargo::native::build_native_executable` on Unix. It builds an
explicit package's binary or example and returns Cargo's reported executable and
target root, without reconstructing paths for custom profiles or target directories.
The caller supplies its Cargo command factory, profile/features, diagnostics writer
and per-message JSON allowance. Set shared manifest/configuration/target paths in
the factory so metadata and build use the same recipe; build-only selection and
path overrides are refused. Compiler diagnostics stream without a default deadline.

```rust,no_run
use ic_build::cargo::native::{NativeExecutableSpec, build_native_executable};
use std::process::Command;

let spec = NativeExecutableSpec {
    package: "my-package",
    target_name: "worker",
    target_kind: "example",
    build_args: &[],
    max_message_bytes: 16 * 1024 * 1024,
};
let built = build_native_executable(&spec, || {
    let mut cargo = Command::new("cargo");
    cargo.current_dir("/checkout");
    cargo
}, std::io::stderr(), || Ok(()))?;
# Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
```

Admission runs before each dispatch; commands already running complete through
Host's direct-child owner. Source/configuration/tool exclusion, in-flight signal
policy and artifact lifetime stay with the caller. This result does not establish
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
Failed native jobs use the shared evidence action. These jobs are configured but
have not yet qualified the extracted engine on native macOS.

Keep Cargo.lock tracked: the full shared pin checker enforces lockfile custody
as well as root/member dependency inheritance. Each dependency graph is checked
with its selected lockfile; validation never changes package versions.

`make cloc` reports workspace Rust LOC. The transferred tests execute real
filesystem and process boundaries, Cargo metadata and Wasm compilation, alongside
synthetic producer fixtures for failure and concurrency cases. These checks passed
on Linux; native macOS qualification remains pending. The
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
