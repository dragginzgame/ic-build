# Current handoff

## Published 0.2.0 adoption and pending 0.2.1 tooling — 2026-10-13

Version 0.2.0 is published from
`9ddcd4ee0b2ac85f0608b8aaf49493cf015f295a`; archive SHA-256 is
`fb055d1b5649636c09a52f23fdc091417c732cbb87a2125d5a24f93c043c8df8`.
Its [release CI](https://github.com/dragginzgame/ic-build/actions/runs/38389784699)
passes Linux and lint/security, including fresh release-editor setup. Both macOS
jobs remain queued at inspection; earlier cancelled runs do not qualify them.

The authorized continuation adopts the published registry line in IcyDB and
Testkit. Their unused Host 0.16 artifact/fs/process packages leave the lockfiles;
all other selected registry versions/checksums remain unchanged. Canic's owner
had already selected 0.2.0. No real consumer carries a producer path override.
Focused registry qualification passes strict Canic Host all-target/all-feature
Clippy and 45 artifact/output/binding/declaration cases, including the extractor;
one governed PocketIC binding case stays ignored. IcyDB passes selected Clippy,
nine artifact/Candid and five cache cases, including real Wasm flag identity;
the ordinary CLI remains parser-free. Testkit passes library/driver Clippy,
17 driver, six diagnostics and six pure pool tests, plus actual Rust 1.88.
Receipts and consumer before-images are under
`/tmp/ic-build-020-registry-adoption/`. Incoming consumer work is preserved.

Pending compatible 0.2.1 adopts [Shared 0.10](https://github.com/dragginzgame/ic-build/issues/10)
at `ed9c26f015691859f67b50b512a94709dcdc0c6c`: 70 roots, 88 canonical files.
Twenty-seven payloads update, and the newly referenced reporting rule is added
as a canonical entrypoint. Initial link validation caught that omitted rule;
the retained failed log precedes the complete, verified closure. Common tool
versions/digests match; no exception or alternate installation owner is added.
Release-editor selection remains consumer-owned. Explicit setup/offline admission
and full Linux `make ci lint check-secrets` pass with the incoming Host 0.17.1
lock graph, including actual Rust 1.88. Final receipt:
`/tmp/ic-build-020-registry-adoption/shared010-final-validation.log`.
README registry guidance and the local baseline identify the current owners.
KEEP-only repeat refresh and integrity pass. Canonical Make, formatting,
freshness, host/IC store and actual evidence-collector fixtures pass on Bash 5
and genuine Bash 3.2. Corruption cases use only synthetic owned installations;
their receipts are the `shared010-final-owner-*` and `shared010-host-ic-*` logs.

Native/application acceptance remains open on the existing issues; the larger
[Canic #530](https://github.com/dragginzgame/canic/issues/530) engine removal
still requires those prerequisites. Toko's migration remains on
[#1796](https://github.com/dragginzgame/toko/issues/1796). Own metadata stays
0.2.0, IcyDB's compatible draft stays 0.273.1, and Testkit's stays 0.37.1.
No staging, commit, push, version change, release, publication or CI action ran
in this continuation. Historical sections retain their original graph scope.

## 0.2.0 release setup and consumer qualification — 2026-10-13

The authorized continuation fixes [#11](https://github.com/dragginzgame/ic-build/issues/11):
`make install-tools` explicitly prepares consumer-pinned cargo-edit 0.13.13
through the canonical shared selected-tool installer. Offline checks and release
metadata use its admitted absolute executable. Preflight checks both pin files
for input changes; failed adapter fixtures print bounded inner diagnostics and
retain complete logs. The existing Host 0.17 and LTO work remains intact.

Linux `make ci lint check-secrets` passes, including 146 units, real Cargo/Wasm
and process contracts, strict Clippy and actual Rust 1.88. Receipts:
`/tmp/ic-build-020-11-validation.log` and the final shell/documentation/adapter
rerun `/tmp/ic-build-020-11-final-focused.log`. Private Cargo-home/no-global-plugin
adapters pass under `/tmp/ic-build-11-no-global.Um1McV/`. Missing editor selection
refuses offline; explicit setup and adapters pass under
`/tmp/ic-build-11-fresh-selection.COoG9r/`. An injected admission fault prints
the retained inner log. The first fresh harness omitted common-tool link targets;
that harness failure is retained separately from the corrected proof.

Frozen candidate qualification lives under `/tmp/ic-build-020-consumers.GxZq30/`.
Only those disposable producers/manifests select 0.2.0 and local overrides.
Current Canic `0d6779da5d9f1f7e92ef532ef32663f28fff4677` passes strict Host
all-target/all-feature Clippy and 45 artifact/output/binding/declaration checks,
including the real extractor; one governed PocketIC binding case stays ignored.
Its original Blob 0.23.1 selection is preserved after explicit exact cache setup.
Testkit's existing dirty retention work passes driver Clippy and all 17 tests;
initial temporary proofs lacked nested Cargo override/Git provenance and are
retained as fixture failures. IcyDB's corrected `candid-service` integration
profile passes strict selected Clippy and nine artifact/Candid tests on both its
registry graph and the candidate; its ordinary CLI remains parser-free.

Real consumer catalogs remain registry-based. Publish 0.2.0 before adopting its
requirement in Canic, IcyDB and Testkit; Canic's normal Host boundary still needs
that release. Toko's existing [#1796](https://github.com/dragginzgame/toko/issues/1796)
now names direct IC Build/current Testkit ownership. The larger Canic #530
removal remains behind native/application acceptance. Native macOS is unqualified;
0.1.4 CI still has the recorded Linux failure and queued macOS jobs. Package
metadata remains 0.1.4. No live Git, version, release, publication or CI effects.

## Host 0.17 public-boundary alignment — 2026-10-13

The maintainer authorized this sibling change to unblock Canic. IC Build now
selects Host artifacts/fs/process 0.17 together. No Rust API implementation or
cache-format change was needed. Its public grouping `OutputLimits` and typed
Host failures now belong to 0.17, so the pending release is correctly classified
as breaking 0.2.0. Package metadata remains 0.1.4; publication is maintainer-owned.

Linux delivery evidence passes across `/tmp/ic-build-host017-validation.log`
and `/tmp/ic-build-host017-followup.log`: compilation, strict Clippy, rustdoc,
146 unit tests, real Cargo/Wasm and process cases, actual Rust 1.88, release
commands/adapters, shell/workflow lint and secret scanning. The first adapter
fixture failed because it reused the live minor draft for a patch transaction;
it now authors its own disposable patch notes, and its focused rerun passes.
No live Git release effects ran. Native macOS remains unqualified for this graph.

Canic Host compiles against the frozen producer in
`/tmp/canic-host017-proof.oEVphD`, using an override only in that temporary copy.
Its strict all-feature library/test Clippy check also passes
(`/tmp/canic-host017-consumer-clippy.log`). The real Canic catalog/lock remain registry-based; publication and adoption are
still needed to clear its normal build. Concurrent LTO documentation was
preserved. No commit, package-version edit, push or publication was performed.

## Prepared deployable-Wasm guidance — 2026-10-13

Released HEAD is 0.1.4 at `848ae20548527f5595ea68e7edf9cfec4f3188ce`.
The undated 0.2.0 draft adds [LTO guidance](../artifacts.md#deployable-wasm-lto):
IC Timers' linked probes save 18.67–18.85% raw Wasm with full LTO. This motivates
complete-application qualification, including final transforms and instruction
budgets, before a common deployable-Wasm adapter adopts the selection. Existing
`with_extra_env` already makes an explicit override part of build identity.
Engine defaults, public API and Cargo inputs are unchanged. Documentation checks
are scoped separately from builds and product acceptance. Earlier sections retain
their original source and qualification identities.

## Pending 0.1.4 qualification batch — 2026-10-13

The maintainer requested more work for 0.1.4. The existing fresh-runner Wasm
setup correction is extended with [#8](https://github.com/dragginzgame/ic-build/issues/8):
warning-denied Rust Clippy now passes for the library, unit/integration tests and
examples with all features. `make clippy` is a primitive, offline/locked package
check; `make check` and `make ci` run it before rustdoc and tests. The development
toolchain declares both Clippy and the Wasm target for explicit setup.

The 43 initial library/test diagnostics are corrected, together with one source
guard diagnostic exposed when integration targets became reachable. Public
error/panic contracts are documented, private cache-miss inputs are borrowed,
and small test/style corrections preserve behavior. Source guards remain held
through the final session/snapshot operation; borrowed owners explicitly drop
before the guards. Two statement-local lint expectations document why descriptor
inheritance tests must inspect underscore-named RAII fields. The declared lint
policy, public production signatures, dependencies and cache bytes are unchanged.

Full Linux `make ci lint check-secrets` passes with the new gate, including 146
unit tests, 14 real Cargo/Wasm cases, process/retention suites, rustdoc, actual
Rust 1.88.0 and isolated release-adapter fixtures. Receipt:
`/tmp/ic-build-014-full-validation.log`; strict initial/final diagnostics and
before-images are retained under `/tmp/ic-build-014-*`. Cargo.lock remains
`c0c98783fc94d305034cef7788a7558a5629aacef8c8ca039d2a638ab5fcd235`.
Explicit setup in the retained private Rustup fixture also prepares Clippy while
keeping Wasm installed; receipts are `clippy-install.log` and
`clippy-components.log` under `/tmp/ic-build-6-fresh-rustup.FGmFTo/`.

Published 0.1.3 now has the same heartbeat failure on Linux and native macOS
Apple Silicon; Intel remains queued at inspection. The delivered 0.1.4 native
matrix still needs to qualify these working-tree corrections. Metadata remains
0.1.3, the numbered draft remains 0.1.4, and Git/release effects are user-owned.
No sibling source was changed in this batch. Earlier sections retain their
historical source/graph and qualification state.

## Published 0.1.3 and current consumer qualification — 2026-10-13

Version 0.1.3 is published from
`3248364dccc9bdb9b56a666f86cc7682b4dead42`. The cached crates.io archive's
SHA-256 is
`1527d2dfe308fc7c1236b6db42c8c6883635f0d60d1dbbabd28462648058caec`;
its VCS receipt names that release and its staging source/tests match this
checkout. The published dependency graph selects Host 0.16.0. Earlier Linux
receipts below qualified Host 0.15.2 and remain historical evidence.

The authorized Canic #529 replacement is applied against registry 0.1.3, with
no local producer override or parallel staging engine. Canic keeps its recipes,
read/output policy, release binding and qualification-first durable publication.
Its current graph selects Host 0.16.0, Memory 0.36.1, Testkit 0.36.0,
Blob/contracts 0.23.0 and Timers 0.21.4. Concurrent catalog updates are preserved.

All 45 focused consumer checks pass in a frozen checkout with a separate target:
23 artifact cases, two real Cargo-output cases, three release-binding cases and
17 declaration/cache cases including the real extractor. One governed PocketIC
binding case remains ignored. Strict Host all-target/all-feature Clippy and
changed-file formatting pass. A final custody check matches 1,592 owning-package
Rust/manifest/config inputs to live Canic; receipts are under
`/tmp/canic-529-registry-qualification/` and `/tmp/canic-529-registry-*.log`.
Three consumer production adapters shrink by 25 code lines. Canic's handoff and
0.112 draft own consumer evidence; native/application acceptance remains pending.
Package versions, staging, Git delivery and further publication remain user-owned.

The [release CI run](https://github.com/dragginzgame/ic-build/actions/runs/38373448100)
has a failed Linux job and queued macOS jobs at inspection. Linux again fails
the real positive-heartbeat build with a missing Wasm standard library, despite
the released prepared-compiler selection. Rustup 1.29.1's no-channel install
branch reads required targets from the toolchain file and does not apply the
command's target/component flags. The released file omitted the Wasm target.

The working-tree #6 follow-up declares `wasm32-unknown-unknown` there. A fresh
private Rustup home under `/tmp/ic-build-6-fresh-rustup.FGmFTo/` reproduces the
old setup's successful installation without Wasm, then the same explicit setup
installs Wasm with the amended file. Its real positive-heartbeat test passes
with that private compiler and no inherited toolchain override. The user's
installed compilers, global default and targets are untouched.

Full Linux `make ci lint check-secrets` passes on the released Host 0.16 graph,
including 146 units, 14 real Cargo/Wasm cases, process/retention suites and actual
Rust 1.88.0. Receipt: `/tmp/ic-build-013-host016-final-ci.log`. Metadata remains
0.1.3; the current numbered 0.1.4 draft covers this setup correction. Native
macOS acceptance and #6 closure still require delivered CI results. Historical
working-tree/version claims below describe their recorded source state.

## Historical compiler qualification

Version 0.1.2 is published from `24c5f49a1684e62f67a659a931a628ada6cd2cdd`;
package metadata remains 0.1.2 and the next pending notes are 0.1.3. The later
sections preserve prior batch receipts, including historical version and
working-tree references.

The working-tree repair for
[#6](https://github.com/dragginzgame/ic-build/issues/6) selects the prepared
compiler in the `make test` entrypoint and passes its absolute path as `RUSTC`
to the test command. This existing caller-environment contract covers both
identity probes and Cargo builds in all independent fixtures. Heartbeat
zero-interval rejection and the real positive-interval build remain unchanged;
the library API, cache formats and process implementation are unchanged.

The original heartbeat failure was reproduced locally with inherited compiler
selection removed. The ambient stable toolchain lacked the Wasm standard library,
while the repository-selected 1.99.0 toolchain had it. An initial fixture-only
fix exposed the same cause in 11 integration cases; the common entrypoint now
qualifies all 141 unit tests and all 14 real Cargo/Wasm cases with a direct
prepared Cargo executable and no inherited toolchain/compiler selection.
The focused heartbeat invocation also passes from that ambient environment
with the selected `RUSTC`. No global default or installed targets were changed.

The standard full Linux `make ci lint check-secrets` passed, including actual
Rust 1.88.0 and release-adapter fixtures. Evidence remains under
`/tmp/ic-build-6-ambient-qualification.puIRI8/`; `compiler-selection.log`,
`focused-after.log` and `full-validation.log` own the successful receipts.
`ambient-direct-cargo-gate.log` retains the adversarial suite pass and a
qualification-harness-only MSRV dispatch failure: direct Cargo cannot interpret
rustup's `+1.88.0` syntax. Normal MSRV dispatch subsequently passed.
The original failures remain in `/tmp/ic-build-6-ambient-before.log` and
`/tmp/ic-build-6-ambient-suite-before.log`.

The authorized Toko adoption finding was added to existing
[Toko #1796](https://github.com/dragginzgame/toko/issues/1796#issuecomment-6140005110)
without sibling file changes. Git delivery remains user-owned; this repair is
uncommitted and native macOS qualification requires CI against the delivered
change. Local Linux evidence does not establish native macOS acceptance.

The maintainer separately authorized implementing
[Canic #528](https://github.com/dragginzgame/canic/issues/528). Canic now delegates
declaration caching to its existing registry IC Build 0.1.1 transaction API;
private JSON persistence is removed in the same working-tree change. Its recipe
adapter retains optional-cache fallback, input/tool custody and bounded workers.
All 17 scoped extraction/cache tests, including real extractor byte parity, and
strict Host Clippy pass on Linux. Detailed custody and qualification live in
Canic's handoff and the issue; native macOS acceptance remains pending. This
consumer change needs no new IC Build API or dependency update and does not
expand the producer's pending 0.1.3 notes beyond the compiler-selection repair.

## Temporary staging producer and pending Canic adoption

Pending 0.1.3 now also prepares temporary artifact staging and producer-output
capture for [Canic #529](https://github.com/dragginzgame/canic/issues/529).
`ArtifactStagingDirectory` owns an invocation's temporary child and explicit
close/best-effort Drop cleanup. `CapturedArtifact` keeps exact copied producer
bytes alive across later source replacement, with a caller-selected read budget.
These APIs do not publish/reuse cache entries, select recipes, prune retained
artifacts or admit a release. No dependencies or package versions were added.

The full Linux `make ci lint check-secrets` passes on the maintainer's current
staged Host 0.15.2 lock graph, including 146 unit tests, the existing real
Cargo/Wasm/process suites and actual Rust 1.88.0. Its receipt is
`/tmp/ic-build-529-final-ci.log`; the five focused staging tests and updated
docs/format checks are retained separately. Optional strict Rust Clippy still
reports 43 distinct existing diagnostics. A disposable pre-staging baseline on
the same graph produces identical normalized diagnostics; the new module adds
none. [IC Build #8](https://github.com/dragginzgame/ic-build/issues/8) owns that
independent follow-up.

The Canic replacement is prepared as `/tmp/canic-529-adoption.patch` with its
source candidate and independent qualification checkout under
`/tmp/canic-529-candidate/` and `/tmp/canic-529-review/`. Its compiled-output,
artifact qualification and release-binding checks plus strict Host Clippy pass
in isolation (22 passing checks, with the governed PocketIC case still ignored).
That frozen consumer graph selects Host 0.15.1 and Memory 0.35.19; unrelated
live Canic catalog/lock updates are preserved and need qualification when the
patch is adopted. The producer is represented as 0.1.3 only in the disposable
qualification fixture; real metadata stays 0.1.2. This is candidate evidence,
not registry adoption or native macOS acceptance. The live Canic staging path
has not been removed yet: publish the new API, then update its registry
selection/lock and apply the qualified replacement and retirement together.
Canic retains its source-read policy, transform/output limits, release binding,
qualification and durable publication order. No live source override or dual
compatibility pipeline is introduced. User-owned staging and other dirty work
remain untouched; the new API and notes are unstaged.

## CI dependency preparation

Version 0.1.1 is published from `dbb17982557174eed29e1a499a2b1a7bb278d5a7`.
The next pending notes are 0.1.2; package metadata remains 0.1.1. The sections
below preserve earlier extraction/delivery evidence, including their historical
version and working-tree references.

The current working-tree repair for
[#5](https://github.com/dragginzgame/ic-build/issues/5) adds explicit
`cargo fetch --locked` preparation on every CI matrix host before the offline
gate. Fetch output is retained with the existing failed-validation evidence.
Developer setup now documents the same prerequisite; validation remains offline
and does not install dependencies implicitly.

A private checkout of the released source with the workflow/changelog repair and
an empty Cargo home reproduced the missing `fs2` failure before preparation.
Locked fetching then enabled the full Linux `make ci lint check-secrets` gate,
including 141 unit tests, real Cargo/Wasm/process suites, actual Rust 1.88.0 and
release-adapter fixtures. The released lockfile checksum stayed
`c1704c433e54f8bd20c054561570f6e7ed85cf8fbfa165093a6b55eb4429a51e`
before fetch, after fetch and after the gate. Evidence remains under
`/tmp/ic-build-ci-cache-repair.Vn3mPr/`, including `before-fetch.log`,
`dependencies-fetch.log` and `full-validation.log`.

The real checkout's existing Host 0.15.1/syn 3.0.9 lockfile update was preserved
unchanged and was not substituted for the released graph in this validation.
The repair remains uncommitted; no package version, real index, Git refs,
publication, CI dispatch or retained artifacts were changed. Native macOS
qualification still requires CI against the delivered repair; local Linux checks
do not establish it.

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
