.DEFAULT_GOAL := help
# This bootstrap has documentation only. Enable Rust integration with the crate.
SHARED_TOOLING_RUST := 0
override SHARED_TOOLING_ROOT := $(CURDIR)
include make/consumer.mk

.PHONY: help check ci check-doc-links release-version release-preflight
help:
	@echo 'check / ci: verify snapshot integrity and documentation links'
	@echo 'install-tools / tools-check: explicit shared setup and offline admission'
	@echo 'release-*: shared entrypoints; unavailable until package metadata and adapter exist'
check ci:
	+$(MAKE) --no-print-directory shared-tooling-check
	+$(MAKE) --no-print-directory check-doc-links
check-doc-links:
	perl scripts/ci/check-documentation-links.pl --root "$(CURDIR)" README.md AGENTS.md docs/status/current.md docs/design/build-ownership.md
release-version release-preflight:
	@echo 'IC Build is a planning repository; package metadata and a release adapter are not defined.' >&2
	@exit 1
