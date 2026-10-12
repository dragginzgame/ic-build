# IC Build

IC Build is the planned common Cargo and Wasm artifact build engine for Canic,
IC Testkit and IcyDB. This repository contains the ownership and extraction design
and an unpublished Rust package scaffold. The build engine is not implemented.

Start with [Build ownership and extraction](docs/design/build-ownership.md).
It records the inspected build paths, duplicated responsibilities, intended
owners, retained safety boundaries and extraction sequence. Read the
[current handoff](docs/status/current.md) before implementation.

The intended dependency direction is application adapters → IC Build → IC Host
Tooling. IC Build must not depend on Canic, IcyDB, PocketIC or IC Testkit.

## Development

Shared Tooling 0.9.1 is pinned in
[.shared-tooling.snapshot](.shared-tooling.snapshot); commands use this reviewed
local snapshot. The [Make interface](docs/make-interface.md) owns common setup,
formatting, hook installation and release entrypoints.

The virtual root [Cargo manifest](Cargo.toml) owns version `0.1.0`, edition 2024,
MSRV 1.88.0, common metadata and lints. The
[ic-build package](crates/ic-build/Cargo.toml) inherits them and disables registry
publication. Its dependency-free [library root](crates/ic-build/src/lib.rs)
exposes no build API. The root [lockfile](Cargo.lock) records this scaffold's
selected graph; extracted dependencies will need separate qualification.

```text
Cargo.toml                  # Virtual workspace and shared metadata
Cargo.lock                  # One selected graph
crates/ic-build/Cargo.toml   # Inherited package metadata; publish = false
crates/ic-build/src/lib.rs   # Library scaffold
```

Prepare the declared development compiler and common tools explicitly:

```bash
rustup toolchain install --profile minimal --component rustfmt --no-self-update
make install-tools
make install-hooks
```

[rust-toolchain.toml](rust-toolchain.toml) selects the existing Rust 1.99.0
development compiler independently of the proposed library MSRV. The complete
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
| `make check-package` / `make docs-check` | Compile the scaffold and build rustdoc with warnings denied. |
| `make msrv` | Compile the scaffold using the minimum compiler declared in Cargo.toml. |
| `make check` / `make ci` | Run the complete bootstrap gate: snapshot, tools, Markdown links, Cargo metadata, shared dependency/pin checks, formatting, scaffold compilation/rustdoc/MSRV and isolated release routing. |
| `make install-validation-tools` | Prepare pinned Linux x86-64 ShellCheck, actionlint and Gitleaks. |
| `make lint check-secrets` | Check shell/workflow sources and scan committed history for secrets. |

The full local delivery suite is `make ci` plus `make lint check-secrets` on
Linux after explicit tool setup. [CI](.github/workflows/ci.yml) runs lint/security
on Linux and the bootstrap gate on Linux x86-64, macOS 15 Apple Silicon and
macOS 15 Intel. It explicitly prepares the minimum compiler from Cargo.toml.
Failed native jobs use the shared evidence action. These jobs are configured but
have not yet qualified the package scaffold on native macOS.

Keep Cargo.lock tracked: the full shared pin checker enforces lockfile custody
as well as root/member dependency inheritance. Before the new lockfile is staged,
that gate refuses the working checkout; layout preparation is validated in an
isolated checkout without changing the real index.

`make cloc` reports the workspace's Rust LOC. The scaffold has no implementation
or behavioral tests. Package extraction must introduce the engine and its real
behavioral/MSRV gates. Release entrypoints use the shared runner but refuse until
engine extraction and a reviewed release adapter exist. Version `0.1.0` is
preparation, not a release.

When implementation starts, extract the existing generic implementation into
`crates/ic-build/`, following [AGENTS.md](AGENTS.md) and the ownership design.

## Coordination

The design owns architectural decisions, not issue status. GitHub issues in the
owning [GitHub repository](https://github.com/dragginzgame/ic-build) remain the
sole follow-up tracker.
Changes in Canic, Testkit, IcyDB or Shared Tooling require authorization for that
repository. Saving this plan does not authorize their extraction or cleanup.
