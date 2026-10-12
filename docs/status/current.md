# Current handoff

## Generic engine extraction

The maintainer authorized moving generic build/artifact code from IC Testkit
and Canic into this repository. The extracted implementation is in
`crates/ic-build/`; Cargo metadata currently selects version `0.1.0` and permits
registry publication (`publish = true`). Version 0.1.0 is published and selected
from the registry by Testkit and IcyDB working trees. Its cached archive checksum
matches their tracked lock entries. Release/publication commands remain
maintainer-operated. The changelog preserves the original 0.1.0 scaffold notes as
historical evidence; the published archive also contains the extracted engine.
Pending 0.1.1 covers subsequent corrections, documentation and tooling work.
The Testkit namespace removal retains its separately documented minor boundary.
The maintainer selected 0.1.1 for these notes; Cargo metadata stays at 0.1.0.
[IC Build #1](https://github.com/dragginzgame/ic-build/issues/1) owns extraction
acceptance and follow-up. The [ownership design](../design/build-ownership.md)
is the architecture and historical audit evidence.

The engine owns Cargo input resolution, exact Wasm caches, transactional
artifact sets, retained records, sessions, batch outcomes, freshness and explicit
maintenance. Canic's feature-equivalent Cargo grouping also lives here; callers
supply the Cargo command/environment, target and output policy. The engine
requires neither IC Testkit nor Canic nor PocketIC.

Consumer migrations were applied as precise static patches against their current
working files after validation in isolated source checkouts and verification of
exact before-images and backups. Testkit removes its artifact module and
moves generic tests/examples to this owner, with direct IC Build imports in its
remaining fixture recipes. Canic migrates direct artifact
consumers and replaces its grouping implementation with a policy adapter.
There are no compatibility reexports or remaining Testkit artifact callers in
the authorized consumer Rust sources.

Testkit keeps PocketIC provisioning, lifecycle, diagnostics and benchmarks.
Canic keeps recipes, generated infrastructure, build environment, source custody,
credentials, release identities and complete-release admission. Its separate
production complete-release reuse/finalization path is retained: a generic cache
hit does not prove a valid Canic release. Converging that path requires preserving
its additional dependency, before/after, Candid and release invariants. IcyDB's
read-only working tree at `bf444afeb4634dacc809396dbdc8c10cc2e984f4` now selects
registry IC Build and imports its artifacts directly. Its owner is qualifying
that dirty migration; source inspection does not establish acceptance or delivery.

## Retained contracts and dependency custody

Existing `ic-testkit-wasm-build-v1`, `ic-testkit-watched-input-v1` and
`ic-testkit-artifact-set-v1` digest/stamp domains, bytes, `.ic-testkit` cache paths
and lock namespaces remain unchanged. No alternate reader, implicit migration or
artifact cleanup is introduced. Records retain artifact ownership through use;
failures, cancellation, input-change rejection, feature-safe batching and explicit
maintenance remain with the extracted engine. Builds have no finite default
elapsed-time deadline. The Testkit #66 lock-test isolation repair is preserved.

The virtual root workspace owns edition 2024, Rust floor 1.88.0, MIT licensing,
shared dependency selections and the lockfile. License metadata inherits SPDX
`MIT`; the package-local regular `LICENSE` preserves the repository's original
notice and is compatible with the shared formatting hook. The redundant
`license-file` field is removed to avoid Cargo's manifest warning.
IC Build selects Host 0.15.0,
matching the independently prepared Canic and
Testkit source boundaries. Consumer lockfiles retain existing dependency versions
while reconciling the explicit source inputs and relocated engine. Both consumer
graphs compile; retained transitive Host 0.14 dependencies do not own the engine.
Canic's incubation source paths and evidence are explicit dependency-pin exceptions;
Testkit now selects registry IC Build 0.1.0 with a tracked checksum.
No workspace package versions have been changed by this extraction. Testkit's
published 0.34.7 history and ongoing Host selection are preserved; the artifact
namespace removal is documented under pending 0.35.0 notes. Canic's existing
0.112.0 draft is extended. Consumer delivery requires the appropriate version
boundary and registry alignment; IC Build publication does not establish consumer
acceptance.

## Qualification and evidence

The extracted package passed `make ci` and `make lint check-secrets` on Linux
x86-64, including the shared snapshot, tool admission, documentation links,
manifest/pin inheritance, formatting, all-target compilation, rustdoc with
warnings denied, behavioral/process tests and compilation with actual Rust/Cargo
1.88.0. The initial extraction gate included 139 unit tests, cross-process cache locking,
retained-artifact handoff, helper coverage, 14 real Cargo/Wasm cases, shared-target
process coordination and a doctest. Ignored worker entrypoints execute through
the parent process tests; synthetic producer fixtures cover failure/race cases.
The real Wasm cases use independent Cargo fixture workspaces, not a silently
missing Testkit perf package. The artifact API guide is [here](../artifacts.md).

The current local issue-repair gate passed `make ci lint check-secrets`, with
141 unit tests and the existing Cargo/Wasm/process suites, plus actual Rust
1.88.0 compilation and release-adapter fixtures. Cargo grouping now returns
Host's original typed nonzero failure with status and bounded output, and rejects
duplicate/empty package inputs before invoking the command factory. Regression
checks cover truncation evidence, malformed forests, typed capture/spawn failures
and real Cargo feature-safe grouping. The README's direct registry/import example
also compiled through rustdoc. Current public documentation and the GitHub
description acknowledge published 0.1.0; its immutable registry README retains
the old text until a subsequent package is published. Evidence is retained in
`/tmp/ic-build-issues-full-validation.log`. These repairs are working-tree changes;
release identities remain unchanged. Native macOS and consumer
acceptance remain with the linked extraction issue.

Testkit all-target compilation passed in both the isolated review and live
workspace against its current 0.34.7 source and selected Host 0.15.0 graph. Its
18 benchmark-driver tests also passed in the live workspace, with six PocketIC
diagnostic unit checks passing in isolation. All-target compilation passed in
both isolation and the live workspace for Canic Host,
Testing Internal and Tests against the extracted engine and changed Testkit
producer. Nine focused Canic tests passed in the live workspace, exercising
retained-artifact handoff, cache timing and generated-input/build-policy custody
without starting PocketIC. Both consumers' dependency-pin and focused
source-format checks passed. Full consumer/PocketIC
suites remain maintainer-owned and were not run.

Review checkouts, exact live before-images, hashes and static consumer patches
are retained under `/tmp/ic-build-extraction-review.18ss90/`; focused logs are
`/tmp/ic-build-testkit-live-check.log`, `/tmp/ic-build-testkit-live-driver.log`,
`/tmp/ic-build-canic-live-check.log`, and `/tmp/ic-build-canic-live-targeted.log`.
The complete IC Build gate is recorded in `/tmp/ic-build-extraction-final-ci.log`.
Earlier engine test/MSRV logs and failed
validation evidence remain retained. These local paths are evidence custody,
not portable CI receipts.

CI is configured for Linux x86-64 and native macOS 15 Intel/Apple Silicon, with
explicit Wasm-target and declared-MSRV preparation. Native macOS extraction
qualification has not run; Linux does not substitute for it. Shared Tooling 0.9.5
is pinned at `b1630b89482cc6071fc928a86962d79c9c11fdb7` with exact snapshot
digests. All common tools and pinned Linux validation tools are prepared offline;
shared helpers have no consumer edits. The development compiler remains Rust 1.99.0.

The 0.9.5 adoption used a clean, detached source checkout and retained the 86-file
entrypoint closure. The sibling's dirty 0.9.6 work was excluded. Existing tool
pins, package metadata and extracted Rust sources were preserved. Snapshot/tool
admission, documentation links, dependency pins, formatting, release routing and
shell/workflow lint passed on Linux. Canonical checksum-refusal and release-runner
stub fixtures passed against the exact adopted helper bytes; curl's transient
recovery, exhaustion, permanent failure and checksum refusal passed through a
bounded loopback fixture. These fixtures perform no real release or tool install.
Evidence is retained in `/tmp/ic-build-shared-tooling-095.hFZahO/` and the
`/tmp/ic-build-shared-tooling-095-*.log` validation logs. The committed 0.1.0
scaffold changelog section is preserved verbatim beneath pending 0.1.1 notes.

`make check`/`make ci` include the actual package tests and isolated consumer
release-adapter checks. The bootstrap refusal is replaced by direct-delivery
metadata adapters using the shared runner, source checker, changelog finalizer
and local-lock writer. The snapshot now records 87 files. The metadata set is
exactly `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md`; package versions and
registry selections are checked across preparation and selected-commit recovery.
Publication requires clean HEAD at the annotated workspace-version tag and is
separate from release delivery. No adapter cleans retained artifacts. The
[release guide](../publishing.md) owns operating instructions; native macOS
qualification remains outstanding. This batch prepares adapters, not a release.
The full Linux `make ci lint check-secrets` passed with these adapters, including
the actual nested validation logger and disposable Git/index/tag fixtures.
The fixtures cover metadata rollback, unchanged external lock entries, saved
candidate replay, selected older-commit checks, publication refusal/failure and
retained artifacts. Logs are retained at `/tmp/ic-build-release-full-validation.log`
and `/tmp/ic-build-release-adapter-test.log`; failed fixture attempts remain in
their printed temporary directories. No live release or registry request ran.
Commit the adapter changes before invoking a normal release: preflight correctly
refuses implementation edits outside the three declared metadata files.
The public repository is established; the maintainer committed the earlier
package layout as `f4ae89c` and extraction/tooling as `d4f6222`. The maintainer
has now requested committing and pushing the validated current batch to
`origin/main`; associated issues record delivery identities. This delivery does
not change package versions, release tags, publication or retained artifacts.
GitHub issues are the follow-up tracker; this handoff records ownership and
qualification evidence.
