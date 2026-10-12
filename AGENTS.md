# IC Build Agent Rules

Read [docs/status/current.md](docs/status/current.md), then
[DRAGGINZGAME.md](DRAGGINZGAME.md) and the
[ownership design](docs/design/build-ownership.md).
The baseline is Shared Tooling 0.9.1 revision
`e8a9ea6b660f216166eb00d0af6a32462bdbda4a`, recorded with exact digests in
[.shared-tooling.snapshot](.shared-tooling.snapshot).

- Mutate only this repository unless the maintainer expressly authorizes another
  target. Existing sibling repositories remain read-only.
- This is a planning bootstrap. Do not claim that an engine, crate, migration,
  native qualification or release exists before it is implemented and verified.
- Extract one implementation from its existing owners; do not copy a second
  production engine or leave compatibility reexports and parallel owners behind.
- IC Build owns generic build/artifact mechanics. Consumers own recipes, budgets,
  tool selections, credentials, source custody, release identities and lifecycle.
  Keep Canic release admission and Testkit's PocketIC lifecycle outside this crate.
- Preserve retained artifacts, errors, cancellation, input-change rejection,
  feature-safe batching and artifact lifetime. Builds have no finite default
  elapsed-time deadline. Maintenance and cleanup require explicit policy.
- For Rust implementation, use edition 2024 and initially target the existing
  Host/Testkit MSRV of 1.88.0, qualifying the extracted dependency graph before
  advertising support. Use a virtual root workspace and `crates/ic-build/`.
  Do not add Python tooling.
- Linux x86-64 and native macOS 15 Intel/Apple Silicon are the initial required
  hosts. Linux or substitute checks do not establish native macOS qualification.
- Check active Cargo/rustc processes before edits or builds. Preserve dirty source
  and artifacts. The bootstrap validation is `make check` (also `make ci`), plus
  `make lint check-secrets` on Linux with explicitly prepared validation tools.
  Extend formatting, dependency checking and `make ci` with actual package/MSRV
  gates when executable code is introduced; do not claim empty-workspace checks
  establish package behavior.
- Follow [contribution rules](rules/contributions.md). Staging, commits, pushes,
  package versions, releases and publication are user-owned unless delegated.
- GitHub issues are the follow-up tracker. Keep the design and handoff as evidence
  and architecture; do not create local issue queues.
