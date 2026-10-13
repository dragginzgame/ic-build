# Artifact and Wasm build APIs

These generic APIs are extracted from IC Testkit. Recipes, budgets, tool
selection, credentials, source custody and lifecycle remain with the caller.
The existing v1 retained formats and namespaces are unchanged.

## Deployable Wasm LTO

Full LTO is the leading candidate for a shared deployable-Wasm release default.
Qualify it against complete applications, then apply the selected policy in their
common build adapter. A library's own profile does not control its consumers:
[Cargo reads profiles from the workspace root](https://doc.rust-lang.org/cargo/reference/profiles.html#lto),
and full LTO optimizes across the linked dependency graph. There is no requirement
for every application to maintain a separate manifest edit.

The existing API can apply a shared release selection explicitly:

```rust,no_run
use ic_build::artifacts::WasmBuildSpec;
use std::path::Path;

let workspace = Path::new("/application/workspace");
let target = workspace.join("target/canister-builds");
let spec = WasmBuildSpec::new(workspace, &target, &["application_canister"], "release")
    .with_cargo_profile_args(["--release", "--locked", "--offline"])
    .with_extra_env([("CARGO_PROFILE_RELEASE_LTO", "fat")]);
```

For a custom profile, select its matching output directory and Cargo argument,
and use its corresponding `CARGO_PROFILE_<NAME>_LTO` variable. Keep other explicit
overrides in the same `with_extra_env` call; the setter replaces that map.
[Cargo's profile environment configuration](https://doc.rust-lang.org/cargo/reference/config.html#profilenamelto)
overrides the manifest. IC Build fingerprints explicit child environment and
workspace profiles, so thin and fat selections have distinct cache identities.
Prefer this declared selection over an untracked ambient override. The engine
continues to execute the caller's recipe; it does not silently change profiles.

### Initial whole-canister evidence

IC Timers measured its four linked size-probe canisters at released source
`44935d6188a0da82314e00a56f5f68a9d0c59638` (0.21.4), using Rust 1.88.0,
`wasm32-unknown-unknown`, and the same frozen Cargo.lock (Metrics 0.8.3). The
`timer-probe` profile inherited release with `opt-level = "z"`, `panic = "abort"`
and `strip = "symbols"`. Only LTO changed. All variants used the same source
path and locked/offline inputs. These are complete probe artifacts containing
CDK, Candid and other dependencies, rather than an isolated library size.

| Canister cohort | Thin LTO bytes | Full LTO bytes | Reduction |
| --- | ---: | ---: | ---: |
| Baseline | 264,414 | 214,565 | 18.85% |
| Once | 324,104 | 263,405 | 18.73% |
| After-completion | 324,974 | 264,288 | 18.67% |
| Watchdog | 325,763 | 264,670 | 18.75% |

Export names matched. No Wasm optimizer or PocketIC execution was performed;
instruction cost, heap use and comparable build-time changes remain unmeasured.
These small canisters establish a promising candidate, not the best setting for
every production app or compiler. The comparison excludes the later incoming
Metrics 0.8.4 selection. The source and four feature selections are maintained
in [IC Timers](https://github.com/dragginzgame/ic-timers/tree/44935d6188a0da82314e00a56f5f68a9d0c59638/testing/crates/ic-timers-size-probe).

### Qualifying a common release default

Compare thin and fat at the same application source, lockfile, compiler, target,
features and optimization recipe. Use the current profile's optimization level;
do not attribute simultaneous flag changes to LTO. Keep both retained outputs
alive through inspection. Apply the normal final transforms before comparing
deployable bytes, required exports, Candid and required Wasm features. Measure
representative instruction costs and clean/warm build time; run the application's
existing PocketIC acceptance against the selected final artifact.

If full LTO wins the representative application matrix within its accepted build
time and instruction budgets, adopt it once in the common deployable-Wasm adapter
and reuse it across applications. Keep development, native tools and deliberately
different product recipes scoped separately. Revisit the evidence after compiler
or optimizer changes. Build identity and execution remain in IC Build; product
acceptance and release publication remain with consumers. No new comparison
framework or public configuration type is needed for this initial qualification.

## Standalone input evidence and depfiles

`InputEvidenceSpec` declares absolute file/directory/missing roots, an opaque
caller-derived non-file identity, generated-root exclusions and optional exact
child-name exclusions. It selects no recipes, tools, output roots or time limits.
`InputEvidence::capture` uses the same tree traversal as cache fingerprints,
records complete non-excluded directory membership, hashes regular file bytes
through Host's no-follow reader, and refuses child links or special files. Caller
custody must cover sources and parent directories through capture, work and
validation; evidence does not provide a lock or release admission.

`entries()` supplies per-path `Missing`, `Directory`, or raw SHA-256 `File` facts
for consumer-owned records. `digest()` retains the existing consumer encoding:
identity followed by UTF-8 paths in lexical string order and their values, each
field prefixed by its u64 big-endian byte length. Values are `absent`, `directory`,
or lowercase file SHA-256. Capture preserves native paths; this particular digest
encoding refuses non-UTF-8 paths. Existing cache domains and field encoding remain
unchanged. Consumers keep their record formats and namespaces.

`before.validate_after(&after)` permits replacement or expansion of the declared
inventory while requiring the same identity and exclusion policy. It rechecks
every original root, even if a later depfile no longer names it, and rejects
changed bytes, absence or directory membership. Newly discovered bytes cannot
gain retrospective authority. A newly named missing path is admissible only when
an earlier complete directory scan proved its first child absent; excluded or
unobserved paths cannot establish that proof. The caller supplies the after
capture under the same source custody boundary and handles refusal according to
its own build/release policy.

`discover_cargo_depfile_inputs(current_dir, directories, excluded_roots,
maximum_record_bytes)` reads `.d` files directly from explicitly selected
directories. Missing directories yield an empty cold-build inventory. Each
record uses a caller-sized bounded, no-follow read; any malformed, oversized or
unsupported record refuses the entire discovery. Relative inputs bind to the
explicit working directory. Existing aliases resolve before output exclusion;
missing paths containing parent traversal remain conservative external inputs.
The records themselves are not source inputs: Cargo may replace them during work.

`parse_cargo_depfile` accepts Cargo/rustc Make dependency rules, escaped spaces,
tabs, backslashes, hash/colon characters, doubled dollars, LF/CRLF continuations,
multiple rules and empty phony targets. It refuses other Make expressions instead
of interpreting them or returning partial evidence. Full-line comments are
ignored, so environment dependencies belong in caller identity. Profile/target
directory selection and generated-output policy remain consumer-owned. These APIs
do not invoke Cargo, create another artifact cache or manage maintenance.

## Temporary staging and captured outputs

`ArtifactStagingDirectory::new(parent)` allocates one owned child beneath an
existing caller-selected parent. Callers choose candidate filenames and perform
their recipe and qualification there before publishing any product files.
The directory remains alive through its owner; `close()` reports cleanup errors,
and Drop performs best-effort cleanup of that invocation's child. The caller
keeps custody of the parent and staging paths against unrelated writers.

`CapturedArtifact::capture(source, parent, maximum_bytes)` copies a completed
producer output synchronously with the caller's byte budget. Keep the returned
record through every use of `path()`, including later Cargo commands that may
overwrite or delete the original output. Multiple captures remain independent,
including captures of identical source bytes. Closing or dropping a capture
releases only its temporary staging directory.
`retain()` explicitly transfers the captured path and its staging child to
caller-owned evidence retention. It preserves the bytes after the record is
consumed; the caller owns later cleanup and custody. It creates no cache or
release admission. Normal close/Drop behavior remains unchanged.

These temporary owners create no cache entry, cache hit or release admission.
They do not prune retained artifacts, run transforms, install tools or select
an elapsed-time deadline. Canic retains transform order, validation, release
binding, build coordination and durable product publication. Its producer and
consumer adoption evidence is tracked in
[Canic #529](https://github.com/dragginzgame/canic/issues/529).

## Caller-executed Cargo examples

`WasmBuildSpec::cargo_build_command()` prepares the same Cargo command used by
cached acquisition, using the specification's explicit target directory. It
starts no process or cache and executes no maintenance. Resolve the specification
first and call `ResolvedCargoBuildInputs::artifact_paths()` to admit its selected
Wasm output. Generic snapshots can describe non-Wasm packages, but this output
accessor refuses absent, non-cdylib, renamed or colliding targets.

For Query's probe, select package `ic-query`, example `governance_probe`, profile
`release`, and arguments `--release --no-default-features --features canister
--locked --offline`. Explicit compiler/tool/environment selection remains in the
specification. The command's `CARGO_TARGET_DIR` overrides a configured Cargo
output directory; discovered configuration still contributes to input identity.

Keep the existing caller executor: Query owns its 600-second deadline,
cancellation, TERM grace, bounded reaping and original command/output/cleanup
errors. Preserve the generated command's arguments and environment. Hold source
and target custody, capture pre-build `InputEvidence` from resolved inputs and
exclusions, execute, re-resolve identity and validate before capturing outputs.
Discover caller-selected example depfiles and validate their expanded inventory;
a newly discovered external input needs pre-build authority. Predeclare it or
perform a fresh build after inventory discovery. Command success alone does not
admit retained bytes or a product release.

Query's receipt encoding remains caller-owned: sorted relative paths, NUL,
raw file bytes, NUL. Use shared inventory facts to select its existing explicit
files and Rust sources, then retain that encoder. `InputEvidence::digest()`
uses different framing and is not a replacement for Query's `source_sha256`.
Query also retains DID selection, public metadata, smoke/deployment lifecycle and
publication. Host owns generic custom-section framing. This capability does not
establish Query adoption or native smoke qualification.

## Wasm artifact helpers

The host-only `artifacts` module provides workspace-relative paths, dedicated
test target directories, content-addressed build coordination, Wasm loading,
and exact generated-artifact freshness checks:

`workspace_root_for` asks prepared Cargo for actual workspace membership and
returns `io::Result<PathBuf>`. It supports App layouts, exclusions and independent
workspaces without guessing from directory names or resolving dependencies.
`WasmBuildSpec::new` shares `<workspace>/target/ic-testkit-incremental` compiler
state across fingerprints by default. Exact final artifacts remain independently
cached. Select `with_isolated_builds()` only when separate compiler state is
required; configure age/byte retention through the existing pruning and
shared-target maintenance builders. Existing isolated build directories are
preserved and need explicit maintenance if no longer wanted.

```rust,no_run
use ic_build::artifacts::{
    ArtifactCachePrunePolicy, WasmBuildOutcome, WasmBuildSpec,
    build_wasm_canisters_cached, test_target_dir, workspace_root_for,
};
use std::time::Duration;

let workspace = workspace_root_for(env!("CARGO_MANIFEST_DIR"))
    .expect("resolve the crate's Cargo workspace");
let target = test_target_dir(&workspace, "pic-wasm");
let spec = WasmBuildSpec::new(
    &workspace,
    &target,
    &["counter_canister"],
    "release",
)
.with_cargo_profile_args(["--release", "--locked"])
.with_inherited_env(["COUNTER_SCHEMA_MODE"])
.with_additional_inputs(["config/counter-schema.json"])
.with_prune_policy(
    ArtifactCachePrunePolicy::new()
        .with_max_age(Duration::from_secs(14 * 24 * 60 * 60))
        .with_max_size_bytes(10 * 1024 * 1024 * 1024),
);

let outcome = build_wasm_canisters_cached(&spec)?;
eprintln!("immutable cache: {}", outcome.record().exact_cache_path().display());
match &outcome {
    WasmBuildOutcome::Built(record) => {
        eprintln!("built {} in {:?}", record.fingerprint(), record.timings().total());
    }
    WasmBuildOutcome::Reused(record) => {
        eprintln!("reused {}", record.fingerprint());
    }
}

// Keep the outcome alive while reading its retained, read-only artifact.
let wasm = std::fs::read(&outcome.record().artifacts()[0])?;

if let Some(pruned) = outcome
    .record()
    .maintenance()
    .and_then(|maintenance| maintenance.prune_report())
{
    eprintln!(
        "removed {} builds ({} bytes)",
        pruned.entries_removed(),
        pruned.bytes_removed(),
    );
}
# let _ = wasm;
# Ok::<(), Box<dyn std::error::Error>>(())
```

The fingerprint covers the selected package set and resolved dependency nodes,
enabled features, exact external source/checksum/revision identities, local
dependency sources, relevant inherited package fields, workspace profiles,
resolver and lint settings, Cargo configuration, Rust toolchain files, target
and profile arguments, Cargo and rustc identities, explicit child environment,
selected inherited environment, and caller-declared additional inputs. This
validated semantic workspace projection lets an unrelated host-only workspace
dependency or lockfile change retain the same exact Wasm key.

For a Cargo shim that selects another compiler, pair `with_cargo_program(shim)`
with `with_rustc_program(actual_compiler)` so the `-vV` probe fingerprints that
compiler. The latter selects identity only; it does not configure the shim's
build. When Cargo should use a selected compiler, use
`with_extra_env([("RUSTC", actual_compiler)])`, which also takes precedence for
the identity probe. Keep the selected compiler consistent with the build command.
The [compiled post-link recipe](../crates/ic-build/src/artifacts/mod.rs) combines
this spec with `ArtifactCacheSpec::with_cargo_build_inputs`, declared input/tool
bytes and bounded optimizer execution. It retains the source acquisition through
commit and reads the retained transformed output. The
[external transform example](../crates/ic-build/examples/transactional_artifact_cache.rs)
also works for inputs supplied independently of a Wasm build.

The complete workspace manifest and lockfile remain part of a separate,
conservative validation digest. They are rehashed around builds and attached
external artifact transactions, so any raw workspace mutation during an
acquisition is rejected even when its semantic projection is unchanged.
`ResolvedCargoBuildInputs` exposes both `input_digest` (semantic cache identity)
and `validation_digest` (mutation guard). Projection falls back to the complete
manifest/lockfile identity when a selected local package is rooted at or cannot
be normalized beneath the workspace root. Cargo configuration discovery follows
the files Cargo can read from the invocation directory and its ancestors plus
the effective Cargo home. The extensionless `.cargo/config` takes precedence
when both supported names exist, and recursive required or optional `include`
entries are exact inputs. Build scripts that read application-specific
environment variables or files outside Cargo's package graph must still declare
them on the spec. Package inputs are content-hashed exactly but conservatively;
exact hashing does not promise that every file in the package closure is
semantically relevant to a build.

Calls sharing a target directory coordinate through one process lock. A cache
hit requires every expected nonempty Wasm artifact to carry the exact atomic
stamp for both the current fingerprint and artifact content. Exact builds are
retained under fingerprint-specific Cargo target directories, so a prior spec
can be materialized again after another spec used the caller-facing output.
`WasmBuildRecord::exact_cache_path` exposes the selected immutable directory;
CI cache collection does not need to reconstruct its private on-disk layout.
Both `WasmBuildRecord::artifacts()` and `ArtifactCacheRecord::artifacts()` return
read-only exact-cache paths retained by the record. Keep the outcome, batch
report, or a cloned record alive through post-link commit and final reading or
staging. A copied path or artifact descriptor alone does not retain ownership.
Configured compiler and post-link destinations remain mutable. Never transform
an exact artifact in place; write into the next transaction's staging outputs.
See the `artifacts` module documentation for a complete handoff example.

Pruning skips all retained entries, including those owned by another process.
The last record clone dropping or its process exiting releases retention; the
next maintenance pass can reclaim the entry. Cache bounds can be temporarily
exceeded while consumers hold records. Batch successes remain independently
retained when later entries fail, so keep their records instead of reducing a
report to indexes. Manual deletion or mutation of exact-cache files is outside
this contract.

Failed builds remove their incomplete fingerprint directory before returning;
if cleanup also fails, `WasmBuildError::FailedBuildCleanup` preserves both
failures. Every used target root receives a standards-compliant
`CACHEDIR.TAG`. Successful builds and cache hits atomically update a per-entry
last-use marker. `prune_wasm_build_cache` takes the build lock, removes entries
over the optional age limit, then removes least-recently-used entries until the
optional logical-byte limit is met. It only prunes direct fingerprint-named
children below `.ic-testkit/wasm-targets`; public artifacts and unrelated Cargo
target contents remain caller-owned. `WasmBuildSpec::with_prune_policy` performs
the same maintenance without reacquiring the lock, protects the active
fingerprint even when it exceeds the configured bound, and reports a nonfatal
`ArtifactCacheMaintenance` result plus its duration on the successful build
record. The standalone pruning function remains available when maintenance
failure should be returned directly.

High-frequency suites can use `with_prune_policy_at_most_every` to retain the
same limits while replacing repeated hit-path directory scans with a small
namespace marker check. The original `with_prune_policy` continues to run on
every successful acquisition.

`WasmBuildTimings::input_resolution` returns the structured Cargo/rustc
identity, Cargo metadata, input discovery, content hashing, and total timing.

Relative exact target paths use the caller's working directory, matching cache
filesystem operations. Relative shared targets use the workspace root. Select
these paths through `WasmBuildSpec`; overriding `CARGO_TARGET_DIR` through
`with_extra_env` or passing `--target-dir` through `with_cargo_profile_args`
returns `InvalidSpec` before acquisition.

The profile output directory must match the Cargo arguments: default builds and
`--profile dev` or `--profile test` use `debug`; `--release`, `-r`, and
`--profile bench` use `release`; custom profiles use their own name. A mismatch
returns `InvalidSpec` before input resolution or cache work. Setting the output
directory alone does not select a Cargo profile.

Wasm acquisition builds package libraries with Cargo's `--lib` flag by default.
Use `with_example(name)` to select one named `cdylib` example from exactly one
package. Metadata admits that target before acquisition; its name contributes to
the fingerprint and selects `<target>/<profile>/examples/<library_name>.wasm`,
with hyphens normalized to underscores in the filename. Default-library cache
identity and output layout remain unchanged.

Raw binary, example, test, bench, and `--all-targets` arguments return
`InvalidSpec` before input resolution or cache work. Target selection belongs to
the specification; other targets can overwrite the canister library's output.
An explicit `--lib` remains accepted only for the default-library selection.

Acquired packages must declare a `cdylib` library with Cargo's default library
name: package hyphens become underscores. That admitted name selects every cached
and published Wasm path; packages whose default names collide are refused.
Acquisition validates this declaration from Cargo metadata before reuse, compilation, or scheduled
shared-target maintenance. Libraries that only produce an `rlib`, renamed
libraries, and binary-only packages return `InvalidSpec`. Input-only snapshots
from `resolve_cargo_build_inputs` can still describe arbitrary Cargo targets
for transactional artifact recipes.

Suites can override the default workspace shared Cargo target while retaining
exact immutable final Wasm entries:

```rust,no_run
use ic_build::artifacts::{
    SharedIncrementalTargetPrunePolicy, WasmBuildSpec, build_wasm_canisters_cached,
    inspect_shared_incremental_target,
};

# let workspace = std::path::PathBuf::from(".");
let exact_cache = workspace.join("target/pic-wasm");
let shared_incremental = workspace.join("target/pic-wasm-incremental");
let spec = WasmBuildSpec::new(
    &workspace,
    &exact_cache,
    &["counter_canister"],
    "debug",
)
.with_shared_incremental_target(&shared_incremental)
.with_shared_incremental_target_maintenance_at_most_every(
    SharedIncrementalTargetPrunePolicy::new()
        .with_max_age(std::time::Duration::from_secs(7 * 24 * 60 * 60))
        .with_max_size_bytes(4 * 1024 * 1024 * 1024),
    std::time::Duration::from_secs(24 * 60 * 60),
);
let outcome = build_wasm_canisters_cached(&spec)?;
let shared_usage = inspect_shared_incremental_target(&spec)?
    .expect("configured acquisition creates the shared target");
let maintenance = outcome
    .record()
    .shared_incremental_maintenance()
    .expect("shared maintenance was configured");
eprintln!(
    "shared_bytes={} maintenance={} {}",
    shared_usage.logical_size_bytes(),
    maintenance,
    outcome,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Without integrated shared-target maintenance, an exact hit never acquires or
mutates the shared target. On a miss, callers using that target serialize
across processes; Cargo builds incrementally, then IC Build re-resolves
every exact input and publishes only the final Wasm files. A source race
rejects publication, while a failed build preserves the caller-owned
incremental state. Retention owns only immutable output entries, not the
shared Cargo target. `inspect_shared_incremental_target` takes the same
cross-process lock and reports its canonical path, logical size, last recorded
build use, and lock wait without removing caller-owned Cargo state.
`maintain_shared_incremental_target` is an explicit whole-target operation: if
an age or size threshold is exceeded it removes every other child while
retaining the root, cache tag, and live coordination lock. Callers must not
colocate unrelated data that needs to survive a clear. Before clearing, the
exact Cargo resolver rejects a target that overlaps source, configuration, or
additional inputs, or whose clearing could reach exact cached artifacts.
The exact and shared target may use the same container, whose `.ic-testkit`
metadata remains preserved. Independent high-frequency callers can use
`maintain_shared_incremental_target_at_most_every`; matching recent passes
coordinate through a small cross-process marker and skip both Cargo input
resolution and whole-target traversal. Missing targets remain uncreated,
policy changes are immediately due, and a zero interval always runs. When
retention always accompanies a Wasm acquisition, configure
`with_shared_incremental_target_maintenance_at_most_every` instead. It creates
and coordinates the target even on exact hits, reuses the acquisition's input
resolution for due maintenance, and reports the result through the build
record and structured progress events. Integrated maintenance is strict by
default. Consumers that prefer a usable Wasm acquisition over successful
retention can supply an explicit best-effort configuration:

```rust,no_run
use ic_build::artifacts::{
    SharedIncrementalTargetMaintenanceConfig,
    SharedIncrementalTargetMaintenanceFailureMode,
    SharedIncrementalTargetPrunePolicy, WasmBuildSpec,
};
use std::time::Duration;

# let workspace = std::path::PathBuf::from(".");
# let target = workspace.join("target/pic-wasm");
let maintenance = SharedIncrementalTargetMaintenanceConfig::new(
    SharedIncrementalTargetPrunePolicy::new().with_max_size_bytes(4 * 1024 * 1024 * 1024),
    Duration::from_secs(24 * 60 * 60),
)
.with_failure_mode(SharedIncrementalTargetMaintenanceFailureMode::BestEffort);
let spec = WasmBuildSpec::new(
    &workspace,
    &target,
    &["counter_canister"],
    "debug",
)
.with_shared_incremental_target(target.join("shared"))
.with_shared_incremental_target_maintenance(maintenance);
# let _ = spec;
```

A best-effort failure is retained as
`SharedIncrementalTargetMaintenanceOutcome::Failed` and does not record a
successful schedule marker. Spec validation, Cargo input resolution, builds,
source-race detection, and artifact publication remain strict. Configuration
getters support policy assertions without performing work.

Long input resolution, lock waits, maintenance, Cargo builds, and artifact
publication can expose synchronous structured progress without changing the
existing silent API:

```rust,no_run
use ic_build::artifacts::{
    WasmBuildProgressConfig, WasmBuildProgressEvent, WasmBuildSpec,
    build_wasm_canisters_cached_with_progress,
};
use std::time::Duration;

# let workspace = std::path::PathBuf::from(".");
let spec = WasmBuildSpec::new(
    &workspace,
    &workspace.join("target/pic-wasm"),
    &["counter_canister"],
    "debug",
);
let outcome = build_wasm_canisters_cached_with_progress(
    &spec,
    WasmBuildProgressConfig::new()
        .with_heartbeat_interval(Duration::from_secs(15))
        .with_cargo_output(false),
    |event| {
        if let WasmBuildProgressEvent::Heartbeat { phase, elapsed } = event {
            eprintln!("{phase} is still active after {elapsed:?}");
        }
    },
)?;
eprintln!("{outcome}");
# Ok::<(), Box<dyn std::error::Error>>(())
```

Observers run synchronously on the build thread and should return promptly.
Raw output events preserve non-UTF-8 bytes; output is still captured in
`WasmBuildError` when forwarding is disabled. An observer panic joins active
phase work, terminates and waits for a running Cargo build, and then performs
normal lock and incomplete-entry cleanup.

Suites that require standalone feature resolution for several variants can
batch independent specs without changing Cargo feature semantics:

```rust,no_run
use ic_build::artifacts::{
    LabeledWasmBuildSpec, WasmBuildSpec, build_wasm_canisters_cached_batch,
};

# let workspace = std::path::PathBuf::from(".");
# let target = workspace.join("target/pic-wasm");
let specs = [
    LabeledWasmBuildSpec::new(
        "role-a",
        WasmBuildSpec::new(&workspace, &target, &["role_a"], "debug"),
    ),
    LabeledWasmBuildSpec::new(
        "role-b",
        WasmBuildSpec::new(&workspace, &target, &["role_b"], "debug"),
    ),
];
let batch = build_wasm_canisters_cached_batch(&specs)?;
eprintln!("{batch}");
for failure in batch.failures() {
    eprintln!(
        "build {} ({}) failed in {:?} after {:?}: {}",
        failure.label(),
        failure.index(),
        failure.phase(),
        failure.entry_elapsed(),
        failure.error(),
    );
    eprintln!("partial timings: {:?}", failure.timings());
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

The batch is sequential and collect-all: every independent failure is returned
in one report, but entries are not run simultaneously. Every
`LabeledWasmBuildSpec` label
must be nonempty and unique; invalid structure returns
`WasmBuildBatchContractError` before resolution, progress, or build work. Its
`WasmBuildBatchReport` retains canonical ordered entries and provides
structured labeled `outcomes` and `failures` iterators. Labels are reporting
identity only and never affect exact cache keys. Progress and integrated
maintenance entries carry the same label. Every entry uses the ordinary
single-spec implementation and its own exact identity, locks, policy, and
Cargo command; packages are never collapsed into one invocation that could
unify shared dependency features.
When several packages are intentionally built together with Cargo's normal
shared feature resolution, keep them in one multi-package `WasmBuildSpec`;
splitting that build into per-package batch entries would repeat Cargo work and
may change feature resolution.

Resolution-compatible entries share one workspace/toolchain snapshot: Cargo
and rustc identity, Cargo metadata, input discovery, and memoized per-root
content digests are reused while each spec retains its standalone fingerprint
and Cargo invocation. A different workspace, tool environment, or
metadata-affecting feature argument starts a separate snapshot so independent
feature semantics remain intact.

`WasmBuildBatchReport::metrics` aggregates built, exact-cache reused, and failed
counts; input-resolution runs and compatible-snapshot reuses; and summed
successful acquisition timings. The structured input-resolution total makes
distinct feature-resolution costs visible while reused snapshots remain
counted explicitly, including reuse from an explicit session or prepared
concurrent snapshot. `entry_elapsed` retains wall time for every ordered entry,
including failures; `failures` returns structured entries that bundle the
label, index, error, primary failed phase, partial phase timings, and elapsed
time. Successful entries continue to expose detailed phase timings through
their build records.

### Source immutability for sessions

Repeated batches can reuse exact input snapshots when the caller owns a real
write-exclusion boundary for every build input:

```rust,no_run
use ic_build::artifacts::{
    LabeledWasmBuildSpec, WasmBuildBatchConfig, WasmBuildSession,
};
use std::sync::Mutex;

# let specs: Vec<LabeledWasmBuildSpec> = Vec::new();
// Every source/config/tool/environment writer must coordinate on this lock.
let source_write_exclusion = Mutex::new(());
let source_guard = source_write_exclusion.lock().expect("acquire source lease");
let mut session = WasmBuildSession::assume_sources_immutable(&source_guard);
let first = session.build_batch(&specs, WasmBuildBatchConfig::new())?;
let second = session.build_batch(&specs, WasmBuildBatchConfig::new())?;
eprintln!("first={first}; second={second}; session={:?}", session.metrics());
# Ok::<(), Box<dyn std::error::Error>>(())
```

The `assume_sources_immutable` name is an explicit caller assertion. The
borrowed guard is only a lifetime boundary, not filesystem locking or guard
provenance validation performed by IC Build; supplying an unrelated value
violates the contract and can reuse stale inputs. The lease covers Cargo and
rustc executables, manifests, Cargo configuration, discovered sources,
declared additional inputs, and relevant environment values. A detected input
race permanently invalidates the session, clears pending snapshots, and makes
later calls return
`SourceLeaseInvalidated`. Ordinary batch functions still resolve current inputs
per call, and there is no silent process-global cache. This guide and
[IC Build's public session documentation](../crates/ic-build/src/artifacts/wasm_batch.rs)
describe the supported contract. The earlier Testkit design is preserved below
as a historical reference.

When independent callers need the same resolution concurrently, prepare their
complete exact specification set before starting readers:

```rust,no_run
use ic_build::artifacts::{
    LabeledWasmBuildSpec, WasmBuildBatchConfig, WasmBuildInputSnapshot,
    WasmBuildSpec,
};
use std::sync::Mutex;

# let local_test: Vec<LabeledWasmBuildSpec> = Vec::new();
# let production: Vec<LabeledWasmBuildSpec> = Vec::new();
let prepared_specs = local_test
    .iter()
    .chain(&production)
    .map(|entry| entry.spec().clone())
    .collect::<Vec<WasmBuildSpec>>();
// Every source/config/tool/environment writer must coordinate on this lock.
let source_write_exclusion = Mutex::new(());
let source_guard = source_write_exclusion.lock().expect("acquire source lease");
let snapshot = WasmBuildInputSnapshot::prepare_assuming_sources_immutable(
    &source_guard,
    &prepared_specs,
)?;
let (local_report, production_report) = std::thread::scope(|scope| {
    let local = scope.spawn(|| {
        snapshot.build_batch(&local_test, WasmBuildBatchConfig::new())
    });
    let production = scope.spawn(|| {
        snapshot.build_batch(&production, WasmBuildBatchConfig::new())
    });
    (
        local.join().expect("join LocalTest reader"),
        production.join().expect("join Production reader"),
    )
});
local_report?;
production_report?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Preparation is closed: a reader requesting an exact spec absent from
`prepared_specs` receives `SpecificationNotPrepared` before progress or build
work; it never extends the snapshot or hashes a new path. A detected post-build
source race invalidates the snapshot for every later reader. Publication takes
a shared read boundary against invalidation, so a publication already at that
boundary completes before invalidation while every later publication is
rejected. `WasmBuildInputSnapshot::metrics` exposes preparation work,
cumulative reader reuse, and invalidation; batch metrics count
`input_resolution_prepared_reuses` separately.

Each batch remains sequential. Separate snapshot readers may overlap, but
shared incremental Cargo targets still participate in their existing locks;
use isolated targets to retain useful Cargo parallelism. There is no built-in
parallel scheduler, ambient cache, or mutation-owning workspace abstraction.
Callers without a genuine source write-exclusion guard must keep ordinary
per-call resolution.

When independent specs share incremental targets, batch-owned maintenance
removes the caller convention of modifying the first spec:

```rust,no_run
use ic_build::artifacts::{
    LabeledWasmBuildSpec,
    SharedIncrementalTargetMaintenanceConfig,
    SharedIncrementalTargetMaintenanceFailureMode,
    SharedIncrementalTargetPrunePolicy, WasmBuildBatchConfig,
    build_wasm_canisters_cached_batch_with_config,
};
use std::time::Duration;

# let specs: Vec<LabeledWasmBuildSpec> = Vec::new();
let maintenance = SharedIncrementalTargetMaintenanceConfig::new(
    SharedIncrementalTargetPrunePolicy::new().with_max_size_bytes(4 * 1024 * 1024 * 1024),
    Duration::from_secs(24 * 60 * 60),
)
.with_failure_mode(SharedIncrementalTargetMaintenanceFailureMode::BestEffort);
let batch = build_wasm_canisters_cached_batch_with_config(
    &specs,
    WasmBuildBatchConfig::new().with_shared_incremental_target_maintenance(maintenance),
)?;
for entry in batch.shared_incremental_maintenance_outcomes() {
    eprintln!("build {} ({}): {}", entry.label(), entry.index(), entry.outcome());
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

Maintenance is attached to the first specification for each distinct resolved
shared-target directory. Relative paths are workspace-relative, and filesystem
aliases of the same directory share one maintenance attempt. Isolated specs
are unaffected. A specification
that also configures per-spec integrated maintenance receives a labeled,
indexed error, and later entries still run.

`ResolvedCargoBuildInputs::is_current` resolves the semantic fingerprint again,
while `is_content_current` cheaply rehashes the conservative validation paths.
The generic builders preserve dynamic and non-UTF-8 argument and environment
bytes. Add direct dependencies on the appropriate Host crates for raw artifact,
filesystem, process and IC helpers. IC Build does not re-export them.
For Unix tool selection, use
`ic_host_process::tool::resolve_executable(requested, current_dir, search_directories)`
with an absolute working directory and explicit search order before declaring
the selected path through `ArtifactCacheSpec::with_tool`. Resolution reads no
ambient `PATH`; filesystem errors stop search. It selects a path without admitting
it for execution. `ic_host_process::tool::AdmittedTool` adds caller-supplied digest
and exact version admission, bounded output, an explicit environment and a deadline
for external transforms. Keep the executable and its parent directories protected
from concurrent writers during admission and execution.

Read compiled Wasm through the shared filesystem owner:

```rust
use ic_build::artifacts::wasm_path;
use ic_host_fs::read::read_file;

let path = wasm_path(target_dir, crate_name, profile_target_dir);
let bytes = read_file(&path, max_bytes)?;
```

The caller chooses the byte limit; failures are typed rather than panics.
Paths must be caller-controlled; symbolic links are allowed. Reading bytes does
not validate Wasm; use `ic_host_artifacts::wasm::inspect` for structural facts.
Stream identities, archives and Wasm inspection belong to `ic_host_artifacts`;
pathname reads and durable publication belong to `ic_host_fs`; executable
resolution, execution and Git provenance belong to `ic_host_process`; Candid
extraction and response decoding belong to `ic_host_tools`. Its pure Candid text
APIs are portable; extraction, execution, resolution and provenance are Unix-only.
Structural Wasm inspection does not replace PocketIC's runtime validation.
The IC Build 0.2 family selects IC Host 0.17. Align direct Host dependencies
when passing `OutputLimits` or matching typed Host failures across the engine
boundary. This crate identity changed at IC Build 0.2.0; an `ic-build = "0.1"`
requirement does not select it. Process execution returns completed
results even for nonzero exits: call `require_success()` before admitting or
publishing producer output, and return `io::Result<()>` from output observers.
Durable writers retain typed publication phase and cleanup evidence. Gzip
encoding accepts numeric levels 0–9. Testkit's earlier Host/PocketIC migration
records remain historical references below; PocketIC startup remains Testkit-owned.

The `transactional_artifact_cache` example uses shared digest/version admission
and bounded execution for its transformer. Supply a reviewed raw executable
SHA-256 and the exact trimmed stdout of its `--version` command:

```bash
cargo run -p ic-build --locked --offline --example transactional_artifact_cache -- \
  /path/to/transformer "$TOOL_SHA256" 'transformer 1.0.0' \
  input.wasm output.wasm target/transform-cache
```

It runs with an empty environment, a 60-second deadline and 64 KiB per output
stream, admitting executables up to 256 MiB. A failed admission or execution
does not commit output. The operator owns interpreter/library trust and any
effects the transform has beyond its declared output.

External transforms derived from a Cargo package closure can pass both the
build spec and its resolved snapshot to
`ArtifactCacheSpec::with_cargo_build_inputs`. The semantic Cargo fingerprint
becomes transactional identity, while the conservative Cargo-aware input paths
and generated-state exclusions are revalidated during preparation, hit
materialization, and commit. This avoids duplicating Cargo discovery as broad
workspace scans while still rejecting source, workspace, or toolchain races.

External deterministic tools use the transactional artifact-set cache. The
caller declares exact inputs, tool bytes, arguments, relevant environment, and
the complete output schema; IC Build owns cross-process locking, staging,
before/after input verification, validation, publication, materialization, and
retention:

```rust,no_run
use ic_build::artifacts::{
    ArtifactCacheOutcome, ArtifactCachePreparation, ArtifactCachePrunePolicy,
    ArtifactCacheSpec, prepare_artifact_cache,
};
use std::{io, process::Command, time::Duration};

# let workspace = std::path::PathBuf::from(".");
let input = workspace.join("target/wasm/counter.wasm");
let optimizer = workspace.join("tools/wasm-optimizer");
let destination = workspace.join("target/optimized/counter.wasm");
let spec = ArtifactCacheSpec::new(
    &workspace.join("target/external-artifact-cache"),
    "counter-optimizer",
    "counter/optimizer/v1",
)
.with_input("counter.wasm", &input)
.with_tool("optimizer", &optimizer)
.with_arguments(["--optimize-for-size"])
.with_environment([("OPTIMIZER_MODE", "deterministic")])
.with_output("optimized.wasm", &destination)
.with_prune_policy_at_most_every(
    ArtifactCachePrunePolicy::new()
        .with_max_age(Duration::from_secs(14 * 24 * 60 * 60))
        .with_max_size_bytes(2 * 1024 * 1024 * 1024),
    Duration::from_secs(60 * 60),
);

let outcome = match prepare_artifact_cache(&spec)? {
    ArtifactCachePreparation::Reused(record) => ArtifactCacheOutcome::Reused(record),
    ArtifactCachePreparation::Build(transaction) => {
        let staged = transaction.output_path("optimized.wasm")?;
        let status = Command::new(&optimizer)
            .args(["--optimize-for-size", "-o"])
            .arg(staged)
            .arg(&input)
            .env("OPTIMIZER_MODE", "deterministic")
            .status()?;
        if !status.success() {
            return Err(io::Error::other(format!("optimizer failed with {status}")).into());
        }
        transaction.commit()?
    }
};
eprintln!("artifact key: {}", outcome.record().key());
// Retain the outcome until reading or staging the exact output is complete.
let optimized_wasm = std::fs::read(outcome.record().artifacts()[0].path())?;
# let _ = optimized_wasm;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Multiple independent external recipes can use
`build_artifact_caches_batch`. Every `ArtifactCacheSpec` is wrapped in a
`LabeledArtifactCacheSpec`; labels must be nonempty and unique and are retained
in callbacks and ordered report entries. Labels are composition metadata, not
cache identity. A structural label error rejects the batch before work starts.

The spec describes command identity; it does not execute a tool or apply its
declared arguments and environment to `Command`. The caller's build callback
must run the declared recipe, as in the example above.

The callback receives only cache misses and only one live transaction at a
time, avoiding self-deadlock when specs share a coordination scope. Callback
failures synchronously abort the current staging directory, retain their label,
and do not stop later independent entries. `ArtifactCacheBatchReport::metrics`
aggregates built, reused, and failed counts plus all successful acquisition
timings. Every entry retains wall time; failed entries additionally distinguish
preparation, callback, explicit abort cleanup, and commit time. Commit timing
includes commit-owned failure cleanup. The operation is deliberately not atomic
across specs: use one `ArtifactCacheSpec` with several outputs when all
artifacts must publish as one transaction.

A miss transaction exposes checked staging paths for redirectable tools and an
`import_output` helper for commands that write to fixed locations. `commit`
accepts only the complete declared output set and publishes its manifest last;
dropped, failed, or panicked transactions remove staging without creating a
cache hit. Multi-output recipes declare several logical outputs and commit them
as one unit. Recipe identity is caller-owned and must change when undeclared
pipeline semantics change. Namespace and recipe identifiers are hashed before
being used on disk, environment values are never rendered in `Debug` or cache
manifests, and public destination paths do not affect content identity.
Prefer declaring pipeline implementation files, configuration, and wrappers as
ordinary exact inputs. Reserve manual `recipe_id` bumps for semantic changes
that cannot be represented by stable input bytes.

Declared input and tool roots may contain the cache directory, which is
excluded while recursively hashing that broader tree, but they must not be
located inside the cache themselves. Public output destinations must also stay
outside the cache, resolve to distinct non-nested paths, and not overlap a
declared input
or tool. These checks prevent silently unkeyed inputs, self-invalidating
recipes, and two logical outputs overwriting the same file.

The default coordination scope is the namespace. Recipes that use the same
mutable external work tree can select a shared scope even when their content
keys differ. Independent recipes can use distinct scopes. Exact content-key
locking still ensures that overlapping callers build one result only.

Artifact contents are hashed, imported, and materialized with bounded-memory
streaming. Configured or explicit retention also removes staging abandoned by
terminated processes when the corresponding content-key lock proves that no
transaction is active. `ArtifactCachePruneReport` reports those uncommitted
directories separately from committed entry age/size eviction.

`ArtifactCacheSpec::with_prune_policy_at_most_every` applies the same explicit
limits but scans no more frequently than the selected interval. A skipped pass
has no maintenance outcome; its inexpensive marker-check duration remains in
the timing record.

No retention limit is automatic. As a starting point for developer caches,
the examples use a 14-day age bound with a size appropriate to the entry type
(larger for isolated Cargo targets, smaller for final artifact sets). CI should
choose an explicit limit that fits its cache quota and expected fingerprint
count rather than inheriting a machine-wide default.

`WatchedInputSnapshot` likewise hashes file paths and contents instead of
modification times. Generated artifacts become reusable only after
`mark_artifact_fresh` atomically records the snapshot beside the successfully
built artifact; an unstamped artifact is conservatively stale. This lightweight
stamp records input identity only: it does not hash the output bytes, acquire
producer locks, or retain artifact paths against replacement. The caller must
coordinate the build and verify inputs stayed unchanged before stamping. Use
`ArtifactCacheSpec` when a pipeline needs transactional publication, output
content validation, and retained consumption.

The cache intentionally contains no Binaryen, `icp build`, package-name, or
PocketIC policy. This guide and [IC Build's public cache documentation](../crates/ic-build/src/artifacts/transaction.rs)
describe the supported transactional contract. Shared-target maintenance and
session reuse likewise remain IC Build contracts with caller-owned policy.

## Historical extraction references

The following Testkit records explain the pre-extraction design. They are bound
to extraction source `b906b1a44f8f5f0f2f23d2413a76103d99e4fbb3`; their version
numbers refer to Testkit, and their historical contracts do not supersede this
guide or IC Build's public API documentation.

- [0.5 transactional artifact-set design](https://github.com/dragginzgame/ic-testkit/blob/b906b1a44f8f5f0f2f23d2413a76103d99e4fbb3/docs/design/0.5-artifact-transactions/0.5-design.md).
- [0.6 shared-incremental Wasm design](https://github.com/dragginzgame/ic-testkit/blob/b906b1a44f8f5f0f2f23d2413a76103d99e4fbb3/docs/design/0.6-shared-incremental-wasm/0.6-design.md).
- [0.7 artifact orchestration design](https://github.com/dragginzgame/ic-testkit/blob/b906b1a44f8f5f0f2f23d2413a76103d99e4fbb3/docs/design/0.7-artifact-orchestration/0.7-design.md).

The [Host 0.14 migration guide](https://github.com/dragginzgame/ic-testkit/blob/b906b1a44f8f5f0f2f23d2413a76103d99e4fbb3/crates/ic-testkit/CHANGELOG.md#0340) covers the
completed-result and failure types. Descriptor-based locking replaced the pathname
progress-lock helper in the [Host 0.13 upgrade](https://github.com/dragginzgame/ic-testkit/blob/b906b1a44f8f5f0f2f23d2413a76103d99e4fbb3/crates/ic-testkit/CHANGELOG.md#0334).
Durable writers retain typed publication phase and cleanup evidence; the
[0.28 migration guide](https://github.com/dragginzgame/ic-testkit/blob/b906b1a44f8f5f0f2f23d2413a76103d99e4fbb3/crates/ic-testkit/CHANGELOG.md#0280) covers those earlier
writer changes.
Gzip encoding accepts numeric levels 0–9. When upgrading from 0.21, update Wasm
fact literals and exhaustive inspection
error matches as described in the [0.22 migration guide](https://github.com/dragginzgame/ic-testkit/blob/b906b1a44f8f5f0f2f23d2413a76103d99e4fbb3/crates/ic-testkit/CHANGELOG.md#0220).
The [0.23 migration guide](https://github.com/dragginzgame/ic-testkit/blob/b906b1a44f8f5f0f2f23d2413a76103d99e4fbb3/crates/ic-testkit/CHANGELOG.md#0230) covers Host's
new `ExecutionError::group_error` and the expanded PocketIC startup errors.
Startup errors retain their original cause and append secondary cleanup failures
when displayed; builder failures also retain bounded server stdout/stderr.
The [0.24 migration guide](https://github.com/dragginzgame/ic-testkit/blob/b906b1a44f8f5f0f2f23d2413a76103d99e4fbb3/crates/ic-testkit/CHANGELOG.md#0240) covers Host's
changed execution failure categories and replacing `read_wasm` with `read_file`.

