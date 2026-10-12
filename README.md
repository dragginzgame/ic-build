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

`make check` (also `make ci`) verifies the reviewed shared snapshot and local
Markdown links. It requires Bash and Perl. Shared Tooling 0.9.0 is pinned in
[.shared-tooling.snapshot](.shared-tooling.snapshot); no mutable sibling checkout
is used by these commands.

The common `make install-tools` and `make tools-check` commands are available.
Setup is explicit and has not been run during bootstrap. Release entrypoints use
the shared runner, but release metadata admission refuses until there is a real
package and a reviewed release adapter. No initial package version is invented.

When implementation starts, use a virtual root Cargo workspace with
`crates/ic-build/`, inherited package metadata and dependencies, and the qualified
Rust/MSRV and native-host requirements described in [AGENTS.md](AGENTS.md).

## Coordination

The design owns architectural decisions, not issue status. GitHub issues in the
owning repository remain the sole follow-up tracker once a remote is established.
Changes in Canic, Testkit, IcyDB or Shared Tooling require authorization for that
repository. Saving this plan does not authorize their extraction or cleanup.
