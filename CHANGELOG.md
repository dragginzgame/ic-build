# Changelog

The 0.1.0 section preserves the original bootstrap notes. The published 0.1.0
archive also includes the extracted artifact engine and Cargo grouping API.

## [0.9.0]

- **Breaking:** remove the fourth, output-directory argument from
  `WasmBuildSpec::new`. Cargo profile arguments now select both the build profile
  and its artifact directory; replacing those arguments updates the directory.
  Callers must remove the fourth argument and keep profile selection in
  `with_cargo_profile_args`.
- Preserve existing fingerprint encoding and cache namespaces for matching
  recipes. Malformed profiles and unsafe derived directories still refuse
  before tool execution or acquisition.
- Consolidate artifact identifier/label length checks and use Host's existing
  raw-byte SHA-256 API for configuration observations, preserving limits,
  diagnostics and raw digest bytes.
- **Testing:** restore batch specification-failure coverage after the constructor
  migration and isolate the refusal fixture from source-tree artifacts.

## [0.8.0] - 2026-10-16

- **Breaking:** align exposed Host types with the selected IC Host 0.21 graph.
  Consumers passing Host limits or typed errors across this boundary must align
  their direct Host dependencies and rebuild.

- Decode standalone and archive-qualification Cargo metadata directly into the
  canonical schema without an intermediate JSON tree; retain original JSON in
  cache fingerprinting.
