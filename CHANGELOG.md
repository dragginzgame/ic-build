# Changelog

The 0.1.0 section preserves the original bootstrap notes. The published 0.1.0
archive also includes the extracted artifact engine and Cargo grouping API.

## [0.3.1]

- Adopt Shared Tooling 0.12.7 and delegate release metadata preparation and
  exact-commit validation to its canonical transaction helper, preserving local
  source admission, selected-editor custody and interrupted-release recovery.
- Stream Cargo archive qualification hashes through Host's bounded no-follow
  reader instead of retaining whole files solely for digesting.

- Qualify coordinated Cargo archives in a fresh retained workspace; admit exact
  extracted manifests and frozen external selections, then check source/archive
  integrity after tests. Consumers retain tools, package/version and release policy.
  ([#25](https://github.com/dragginzgame/ic-build/issues/25)).

- Add opt-in caller deadlines, cancellation and cleanup for Cargo input probes
  and revalidation. Preserve default execution, complete-output bounds and
  input identity ([#22 — Caller-bounded Cargo input resolution](https://github.com/dragginzgame/ic-build/issues/22)).

## [0.3.0] - 2026-10-14

- Adopt Shared Tooling 0.12.4 and published IC Host 0.17.11, retaining the
  reviewed tool pins and explicit dependency features.
- Delegate remaining descriptor locks and final-owner unlock to Host and remove
  the direct `fs2` dependency. Preserve cloned-record retention, inherited
  descriptor release and lock timing
  ([#21 — Adopt Host descriptor locks](https://github.com/dragginzgame/ic-build/issues/21)).

- **Breaking:** remove the watched-input stamp API (`WatchedInputSnapshot` and
  `icp_artifact_ready_*`). Use `InputEvidence` for input guards and
  `ArtifactCacheSpec` for verified retained outputs; existing sidecars are
  preserved ([#19 — Retire the watched-input stamp API](https://github.com/dragginzgame/ic-build/issues/19)).
- Share the verified destination-copy decision between Wasm and generic artifact
  caches, preserving digest domains, cold publication and stamp ordering
  ([#20 — Unify verified destination materialization](https://github.com/dragginzgame/ic-build/issues/20)).

- Use IC Host 0.17.9's opt-in Unix test fixture writer and remove both local
  executable-script implementations. Keep fixture content and assertions with
  IC Build ([#18 — Adopt Host executable fixtures](https://github.com/dragginzgame/ic-build/issues/18)).

## [0.2.7] - 2026-10-14

- Require IC Host 0.17.8's checked native permission conversion, preventing
  selection of the earlier retained-staging macOS compilation defect
  ([Host #84](https://github.com/dragginzgame/ic-host-tooling/issues/84)).

## [0.2.6] - 2026-10-14

- Add exact Cargo depfile reads through the existing discovery owner, allowing
  consumers to select their build graph without inspecting unrelated records
  in shared targets. Preserve bounded reads, exclusions and late-input rejection
  ([#17](https://github.com/dragginzgame/ic-build/issues/17)).

## [0.2.5] - 2026-10-13

- Replace the duplicated optional-duration helper with standard-library reduction
  in Wasm and artifact timing aggregation. Preserve absent measurements, measured
  zero and saturation; keep regression coverage at the aggregate owners
  ([#7](https://github.com/dragginzgame/ic-build/issues/7)).

## [0.2.4] - 2026-10-13

- Adopt Shared Tooling 0.11.2 and its unified CI binary installer, retaining
  existing tool pins and incorporating release/evidence fixes.
- Require compatible IC Host 0.17.4 releases for artifact, filesystem and process
  mechanics; retain explicit features and qualify the locked graph on Rust 1.88.

- Admit Cargo's default underscored cdylib filenames for hyphenated packages in
  every cache mode; refuse colliding outputs and genuinely renamed libraries
  ([#16](https://github.com/dragginzgame/ic-build/issues/16)).
- Add explicit captured-artifact retention for caller-owned recovery evidence
  ([#14](https://github.com/dragginzgame/ic-build/issues/14)).
- Bind named cdylib examples to shared Cargo input identity and output admission.
  Expose the existing command recipe for caller-owned execution without requiring
  a cache or changing deadlines, cleanup or consumer receipt encoding
  ([#13](https://github.com/dragginzgame/ic-build/issues/13)).

## [0.2.3] - 2026-10-13

- Make the native-path evidence fixture use filesystem-admissible filenames on
  macOS, retaining native capture, digest and descendant-error coverage. Linux
  still exercises invalid-byte filenames
  ([#15](https://github.com/dragginzgame/ic-build/issues/15)).
- Keep the remaining generic source-inventory and generated-root discovery
  regressions at IC Build as Canic adopts standalone evidence and removes its
  duplicate input engine ([#12](https://github.com/dragginzgame/ic-build/issues/12)).

## [0.2.2] - 2026-10-13

- Add standalone input evidence and strict Cargo depfile discovery for caller-owned
  build inventories. Recheck original inputs when inventories shrink or change,
  refuse late input authority, and preserve consumer evidence and existing cache
  encodings through the shared traversal. Recipes and release admission remain
  with consumers ([#12](https://github.com/dragginzgame/ic-build/issues/12)).
- Correct the artifact guide's IC Build ownership and Host 0.17 requirements.
  Keep source-lease and retained-cache contracts here, and bind historical
  Testkit references to their extraction revision
  ([#2](https://github.com/dragginzgame/ic-build/issues/2)).

## [0.2.1] - 2026-10-13

- Adopt Shared Tooling 0.10.0's snapshot-owned common catalogs. Keep existing
  executable versions and digests; select host, Rust and IC exceptions
  independently. Library APIs and cache formats are unchanged
  ([#10](https://github.com/dragginzgame/ic-build/issues/10)).

## [0.2.0] - 2026-10-13

- Document full LTO as a candidate shared deployable-Wasm release default,
  using the existing profile/environment API. Record IC Timers' linked-canister
  measurements and require complete-application qualification through final
  transforms before adoption; engine defaults and public APIs are unchanged.

- **Breaking:** align the public Cargo grouping/capture boundary with IC Host
  0.17. Consumers supplying `OutputLimits` or matching typed Host failures must
  select Host 0.17. Artifact, filesystem and process dependencies now share that
  family; retained artifact and cache formats are unchanged
  ([#9](https://github.com/dragginzgame/ic-build/issues/9)).
- Keep release-adapter tests independent of the live draft by supplying their
  own disposable patch notes, so a pending minor release can be validated.
- Prepare and admit a pinned Cargo editor through shared tooling, so fresh CI
  and release preparation do not depend on a global `cargo set-version`.
  Failed adapter fixtures show bounded inner diagnostics and retain full logs
  ([#11](https://github.com/dragginzgame/ic-build/issues/11)).

## [0.1.4] - 2026-10-13

- Qualify the extracted library, tests and examples with warning-denied Rust
  Clippy, and include that check in `make check` / `make ci`. Document public
  error contracts and preserve source leases through their final consumers;
  prepare Clippy through the declared development toolchain
  ([#8](https://github.com/dragginzgame/ic-build/issues/8)).
- Declare the development Wasm target in `rust-toolchain.toml`, so Rustup
  prepares it on fresh runners when installing the active toolchain. Preserve
  the prepared compiler selection and real positive-heartbeat build
  ([#6](https://github.com/dragginzgame/ic-build/issues/6)).

## [0.1.3] - 2026-10-13

- Add temporary artifact staging and bounded capture of completed producer outputs.
  Keep captured bytes alive across later compilations; callers retain recipes,
  qualification and release publication. This prepares Canic's staging removal
  ([Canic #529](https://github.com/dragginzgame/canic/issues/529)); consumer adoption
  requires this API to be published.
- Carry the repository-prepared compiler through the test gate into independent
  real Wasm fixtures, so a different ambient default does not lose the installed
  Wasm target. Keep heartbeat rejection and positive-build qualification
  ([#6](https://github.com/dragginzgame/ic-build/issues/6)).

## [0.1.2] - 2026-10-12

- Prepare the locked workspace dependency cache on every CI host before offline
  validation, so fresh runners can compile the package and run its gates
  ([#5](https://github.com/dragginzgame/ic-build/issues/5)).

## [0.1.1] - 2026-10-12

- Preserve typed Cargo grouping failures, including original status and bounded
  stdout/stderr, through Host's existing execution error
  ([#3](https://github.com/dragginzgame/ic-build/issues/3)).
- Reject duplicate and empty package inputs before invoking the Cargo command
  factory, reporting the offending input and zero-based indices. Empty package
  lists remain empty plans; valid grouping semantics are unchanged
  ([#4](https://github.com/dragginzgame/ic-build/issues/4)).
- Align current documentation and repository description with published 0.1.0,
  add a direct dependency/import example, and distinguish pending consumer/native
  acceptance from registry availability
  ([#2](https://github.com/dragginzgame/ic-build/issues/2)).
- Enable the shared release workflow with workspace/lock/changelog preparation
  and recovery, and separate clean tagged-source publication to crates.io.
  Release and publication retain artifacts; cleanup remains explicit.
- Record qualification of Testkit's extracted generic artifact/Wasm engine and
  behavioral/process tests, and Canic's feature-equivalent Cargo grouping. Retain
  the existing cache formats and namespaces; consumers select recipes and retain
  their release/runtime policy
  ([#1](https://github.com/dragginzgame/ic-build/issues/1)).
- Run the extracted behavioral, cross-process and real Cargo/Wasm tests in the
  package gate and include the preserved MIT license notice in package contents.
- Keep SPDX MIT metadata without the redundant license-file field; package the
  original notice as a regular crate-local file, compatible with the shared
  formatting hook and without Cargo warnings.
- Adopt Shared Tooling 0.9.5 with bounded tool-download retries, complete Git
  identity checks and hardened evidence/path verification
  ([Shared #138](https://github.com/dragginzgame/shared-tooling/issues/138),
  [Shared #133](https://github.com/dragginzgame/shared-tooling/issues/133)).

## [0.1.0]

- Record the build ownership and extraction design for the planned common engine.
- Reserve initial Cargo workspace metadata; no build-engine package is implemented
  or released yet.
- Adopt Shared Tooling 0.9.1 and prepare the complete common developer toolset
  through its shared executable store.
- Add the common contributor Make interface, manifest formatting hook, explicit
  development toolchain and bootstrap CI for Linux and both macOS 15 architectures,
  with pinned validation tools and retained failure evidence.
- Add an unpublished `ic-build` library scaffold in the standard `crates/`
  layout, inheriting workspace metadata and lints, with a root lockfile, source
  formatting and compilation/documentation/MSRV gates. The engine remains planned.
