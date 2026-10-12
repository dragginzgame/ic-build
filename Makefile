.DEFAULT_GOAL := help
# The empty workspace has only a manifest to format. Add cargo fmt --all to
# both commands when extraction introduces the first maintained Rust package.
FORMAT_WRITE_COMMAND = bash -ec '"$$1" sort --workspace' -- "$(FORMAT_CARGO)"
FORMAT_CHECK_COMMAND = bash -ec '"$$1" sort --workspace --check' -- "$(FORMAT_CARGO)"
override SHARED_TOOLING_ROOT := $(CURDIR)
include make/consumer.mk

.PHONY: help version tasks check ci check-doc-links check-workspace check-pins \
        check-release-commands install-validation-tools lint check-shell \
        check-workflows check-secrets release-version release-preflight
SHELLCHECK ?= $(CURDIR)/.tools/ci/bin/shellcheck
ACTIONLINT ?= $(CURDIR)/.tools/ci/bin/actionlint
GITLEAKS ?= $(CURDIR)/.tools/ci/bin/gitleaks

help:
	@echo 'version / tasks: workspace version and shared maintenance catalog'
	@echo 'check / ci: snapshot, docs, manifest, declared pins, formatting and release routing'
	@echo 'install-tools / tools-check: explicit shared setup and offline admission'
	@echo 'fmt / fmt-check: sort Cargo manifests; Rust source formatting begins with extraction'
	@echo 'install-hooks: activate the shared repository-local pre-commit formatter'
	@echo 'cloc: shared workspace Rust LOC report; available once packages exist'
	@echo 'install-validation-tools: pinned Linux x86-64 ShellCheck, actionlint and Gitleaks'
	@echo 'lint / check-secrets: shell/workflow lint and history secret scan'
	@echo 'release-*: shared entrypoints; unavailable until a real package and adapter exist'

version:
	@bash scripts/ci/read-cargo-workspace-version.sh --stable Cargo.toml

tasks:
	@cat tasks/README.md

check ci:
	+@bash scripts/ci/run-validation-targets.sh --fail-fast \
		shared-tooling-check tools-check check-doc-links check-workspace check-pins \
		fmt-check check-release-commands

check-doc-links:
	perl scripts/ci/check-documentation-links.pl --root "$(CURDIR)" \
		*.md audits/*.md rules/*.md tasks/*.md docs/*.md docs/principles/*.md \
		docs/design/*.md docs/status/*.md

check-workspace:
	+@CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 cargo metadata \
		--offline --locked --no-deps --format-version 1 > /dev/null

check-pins:
	+@bash scripts/ci/check-bootstrap-pins.sh

check-release-commands:
	@bash scripts/ci/check-release-commands.sh "$(CURDIR)" \
		make/consumer.mk make/tools.mk make/rust-format.mk make/release.mk \
		make/execution.mk scripts/ci/check-make-execution.sh

install-validation-tools:
	@bash -ec 'test "$$(uname -s):$$(uname -m)" = Linux:x86_64; \
		. ci/tool-versions.env; \
		bash scripts/ci/install-shellcheck.sh --version "$$SHARED_TOOLING_SHELLCHECK_VERSION" \
			--sha256 "$$SHARED_TOOLING_SHELLCHECK_SHA256_LINUX_X86_64" --install-dir "$(CURDIR)/.tools/ci/bin"; \
		bash scripts/ci/install-actionlint.sh --version "$$SHARED_TOOLING_ACTIONLINT_VERSION" \
			--sha256 "$$SHARED_TOOLING_ACTIONLINT_SHA256_LINUX_AMD64" --install-dir "$(CURDIR)/.tools/ci/bin"; \
		bash scripts/ci/install-gitleaks.sh --version "$$SHARED_TOOLING_GITLEAKS_VERSION" \
			--sha256 "$$SHARED_TOOLING_GITLEAKS_SHA256_LINUX_X64" --install-dir "$(CURDIR)/.tools/ci/bin"'

lint: check-shell check-workflows

check-shell:
	"$(SHELLCHECK)" scripts/ci/*.sh scripts/dev/*.sh .githooks/pre-commit

check-workflows:
	"$(ACTIONLINT)" .github/workflows/*.yml

check-secrets:
	"$(GITLEAKS)" git --redact=100 --no-banner --no-color --log-level error "$(CURDIR)"

release-version release-preflight:
	@echo 'IC Build has no implemented package or reviewed release adapter; releases are unavailable.' >&2
	@exit 1