- Adopt Shared Tooling 0.15.2 and delegate locked release-cache preparation and
  pin checks to its shared preflight helper, preserving source/recovery admission
  ([#39 — Shared release preflight](https://github.com/dragginzgame/ic-build/issues/39)).
- Remove the unused Cargo release-editor installation, pin and admission flow;
  the shared workspace-version renderer prepares versions directly.
- Use Host's existing SHA-256 formatter for input digests, preserving lowercase
  hexadecimal output, leading zeroes and cache identities.
- Observe each Cargo configuration lookup path and include tree once per call,
  retaining distinct aliases, include ordering and fresh-call re-observation.
- Simplify depfile parsing by collecting dependencies directly and retaining
  only target presence, preserving strict syntax refusal and phony rules.
- Reuse Host's SHA-256 parser for package checksums while retaining lowercase
  syntax, qualification diagnostics and exact external-selection checks.
- Run shared-target coordination and scheduled-maintenance process qualification
  against independent Build-owned Cargo/Wasm fixtures, removing skips for a
  missing Testkit package.
- Share fixture-directory setup across unit and integration tests and use Host
  marker waits with explicit timeouts and early worker-exit diagnostics.

## [0.7.0] - 2026-10-15

- **Breaking:** align exposed Host types with IC Host 0.20. Consumers passing
  Host limits or typed errors across this boundary must align their
  direct Host dependencies and rebuild. Keep trusted digest admission in the
  example and integration tests through Host's new `VersionSpec` API
  ([#9 — Exposed Host boundary](https://github.com/dragginzgame/ic-build/issues/9)).

- Verify exact workspace-version projections in Cargo lockfiles without allowing
  dependency, source, checksum or unrelated metadata changes. Version selection
  and release receipts remain caller-owned
  ([#28 — Strict workspace lock projections](https://github.com/dragginzgame/ic-build/issues/28)).
- Hash captured input evidence relative to an explicit root for relocatable
  source fingerprints, preserving absolute-path digests and caller-owned
  inventories and provenance policy
  ([#40 — Relative source evidence](https://github.com/dragginzgame/ic-build/issues/40)).
- Compose inherited Cargo Rust flags with caller-selected arguments, preserving
  encoded-variable precedence, explicitly empty values and argument boundaries
  ([#36 — Cargo Rust flags](https://github.com/dragginzgame/ic-build/issues/36)).

## [0.6.1] - 2026-10-15

- Consolidate Cargo argument interpretation, exact-cache preparation and private
  Cargo fixture scaffolding while preserving public builders, fingerprint bytes
  and retained cache formats
  ([#46 — Internal cleanup](https://github.com/dragginzgame/ic-build/issues/46)).
- Stream captured artifacts through Host's bounded copy and durable writer,
  avoiding a complete in-memory copy while retaining source admission, typed
  failures and staging lifetime
  ([#45 — Capture and Host mechanics](https://github.com/dragginzgame/ic-build/issues/45)).
- Share read-only Cargo configuration and include observations through
  `cargo::environment`; reuse that owner in the Wasm engine while preserving
  cache identities, lookup aliases and caller-owned admission
  ([#35 — Cargo configuration observations](https://github.com/dragginzgame/ic-build/issues/35)).
- Correct registry and Host dependency guidance for published 0.6.0 and record
  Canic's delivered workspace-discovery and Host-boundary adoption.

## [0.6.0] - 2026-10-15

- **Breaking:** move `artifacts::workspace_root_for` to
  `cargo::observation::workspace_root_for` and require the caller's Cargo command
  and capture limits. Offline membership discovery still avoids dependency
  resolution and toolchain installation; there is no old-path reexport.
- Let owned-directory fields perform Drop cleanup directly, preserving staging
  cleanup before transaction locks are released and explicit recovery behavior
  ([#43 — Discovery and cleanup boundaries](https://github.com/dragginzgame/ic-build/issues/43)).

## [0.5.0] - 2026-10-15

- Adopt Shared Tooling 0.15.0's committed snapshot and strict release-input
  admission; qualify registry IC Host 0.18.6 with unchanged API minimums and
  executable pins.
- **Breaking:** merge `WasmBuildSpec::with_extra_env` overrides across calls,
  with later values winning for duplicate names. Empty calls preserve prior
  values; construct a new specification when a fresh environment map is needed.
- **Breaking:** rename `WasmBuildSpec::with_rustc_program` to
  `with_rustc_identity_program`. This selects only the compiler identity probe;
  explicit `RUSTC` still selects Cargo's compiler and takes probe precedence.
- **Breaking:** remove `artifacts::test_target_dir`. Consumers select their own
  test output paths, for example `workspace.join("target").join(name)`.
- Simplify collect-all artifact batching while preserving phase timings,
  synchronous abort and retained primary/cleanup failures
  ([#42 — Batch and boundary cleanup](https://github.com/dragginzgame/ic-build/issues/42)).

- Correct the registry dependency and Shared Tooling guidance for published 0.4.0,
  and reconcile delivered MSRV and retained-capture acceptance.

## [0.4.0] - 2026-10-15

- **Breaking:** require Rust 1.91.0 or newer and select `cargo-platform` 0.3.3.
  Consumers adopting this release must prepare that compiler or newer; published
  0.3.3's Rust 1.88 declaration is unchanged
  ([#34 — Dependency graph MSRV](https://github.com/dragginzgame/ic-build/issues/34)).
- Use equivalent Rust 1.91 duration constructors in tests and the cache example
  to satisfy strict Clippy after the MSRV increase.
- **Breaking:** remove the public `wasm_path` and `wasm_artifacts_ready` helpers.
  Obtain verified output paths from retained artifact records; fixture-only path
  construction belongs in consumer tests.
- Decode Cargo metadata through the canonical schema while preserving Wasm
  cache fingerprint projections, probe cancellation and retained diagnostics.
- Delegate package-source traversal on Unix to Host's descriptor-based walker,
  preserving source budgets and exact byte/mode evidence.

- Adopt Shared Tooling 0.14.0's committed governance snapshot and concise
  change-reporting guidance, preserving executable pins and release behavior.

## [0.3.3] - 2026-10-14

- Observe Cargo catalogs, resolved graphs and selected dependency trees through
  caller-bounded execution and Cargo's canonical Rust schema; share tree capture
  with feature-safe grouping
  ([#33 — Cargo observations](https://github.com/dragginzgame/ic-build/issues/33)).
- Finalize captured artifacts at caller-selected destinations with explicit
  retained-file and staging custody on rename or cleanup failure
  ([#26 — Capture finalization](https://github.com/dragginzgame/ic-build/issues/26)).
- Reconcile the release and PR handoff with published 0.3.2, preserving separate
  native, consumer-adoption and Canic peer-reproduction acceptance
  ([#1 — Extraction acceptance](https://github.com/dragginzgame/ic-build/issues/1)).

## [0.3.2] - 2026-10-14

- Adopt published Host fixture allocation in both test helpers, preserving
  caller-owned cleanup and retained evidence
  ([#27 — Adopt Host fixture allocation](https://github.com/dragginzgame/ic-build/issues/27)).
- Consolidate owned-directory cleanup across Wasm builds, artifact transactions
  and captures, preserving failure retries, explicit recovery and lock lifetime
  ([#32 — Consolidate cleanup guards](https://github.com/dragginzgame/ic-build/issues/32)).
- Correct the Host dependency guidance for published 0.3.1. Callers exchanging
  Host types with IC Build must align direct dependencies to Host 0.18
  ([#9 — Align the exposed Host boundary](https://github.com/dragginzgame/ic-build/issues/9)).

## [0.3.1] - 2026-10-14

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
