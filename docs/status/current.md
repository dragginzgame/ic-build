# Current handoff

## Published 0.2.6 and registry Canic qualification — 2026-10-14

Registry 0.2.6 names `8afa2d66c4d49a84154f1e1c1904dc033622b585`, checksum
`226c5e59d1226d272d5940155f8ed58abf4630fbd4cbd6bed83ca5143608e5a9`.
Its [exact CI](https://github.com/dragginzgame/ic-build/actions/runs/38530184826)
passes Linux x86-64, native macOS 15 Intel/Apple Silicon and lint/security.
[#15](https://github.com/dragginzgame/ic-build/issues/15) and
[#16](https://github.com/dragginzgame/ic-build/issues/16) close on those native
results. The selected-record API at #17 is published and natively qualified.

The local workspace now requires compatible Host 0.17.8 for artifacts/fs/process,
preserving features and the already-selected lock byte-for-byte. This prevents
older retained-staging source from satisfying the advertised minimum. Compatible
pending notes select 0.2.7; own metadata remains 0.2.6. Full Linux
`make ci lint check-secrets` passes with 164 units and actual Rust 1.88.

The concurrent Canic owner has applied the registry 0.2.6 requirement and exact
record adapter to its existing dirty 0.112 batch. This task preserves those
incoming files and qualifies a frozen current source with actual registry Build
0.2.6/Host 0.17.8, with no producer override or lock change. All 91 build-family
cases pass (three governed cases ignored), plus strict Host library/test Clippy
and actual Rust 1.91 all-feature library compilation.

Both dedicated and embedded managed Blob build-only workers complete four-artifact
builds, then verify exact release-ID reuse under matched launcher environments.
A new malformed foreign depfile was added to the shared output target during
the dedicated cold build and retained through both compositions. It does not
contaminate their selected input graphs. The first embedded repeat is retained
in the evidence: the harness changed launcher environment, and the cache
correctly rebuilt. Matched repetition then reused all four verified artifacts.
No PocketIC server or deployment ran. Evidence is under `/tmp/ic-build-canic-026/`.

The bounded producer #17 closes; Canic #544 retains current-source delivery and
complete application/native acceptance. Larger #1/#9/#12 and Canic #528/#529/#530
remain open. The explicit #542 release hold remains; Canic 0.112 is not delivered.
Host 0.17.8's unrelated Darwin FIFO-test compilation defect is reported on
[Host #85](https://github.com/dragginzgame/ic-host-tooling/issues/85); its full
fixture acceptance is distinct from the successful IC Build native gates.

## Selected-record input boundary and Canic handoff — 2026-10-14

Pending compatible 0.2.6 adds `read_cargo_depfile_inputs` at the existing input
owner ([#17](https://github.com/dragginzgame/ic-build/issues/17)). Exact-record
and directory discovery share bounded no-follow reads, strict parsing, alias
resolution and generated-root exclusion. Consumers choose target records and
cold-inventory absence; malformed selected records and late inputs still refuse.
Directory discovery's original contracts and retained evidence remain unchanged.
Own metadata remains 0.2.5; the incoming Host 0.17.7 lock is preserved byte-for-byte.
Full Linux `make ci lint check-secrets` passes with 164 units, Cargo/Wasm/process
fixtures, strict Clippy/docs and actual Rust 1.88.

Canic's stopped 0.112 release attempt rejected an unobserved foreign App manifest
while Blob's worker shared the suite target. Retained diagnostics prove the
before/after mismatch. A concurrent owner now scopes records to the selected
Cargo graph; [Canic #544](https://github.com/dragginzgame/canic/issues/544) tracks
that repair and shared-owner adoption. This task changes no live Canic source.
The isolated adapter replaces repeated record interpretation with the new API;
Canic retains target selection, generated-export recipes and release admission.
Its 91 build-family checks pass, with three governed cases ignored. Strict Host
library/test Clippy and actual Rust 1.91 all-feature library compilation pass.
The temporary producer override is confined to
the qualification copy. Registry package versions/checksums stay unchanged;
Cargo also normalizes some existing Windows dependency edges there, retained in
`adapter-lock-delta.json`. The real consumer graph is untouched.

Evidence and adapter patch are under `/tmp/ic-build-canic-025-integration/`.
Host 0.17.8 remains unpublished; its native repair must be delivered and adopted
before exact macOS qualification. Publish the producer boundary before Canic's
registry switch. The explicit #542 hold remains until release completion; the
failed attempt and new concurrent source edits do not constitute delivery.

[#7](https://github.com/dragginzgame/ic-build/issues/7) closes on published
IC Build 0.2.5 and Testkit 0.38.0 helper consolidation. Testkit's exact source
`35938ee0c8fc2c3e5efa2cf198e39d6e8c486b2e` passes all four Linux CI lanes;
the separate Host/native and broader extraction acceptance issues remain open.

## Authorized Host repair prepared — 2026-10-13

The maintainer authorized [Host #84](https://github.com/dragginzgame/ic-host-tooling/issues/84).
The repair is applied to Host's released 0.17.7 source
`2bdc5f1cae2c12040bf08d03b5c68502291599cd`, with compatible pending 0.17.8 notes.
Retained staging reuses its existing checked native-permissions helper; invalid
high-bit permissions refuse before creation. Full Linux `make ci msrv-workspace`
passes, including 82 filesystem tests, strict Clippy/docs, tooling/release fixtures
and actual Rust 1.88 workspace compilation. Host's lockfile is unchanged.
Evidence is under `/tmp/ic-build-host84/`. Native qualification and publication
remain outstanding; IC Build's minimum requirement/locked graph adoption must
follow the published repair. Own IC Build versions and dependency inputs remain unchanged.

The completed 0.2.5 CI now fails on both macOS architectures at the same Host
0.17.5 call, with Linux and lint/security passing. Apple Silicon's job log is
`/tmp/ic-build-host84/ic-build-025-apple.log`. Canic's independent 0.112.0 release
is still running governed tests on admitted `644d4b13012b395d6211a6907e2e6aafdf391db9`;
its explicitly queued #542 patch remains unapplied. This follow-up performs no
staging, commit, version bump, release, publication or CI dispatch.

## Pushed 0.2.5, native dependency blocker and Canic hold — 2026-10-13

The maintainer pushed finalized 0.2.5 source
`1ae67242b73e002414105c8de55b05f9364c1e0a`. Its
[exact CI](https://github.com/dragginzgame/ic-build/actions/runs/38441416124)
passes Linux and lint/security; Intel fails during package compilation against
locked Host 0.17.5, and Apple Silicon is queued at inspection. The previous
0.2.4 run now has both native jobs failed against Host 0.17.4. These results
supersede the running/queued observations in the historical sections below.

Both completed native 0.2.4 jobs and the 0.2.5 Intel job report E0308 in Host's
retained staging: its public `u32` permissions reach a Darwin `u16` mode
constructor. The existing checked native-permissions helper already handles
Apple narrowing in the same filesystem owner. The repair and native acceptance
are tracked at [Host #84](https://github.com/dragginzgame/ic-host-tooling/issues/84).
Registry 0.17.6/0.17.7 and current Host source retain the offending call; source
inspection does not qualify those releases. A published Host repair and its
minimum requirement/locked graph adoption must precede new exact native
producer qualification. No downstream workaround or Host source edit was made.
Local logs are `/tmp/ic-build-024-intel.log`, `/tmp/ic-build-024-arm.log` and
`/tmp/ic-build-025-intel.log`; the hosted jobs retain portable evidence.

Canic's independent release remains active on admitted source
`644d4b13012b395d6211a6907e2e6aafdf391db9`, preparing 0.112.0 from 0.111.0.
Its release validation is running governed PocketIC tests and infrastructure
Cargo builds at inspection. The maintainer's explicit hold still applies to
[Canic #542](https://github.com/dragginzgame/canic/issues/542). The retained
selector/identity patch remains unapplied until that release finishes; IC Build's
push does not end Canic's source admission. No sibling source, Git refs, package
versions, release, publication or workflow dispatch were changed in this follow-up.

## Published 0.2.4, helper cleanup and Canic integration — 2026-10-13

Registry IC Build 0.2.4 names source
`b3eba0f4b45fbca8a2a8cba6afcc3cfd88be5348`; checksum is
`68c9379fb1224f483d1896cb91448d0bf17f46a2c98277c1402d71867adb0a23`.
Its [exact CI](https://github.com/dragginzgame/ic-build/actions/runs/38424234824)
passes Linux/lint, with Intel running and Apple Silicon queued at inspection.
The cancelled 0.2.3 run does not establish its unfinished Intel result.

The maintainer authorized Canic integration and continued issue work. For
[#7](https://github.com/dragginzgame/ic-build/issues/7), IC Build now uses standard
optional-duration reduction in both timing aggregates, preserving absent and
measured-zero values plus saturation. The duplicated timing module retires and
its regression follows the Wasm aggregate; artifact aggregation has its own
boundary coverage. Testkit's separate owner has already removed its generic
label/timing modules in the current dirty draft, with contract-specific label
admission and phase accumulation retained there. That source inspection is not
Testkit delivery or a new consumer qualification. No Testkit source was edited.

Compatible pending notes select 0.2.5; own metadata stays 0.2.4. Full Linux
`make ci lint check-secrets` passes with 163 units, actual Cargo/Wasm/process and
retention cases, strict Clippy/docs, actual Rust 1.88 and isolated release adapters.
The incoming Host 0.17.5 lock selection remains byte-for-byte unchanged.

Canic's catalog and lock advance only registry IC Build 0.2.3 to 0.2.4; other
selected packages/checksums are unchanged. Configured and infrastructure Cargo
commands now select libraries, preserving named-library JSON selection, profiles,
features and qualification/publication policy. A new actual Cargo collision case
fails on the original command and passes after the repair. The redundant generic
subcommand helper retires into the existing build command owner. The maintainer
committed these applied files while restarting the Canic release; this agent
performed no staging, commit, release or publication action.

The further shared selector/complete-build identity patch is qualified in the
isolated consumer and retained at `/tmp/ic-build-canic-024/canic-selector-followup.patch`.
The maintainer explicitly requests queueing it until the restarted release
finishes. Its application must preserve that admitted source. Record
format and namespace remain unchanged; the selected library recipe contributes
to identity without reading a parallel cache format or deleting retained records.
Evidence and before-images are under `/tmp/ic-build-canic-024/`; producer validation
is `producer-full-validation.log`. Final consumer checks and application status
are recorded on Canic #528/#529/#530 and IC Build #9/#12. The queued follow-up
is [Canic #542](https://github.com/dragginzgame/canic/issues/542), by explicit
maintainer instruction; it is not applied to the active release source. Native and complete
application acceptance remain open. Historical sections keep their source identities.

## Shared Tooling 0.11.2 and Host 0.17.4 adoption — 2026-10-13

The maintainer authorized both upgrades. Shared Tooling 0.11.2 is exported from
clean committed source `81bdf144482b21f146acf542f8013eefe1dfeba8`, matching remote
main at review, through its canonical preview/refresh workflow. The snapshot
verifies all 86 payloads. Local CI setup uses `install-ci-tool.sh TOOL`; obsolete
snapshot-owned forwarding commands retire through the exporter. Existing common
host/IC/Rust and CI versions/digests are unchanged. Shared release recovery uses
the annotated-tag checker; selected evidence and setup fixes arrive with the
same reviewed snapshot. No live release or hosted evidence round trip ran.

The workspace catalog now requires compatible artifact/fs/process Host 0.17.4
releases, preserving explicit features and disabled defaults. Cargo registry
search confirms 0.17.4 is the newest stable release for all three; their registry
VCS receipts agree on `28f3a6d5771ecb941b86cd3c6ea2507de82b101e`.
The incoming dirty lockfile already selected these versions and is preserved
byte-for-byte. No unrelated dependency upgrade or sibling edit ran.

Explicit locked fetch, `make install-tools tools-check` and actual unified CI
installer preparation pass. Full Linux `make ci lint check-secrets` passes with
162 unit cases, real Cargo/Wasm and retained/process checks, strict Clippy/docs,
actual Rust 1.88 and isolated release-adapter fixtures. Final local links and
snapshot integrity also pass. Evidence and before-images are retained under
`/tmp/ic-build-tooling-upgrade/`; `full-validation.log` is the complete gate receipt.

This compatible consumer adoption joins the existing pending 0.2.4 notes; Shared
Tooling's CLI retirement does not break IC Build's published library boundary.
Own version stays 0.2.3. No staging, commit, push, package-version change, release,
publication or CI dispatch ran. Native qualification of this source/graph remains
pending at #1/#9. Prior 0.2.3 CI passes Linux, Apple Silicon and lint/security;
Intel is running at inspection. Those results do not qualify this new snapshot
or Host selection. Earlier sections preserve their original evidence.

## Published 0.2.3 and pending 0.2.4 consumer boundaries — 2026-10-13

Registry 0.2.3 resolves to source
`23e047b7672aae0d5ba442c4b384c4b782835c87`; its archive SHA-256 is
`9f5baf0449153f28de324d37dc9787298d77788940e0bce88f3d0c252b018ca5`.
Its [exact CI](https://github.com/dragginzgame/ic-build/actions/runs/38416043907)
passes Linux and lint/security; both native macOS 15 jobs are running at inspection.
[#15](https://github.com/dragginzgame/ic-build/issues/15) remains open for that
source's native qualification. The older 0.2.2 run was cancelled.

The maintainer accepted the #16/#14/#13 producer batch. Pending notes select
compatible 0.2.4: additive APIs and corrected default Cargo filenames, with
existing default-library identity, v1 formats, namespaces and execution policy
preserved. Own manifests and lock remain 0.2.3.

- [#16](https://github.com/dragginzgame/ic-build/issues/16): package hyphens become
  underscores in default cdylib output paths across cache modes. Renamed/non-cdylib
  libraries and colliding normalized filenames remain refused before acquisition.
- [#14](https://github.com/dragginzgame/ic-build/issues/14): adopt the prepared
  `CapturedArtifact::retain` transfer. Normal close/Drop still clean only their
  owned child. The Jobs adapter preserves original/O3/Os/Oz bytes, including
  retained capture after a failed rename and process exit. Published producer
  dependency and caller admission still precede Jobs adoption.
- [#13](https://github.com/dragginzgame/ic-build/issues/13): the existing resolver
  structurally selects one named cdylib example, fingerprints that selection and
  admits its output. The existing Cargo command factory is public for uncached,
  caller-executed builds. Query keeps deadlines, cancellation/cleanup, receipts,
  DID/metadata publication and smoke/deployment lifecycle. Shared input inventory
  projects Query's unchanged relative-path/raw-byte framing; it does not replace
  that receipt with `InputEvidence::digest()`.

Complete Linux `make ci lint check-secrets` passes: 162 units, six named-example
integration cases, 16 actual Cargo/Wasm cases, retained artifacts/process/input
checks, strict Clippy/docs, actual Rust 1.88 and isolated release adapters. The
new cases cover isolated/shared reuse, normalized example/library filenames,
changed/missing target admission, configured target-directory override, prepared
snapshot selection, source changes, late depfiles, capture lifetime and caller
failure/deadline/cancellation evidence. A frozen 796-file Query inventory gives
identical old/shared-projection source hashes and rejects mutation. Current Query
locked/offline metadata admits its named probe and selected output; a repeated
resolution confirms identity. These are
producer and adapter checks, not Query smoke or new native qualification.

Evidence is under `/tmp/ic-build-024/`: `full-validation.log`,
`jobs-adapter-proof.log` and `query-receipt-final.log`. Sibling repositories stay
read-only. No staging, commit, version change, push, release, publication or CI
trigger ran. Issues stay open for delivery/native acceptance and actual adoption;
historical sections below preserve their original identities.

## Published 0.2.2 and Canic input-engine integration — 2026-10-13

Version 0.2.2 is published from
`e9be97fa356c14cb5431b879e304a0f8cc5077c3`; archive SHA-256 is
`d408281094f1ae6928f02102131a904b8edcf5693b6c8f01b105e6488d0c4d82`, and
its registry VCS receipt names that release. The newer
[0.2.2 CI](https://github.com/dragginzgame/ic-build/actions/runs/38407403272)
passes Linux and lint/security; Intel fails the new native-path fixture before
capture because its `0xff` filename is inadmissible (`Illegal byte sequence`).
Apple Silicon is queued. Exact
[0.2.1 CI](https://github.com/dragginzgame/ic-build/actions/runs/38394499212)
passes Linux x86-64, native macOS 15 Intel/Apple Silicon and lint/security.
This closes the delivered setup/compiler/Clippy/catalog/editor repairs at
#5, #6, #8, #10 and #11; it does not qualify the new 0.2.2 evidence APIs.
[#15](https://github.com/dragginzgame/ic-build/issues/15) records the bounded
fixture repair: valid Unicode native files on every Unix host, with invalid-byte
files only on Linux, following the existing digest fixture. Actual capture,
digest and descendant symlink/path-error checks remain active on macOS. Engine
semantics are unchanged; exact delivered native qualification is still needed.

The maintainer authorized Canic integration after publication. Canic's catalog
requires registry 0.2.2; its incoming lock already selected that release. Its
complete-build production path now consumes `InputEvidence` directly for capture,
digest and before/after validation, and the strict shared depfile discovery API
for explicitly selected records. Its old recursive scanner, snapshot engine and
parser are removed. Canic retains role/family/generated-export/fixture recipes,
source custody, tool/environment identity, complete-release record format and
namespace, output projection, release/role admission, locks and diagnostics.

Two remaining generic regression declarations move from Canic to IC Build:
source edits/renames/exclusions and generated-root alias/external-input discovery.
They use the existing shared APIs. Complete Linux `make ci lint check-secrets`
passes with 161 units, integration/real Cargo/Wasm/process cases, strict Clippy,
docs and actual Rust 1.88. Own metadata stays 0.2.2; compatible pending 0.2.3
notes cover this producer fixture/test ownership cleanup. Canic's focused reuse suite
passes 40 cases, with its governed opt-in synthetic-sealing case ignored. These
include real Cargo generation, source/tool/environment drift, disappearing
inputs, cold/repeat release reuse, tampered outputs, diagnostics, cancellation and
required-tool absence retaining the original filesystem failure.

Receipts and task-owned before-images are under `/tmp/ic-build-canic-022/`:
`producer-final-validation.log`, `canic-final-clippy.log` and
`canic-final-reuse-tests.log`. Initial receipts remain alongside them. Concurrent
Canic Backup/query/dependency work is preserved. Explicit locked preparation
admits its incoming graph; final strict Host library/test Clippy and reuse checks
pass with Host 0.17.3/Query 0.55.4. Lock SHA-256 is
`0b30060e6d39ff08ac2c54b2d07de74b21f10863e19426ebba89a5630d78bc10`, unchanged
through those checks. The query owner resolves its duration lint. New-source
native and complete consumer acceptance remain at #12/#530/#1/#15.
The wider Canic 0.112 Auth/Backup/Blob batch remains open. No staging, commit,
package-version change, push, release, publication, CI dispatch or broad Canic
gate ran. Historical sections below retain their original source identities.

## Published 0.2.1 and pending 0.2.2 input evidence — 2026-10-13

Version 0.2.1 is published from
`b797ed7cdfc3303987d8c02af5aad8dadd22eddd`; archive SHA-256 is
`a52c3b1a7c80eb24899cd38bbf6767e792d1236cf6f9522f30dd075ab48fae79`.
The archive's VCS receipt names that release. Shared Tooling 0.10 is delivered
in this source. Its [CI](https://github.com/dragginzgame/ic-build/actions/runs/38394499212)
passes Linux, Apple Silicon and lint/security; Intel is running
at inspection. The prior 0.2.0 run was cancelled, so its unfinished hosts do not
establish native acceptance.

The maintainer reports Canic #530 is in progress with its own owner and asks for
continued IC Build issue work. This batch addresses the remaining current-guide
corrections in [#2](https://github.com/dragginzgame/ic-build/issues/2): generic
resolution, source leases and transactions now name IC Build; the public type
boundary selects Host 0.17. This guide and the implemented public Rustdoc own
the supported contracts. Original Testkit designs/migrations remain historical
references, pinned to extraction source
`b906b1a44f8f5f0f2f23d2413a76103d99e4fbb3` with their paths/anchors verified.
Retained `.ic-testkit` namespaces and stamp/cache bytes are untouched.

The ownership design separates implemented consumer routes from the original
audit/proposals. README and agent instructions refer to the canonical version
and handoff instead of carrying a stale workspace-version assertion. Pending
notes are compatible 0.2.2; own metadata remains 0.2.1. The initial documentation
checkpoint passed link/diff checks with executable inputs unchanged.

The active Canic #530 owner then filed
[#12](https://github.com/dragginzgame/ic-build/issues/12) for standalone input
authority. `InputEvidenceSpec`, `InputEvidence`, `InputEvidenceValue` and
`InputEvidenceError` now capture and validate per-path bytes, directory membership
and explicit absence independently of an artifact-cache transaction. Strict
`parse_cargo_depfile` / `discover_cargo_depfile_inputs` use caller-selected
directories, exclusions and record bounds. Original inputs are rechecked even
when depfiles disappear or replace their inventories; late bytes cannot acquire
pre-build authority. The existing traversal is shared, preserving old cache
fingerprints and the consumer's string-sorted, big-endian length-framed encoding.
No second production scanner, recipe, release admission or cache namespace is
introduced. Source/parent custody remains the caller's responsibility.

Thirteen focused cases and a real Cargo external-include depfile fixture pass.
Complete Linux `make ci lint check-secrets` passes, including 159 units, retained
artifact/process/real Wasm integration checks, strict Clippy, docs, actual Rust
1.88 and isolated release-adapter fixtures. Receipts and before-images are under
`/tmp/ic-build-022-docs/`; the final full receipt is `full-validation.log`.
Canic's owner retains its implementation/migration and consumer acceptance; this
batch changes only IC Build. New source is locally verified, unpublished and has
no native macOS qualification yet. Released-source native rows and consumer
delivery acceptance remain at their existing GitHub issues. Manifests, lockfile,
tool selection and retained cache formats are unchanged. No staging, commit,
push, version/release/publication or CI action ran.

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
