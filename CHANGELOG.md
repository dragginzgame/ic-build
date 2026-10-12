# Changelog

The 0.1.0 section preserves the original bootstrap notes. The published 0.1.0
archive also includes the extracted artifact engine and Cargo grouping API.

## [0.1.1]

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
