# Changelog

## [0.1.1]

- Extract Testkit's generic artifact/Wasm engine and behavioral/process tests,
  and Canic's feature-equivalent Cargo grouping. Retain the existing cache formats
  and namespaces; consumers select recipes and retain their release/runtime policy
  ([#1](https://github.com/dragginzgame/ic-build/issues/1)).
- Run the extracted behavioral, cross-process and real Cargo/Wasm tests in the
  package gate and include the preserved MIT license notice in package contents.
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
