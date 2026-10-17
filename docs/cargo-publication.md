# One Cargo publication attempt

`scripts/ci/run-cargo-publication.sh` executes one explicitly authorized,
consumer-admitted Cargo command. It owns retained intent, exact arguments,
command output and exit statuses. It never installs tools, discovers missing
packages, retries an upload or decides whether a previous upload succeeded.
The [release contract](releases.md) keeps publication separate from release.

## Calling contract

Requires Bash 3.2, GNU Make, Git, Cargo, tee and standard Unix utilities. Select and qualify
a Cargo version supporting every requested option and repeated package selection;
this does not change the library's MSRV. Run from the consumer's admitted source
checkout. Prepare an ignored evidence parent and any explicit target directory
before the consumer's final source check. For example, after consumer admission:

```bash
bash scripts/ci/run-cargo-publication.sh check \
  target/publication/check-unique "$admitted_commit" crates-io \
  --package example-library@1.2.3 --package example-cli@1.2.3 \
  --all-features
```

`check` always adds `--dry-run`; `publish` may upload and needs explicit publication
authorization. Both add `--locked` and the selected named registry. There is no
implicit workspace selection: every package must be an exact `NAME@X.Y.Z`, with
no duplicate names. Different selected package versions are allowed. Cargo
resolves those selectors; the consumer must first validate the complete eligible
package/version set and any intentionally omitted, already-published packages.

Supported additional options are `--manifest-path`, `--target-dir`, `--features`,
`--all-features`, `--no-default-features`, `--no-verify` and, for check only,
`--allow-dirty`. Path-option directories must already exist. Other options are
refused before allocation or Cargo execution. Explicit manifests must be regular,
non-symlink files within the selected Git checkout. Registry configuration and credentials
remain with the caller's Cargo environment; do not put secrets in feature/path
arguments. The recorded registry name is not independent proof of its configured
endpoint. Explicit offline settings remain authoritative.

Before dispatch the helper checks the supplied source commit against HEAD and
rechecks both HEAD and Git status after recording intent. Upload requires clean
source. A dirty check requires explicit `--allow-dirty` for Cargo to admit it.
These are race checks, not a seal over arbitrary concurrent filesystem changes.
The consumer retains its wider publication lock, source/tag/provenance admission,
registry observations, verification policy and final eligibility check.

## Evidence and interruption

The attempt directory must not exist, including as a dangling symlink. Its parent
must exist. Raw and resolved directory paths must contain no CR/LF. Spaces work.
An atomic `.cargo-publication.lock` in that parent excludes concurrent helper
attempts sharing the parent; consumers must use one stable parent. This does not
replace a consumer lock covering admission, provenance preparation and recovery.

Before Cargo starts, the fresh directory receives `intent` (mode, source, registry,
then exact package selectors), `cwd`, `working-status`, and NUL-separated
`arguments` including the executable name. A `dispatch` marker records that Cargo
may have started. `cargo.log` retains combined output. On completion, `cargo-status`,
`log-status` and `status` record the two pipeline statuses and returned status.
Cargo's nonzero status wins over a logger failure; logger failure cannot turn into
success. Evidence is retained on success and failure. No attempt is overwritten.

A completed attempt releases the helper lock. An interrupted or incomplete attempt
retains it for inspection, including failures while writing evidence. A dispatch
marker without completed statuses is uncertain, not proof of registry absence.
No resume or automatic retry exists. The consumer must reconcile exact registry
and provenance state, inspect the lock owner, and explicitly select any fresh
attempt. A failed invocation is never permission to repeat an upload blindly.
Status zero reports command completion, not independent registry readback.

## External lock selection admission

Pending IC Build 0.12.1 provides the pure
`ic_build::cargo::packages::verify_external_lock_subset(original, candidate, max_bytes)`
API for callers that need the archive engine's external-selection check without
running a package/test workflow. Both byte slices must fit the explicit
per-document allowance before UTF-8/TOML decoding. It performs no file or process
I/O, dependency resolution, network access or mutation.

Every candidate external name/version/source identity and optional checksum must
match the original. Registry checksums require canonical lowercase SHA-256;
non-registry optional checksum fields compare exactly, including absent versus
empty. Duplicate external identities refuse in either document. Extra valid
original selections and an empty candidate package array are allowed. Non-table
records and malformed identity/checksum fields refuse with typed role/index
evidence; UTF-8 and TOML failures retain their original error sources.

Unsourced tables are outside this check. Consumers retain exact local package
names, versions, paths, dependency edges, forbidden-feature policy, input custody
and release admission. Cargo archive qualification uses this same reader and
comparison, then independently requires a nonempty external graph. Valid archive
flows retain their selected-graph, evidence and lifetime contracts; malformed
records and changed non-registry checksum fields now refuse there too.

Auth's existing `apps/tooling` command boundary can read both locks with its
explicit file budget and invoke this check, leaving its library-graph admission
in the existing script. Publish the producer before adopting that import; a
source rehearsal or temporary path override does not establish registry adoption.

## Consumer adoption boundary

Keep existing adapters until their own equivalence checks pass against a reviewed
snapshot. Export the runner and its declared companion; do not patch vendored
copies. Consumers retain these distinct obligations:

| Consumer | Checks and recovery that stay local |
| --- | --- |
| Host Tooling | Package allowlist, synchronized versions, licenses, declared registry/repository, actual archive provenance, fresh remote source retrieval, all-feature selection and final source admission. |
| Canic | Resume-package admission, tri-state registry observations, selected missing set and final verification of the complete matching set, including predecessors omitted by resume. |
| IcyDB | Complete publishable-set admission, validation-only mode, resume selection and exact release-receipt qualification before choosing `--no-verify`. |
| Query | Exact absence versus lookup failure, source admission, library visibility before CLI upload, bounded retry settings and partial-upload reconciliation. |

Cargo owns dependency ordering, packaging, verification and upload for its admitted
selection. Do not assume this command proves equivalence with each consumer's
polling or recovery contract. Shared fixtures substitute Cargo/Git/logger effects
to test dispatch and evidence. They do not prove real registry propagation,
consumer policy or native host qualification. Consumer adoption must separately
exercise partial publication, unavailable observations, omitted packages, source
changes and actual supported Cargo behavior without unauthorized uploads.
