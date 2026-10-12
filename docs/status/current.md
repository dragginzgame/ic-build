# Current handoff

## Planning bootstrap

The maintainer authorized creation of the local `ic-build` repository and saving
[the build ownership audit](../design/build-ownership.md) here for coordination.
The repository contains the proposed ownership split, findings, extraction order,
source evidence and required safety boundaries. No production implementation has
been copied, extracted or retired.

Shared Tooling 0.9.0 is canonically adopted at
`aec7a149f581a42b9531fc747b46024fc0314fcf`. Source was an isolated clean checkout;
dirty sibling Shared changes were not adopted. The Make entrypoint provides common
explicit setup and release commands; release metadata admission refuses while
this repository has no package or release adapter.

`make check` is the documentation-stage gate: snapshot integrity and local links.
There is no Cargo workspace, package version, executable, installed toolset or
native qualification yet. Create the workspace and meaningful package gates when
extraction implementation is explicitly started; do not manufacture a placeholder
crate, successful runtime test result or release identity for the planning stage.

The local repository uses an unborn `main` branch. Bootstrap leaves all files
unstaged and uncommitted. No GitHub remote or publication is established. The
next implementation work is governed by the design; sibling changes retain their
own target authorization and coordination requirements.

GitHub issues remain the sole follow-up tracker. This handoff is setup evidence,
not an issue backlog. No sibling source or existing build artifacts were changed.
