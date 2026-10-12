# IC Build

IC Build is the planned common Cargo and Wasm artifact build engine for Canic,
IC Testkit and IcyDB. This repository currently contains the ownership and
extraction design; it does not yet contain a Rust crate or a released build engine.

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

The root [Cargo manifest](Cargo.toml) reserves the requested initial version
`0.1.0`, edition 2024 and MSRV target 1.88.0 in a virtual workspace. It has no
members yet; extraction will add the real package at `crates/ic-build/` and
qualify its dependency graph and native hosts before claiming support.

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
| `make fmt` / `make fmt-check` | Sort / check Cargo manifests through the shared formatter. |
| `make install-hooks` | Activate the shared pre-commit formatter for selected staged files. |
| `make check` / `make ci` | Run the complete bootstrap gate: snapshot, tools, all Markdown links, Cargo metadata, declared pins, formatting and isolated release-command routing. |
| `make install-validation-tools` | Prepare pinned Linux x86-64 ShellCheck, actionlint and Gitleaks. |
| `make lint check-secrets` | Check shell/workflow sources and scan committed history for secrets. |

The full local delivery suite is `make ci` plus `make lint check-secrets` on
Linux after explicit tool setup. [CI](.github/workflows/ci.yml) runs lint/security
on Linux and the bootstrap gate on Linux x86-64, macOS 15 Apple Silicon and
macOS 15 Intel. Failed native jobs use the shared evidence action. These jobs
are configured but have not yet qualified this change on native macOS.

The empty workspace has no Rust source or dependency graph. Its formatter
covers the manifest; package extraction must add Rust source formatting, the
tracked lockfile and full shared dependency checker, behavioral tests and MSRV
gates. The shared `make cloc` report requires workspace packages. Release
entrypoints use the shared runner but refuse until a real package and reviewed
release adapter exist. The workspace version is preparation, not a release.

When implementation starts, add `crates/ic-build/` to the virtual workspace with
inherited metadata and dependencies, following the Rust/MSRV and native-host
requirements in [AGENTS.md](AGENTS.md).

## Coordination

The design owns architectural decisions, not issue status. GitHub issues in the
owning [GitHub repository](https://github.com/dragginzgame/ic-build) remain the
sole follow-up tracker.
Changes in Canic, Testkit, IcyDB or Shared Tooling require authorization for that
repository. Saving this plan does not authorize their extraction or cleanup.
