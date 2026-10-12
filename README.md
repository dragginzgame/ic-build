# IC Build

IC Build contains the unpublished extraction candidate for common Cargo/Wasm
builds and retained artifact caches, moved from IC Testkit, and feature-safe
Cargo grouping from Canic. The owning
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

The dependency direction is application adapters → IC Build → IC Host
Tooling. IC Build must not depend on Canic, IcyDB, PocketIC or IC Testkit.

## Development

Shared Tooling 0.9.5 is pinned in
[.shared-tooling.snapshot](.shared-tooling.snapshot); commands use this reviewed
local snapshot. The [Make interface](docs/make-interface.md) owns common setup,
formatting, hook installation and release entrypoints.

The virtual root [Cargo manifest](Cargo.toml) owns version `0.1.0`, edition 2024,
MSRV 1.88.0, common metadata and lints. The
[ic-build package](crates/ic-build/Cargo.toml) inherits them and disables registry
publication while extraction is qualified. The [library root](crates/ic-build/src/lib.rs)
exports the generic engine. The root [lockfile](Cargo.lock) records its selected
dependencies; none depend on Testkit, PocketIC or Canic.

```text
Cargo.toml                  # Virtual workspace and shared metadata
Cargo.lock                  # One selected graph
crates/ic-build/Cargo.toml   # Inherited package metadata; publish = false
crates/ic-build/src/lib.rs   # Generic engine modules
```

Prepare the declared development compiler and common tools explicitly:

```bash
rustup toolchain install --profile minimal --component rustfmt --target wasm32-unknown-unknown --no-self-update
make install-tools
make install-hooks
```

[rust-toolchain.toml](rust-toolchain.toml) selects the existing Rust 1.99.0
development compiler independently of the qualified Linux library MSRV. The complete
common toolset is prepared and checked on Linux x86-64. Interactive shells use
the PATH selection in [local developer setup](docs/local-setup.md).

| Command | Purpose |
| --- | --- |
| `make help` | List the repository commands. |
| `make version` | Read the initial workspace version. |
| `make tasks` | Read the shared maintenance catalog; scheduling remains explicit. |
| `make tools-check` | Verify all 12 common tools offline. |
| `make fmt` / `make fmt-check` | Sort Cargo manifests and format / check all workspace Rust source. |
| `make install-hooks` | Activate the shared pre-commit formatter for selected staged files. |
| `make check-package` / `make docs-check` | Compile the engine and build rustdoc with warnings denied. |
| `make test` | Run behavioral, cross-process retention/locking and documentation tests. |
| `make msrv` | Compile the engine using the minimum compiler declared in Cargo.toml. |
| `make check` / `make ci` | Run the full gate: snapshot, tools, links, metadata, dependency/pin checks, formatting, compilation/rustdoc/tests/MSRV and isolated release routing. |
| `make install-validation-tools` | Prepare pinned Linux x86-64 ShellCheck, actionlint and Gitleaks. |
| `make lint check-secrets` | Check shell/workflow sources and scan committed history for secrets. |

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
on Linux; native macOS qualification remains pending. Release entrypoints refuse
until extraction acceptance and a
reviewed release adapter exist. Version `0.1.0` is preparation, not a release.

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
repository. Saving this plan does not authorize their extraction or cleanup.
