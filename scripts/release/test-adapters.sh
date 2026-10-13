#!/usr/bin/env bash
set -euo pipefail

# Real Git histories, metadata edits and tags stay inside this disposable fixture.
# Registry calls are strict substitutes. No source-checkout index/ref effects.
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset VALIDATION_REPOSITORY_ROOT VALIDATION_RUNNER_SNAPSHOT_PATH
unset RELEASE_KIND RELEASE_PREVIOUS RELEASE_VERSION RELEASE_DATE RELEASE_SOURCE RELEASE_COMMIT
export RELEASE_DELIVERY=direct
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"; root="${root%/.}"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/ic-build-release-adapters.XXXXXX")"
complete=false
failed_command=''
finish() {
    local status=$? log latest=''
    [[ "$complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" == 0 ]]; then rm -rf "$fixture"
    else
        echo "Release adapter fixture retained: $fixture" >&2
        printf 'Failed adapter command: %.2000s\n' "$failed_command" >&2
        for log in "$fixture"/*.log; do
            [[ -f "$log" ]] || continue
            if [[ -z "$latest" || "$log" -nt "$latest" ]]; then latest="$log"; fi
        done
        if [[ -n "$latest" ]]; then
            printf 'Last adapter log (up to 80 lines): %s\n' "$latest" >&2
            tail -n 80 "$latest" | awk '{ print substr($0, 1, 1000) }' >&2 || :
        fi
    fi
    exit "$status"
}
trap finish EXIT
trap 'failed_command=$BASH_COMMAND' ERR
mkdir -p "$fixture/workspace" "$fixture/bin" "$fixture/tmp"
export TMPDIR="$fixture/tmp"
for path in Cargo.toml Cargo.lock CHANGELOG.md README.md LICENSE Makefile .gitignore rust-toolchain.toml crates scripts make ci; do
    cp -R "$root/$path" "$fixture/workspace/$path"
done
mkdir -p "$fixture/workspace/.tools"
cp -P "$root"/.tools/host* "$root"/.tools/ic* "$fixture/workspace/.tools/"
cp -R "$root/.tools/rust" "$fixture/workspace/.tools/rust"
export YQ="$root/.tools/host/bin/yq"
export PATH="$root/.tools/host/bin:$root/.tools/ic/bin:$root/.tools/rust/common/bin:$PATH"
export IC_BUILD_TEST_REAL_CARGO
IC_BUILD_TEST_REAL_CARGO="$(command -v cargo)"
export IC_BUILD_TEST_CARGO_CALLS="$fixture/cargo.calls"
cat > "$fixture/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == publish ]]; then
    printf '%s\n' "$*" >> "$IC_BUILD_TEST_CARGO_CALLS"
    exit "${IC_BUILD_TEST_PUBLISH_STATUS:-0}"
fi
if [[ "$1" == metadata && "${IC_BUILD_TEST_FAIL_METADATA:-0}" == 1 ]]; then exit 42; fi
exec "$IC_BUILD_TEST_REAL_CARGO" "$@"
CARGO
chmod +x "$fixture/bin/cargo"
export PATH="$fixture/bin:$PATH"
export GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null
cd "$fixture/workspace"
perl -0pi -e 's/^check ci:$/check:/m; s/^check-secrets:\n(?:\t[^\n]*\n)+//m; s/^check-shell:\n(?:\t[^\n]*\n)+//m; s/^check-workflows:\n(?:\t[^\n]*\n)+//m' Makefile
cat >> Makefile <<GATES

ci lint check-secrets:
	@printf '%s\\n' '\$@' >> '$fixture/gates'
check-shell check-workflows:
	@:
GATES
# The adapter fixture exercises a patch independently of the live release draft.
fixture_previous="$(make --no-print-directory -s release-version)"
fixture_version="$(bash scripts/ci/next-release-version.sh "$fixture_previous" patch)"
printf '# Changelog\n\n## [%s]\n\n- Exercise the release adapter transaction.\n' "$fixture_version" > CHANGELOG.md
git init --quiet --template=
git config user.name Fixture
git config user.email fixture@example.invalid
git config core.hooksPath /dev/null
git add -- Cargo.toml Cargo.lock CHANGELOG.md README.md LICENSE Makefile .gitignore rust-toolchain.toml crates scripts make ci
git commit --quiet -m 'Adapter fixture baseline'
export RELEASE_SOURCE RELEASE_PREVIOUS RELEASE_VERSION RELEASE_DATE RELEASE_KIND
RELEASE_SOURCE="$(git rev-parse HEAD)"
RELEASE_PREVIOUS="$(make --no-print-directory -s release-version)"
RELEASE_KIND=patch RELEASE_DATE=2026-10-12
RELEASE_VERSION="$(bash scripts/ci/next-release-version.sh "$RELEASE_PREVIOUS" patch)"
expect_failure() {
    if "$@" > "$fixture/refusal.log" 2>&1; then
        echo 'adapter accepted an invalid fixture' >&2; exit 1
    fi
}
# The actual logger recursively loads this fixture's gate sentinels.
# The verification adapter itself remains the production recipe.
make --no-print-directory release-verify > "$fixture/gate.log" 2>&1
printf 'ci\nlint\ncheck-secrets\n' > "$fixture/expected-gates"
cmp "$fixture/gates" "$fixture/expected-gates"
cp Makefile "$fixture/pass-Makefile"
cat >> Makefile <<'GATES'

ci:
	@exit 23
lint check-secrets:
	@exit 97
GATES
expect_failure make --no-print-directory release-verify
cp "$fixture/pass-Makefile" Makefile

make --no-print-directory release-preflight > "$fixture/preflight.log" 2>&1
printf '\n// unrelated\n' >> crates/ic-build/src/lib.rs
expect_failure make --no-print-directory release-preflight
git restore -- crates/ic-build/src/lib.rs
printf '\nunrelated\n' >> README.md
git add -- README.md
git show HEAD:README.md > README.md
expect_failure make --no-print-directory release-preflight
git reset --quiet HEAD -- README.md
for path in Cargo.toml Cargo.lock CHANGELOG.md; do cp "$path" "$fixture/before-${path##*/}"; done
expect_failure env IC_BUILD_TEST_FAIL_METADATA=1 make --no-print-directory release-prepare-version
for path in Cargo.toml Cargo.lock CHANGELOG.md; do cmp "$path" "$fixture/before-${path##*/}"; done
make --no-print-directory release-prepare-version > "$fixture/prepare.log" 2>&1
[[ "$(make --no-print-directory -s release-version)" == "$RELEASE_VERSION" ]]
make --no-print-directory release-prepared-check > "$fixture/prepared.log" 2>&1
# Lost preparation replies are idempotent at the saved candidate.
make --no-print-directory release-prepare-version > "$fixture/reprepare.log" 2>&1
"$YQ" -p toml -o json '.' "$fixture/before-Cargo.lock" | jq '[.package[] | select(.source != null)]' > "$fixture/external-before"
"$YQ" -p toml -o json '.' Cargo.lock | jq '[.package[] | select(.source != null)]' > "$fixture/external-after"
cmp "$fixture/external-before" "$fixture/external-after"
make --no-print-directory -s release-files > "$fixture/release-files"
printf '%s\0' Cargo.toml Cargo.lock CHANGELOG.md > "$fixture/expected-files"
cmp "$fixture/release-files" "$fixture/expected-files"
git add -- Cargo.toml Cargo.lock CHANGELOG.md
make --no-print-directory release-commit-check > "$fixture/commit-check.log" 2>&1
git commit --quiet -m "Release $RELEASE_VERSION"
export RELEASE_COMMIT
RELEASE_COMMIT="$(git rev-parse HEAD)"
make --no-print-directory release-committed-check > "$fixture/committed.log" 2>&1
mkdir -p target
printf 'retained release artifact\n' > target/retained-artifact
expect_failure make --no-print-directory publish
[[ ! -e "$IC_BUILD_TEST_CARGO_CALLS" ]]
git tag -a "v$RELEASE_VERSION" -m "Release $RELEASE_VERSION"
make --no-print-directory publish-check > "$fixture/publish-check.log" 2>&1
[[ "$(cat "$IC_BUILD_TEST_CARGO_CALLS")" == 'publish -p ic-build --locked --registry crates-io --dry-run' ]]
make --no-print-directory publish > "$fixture/publish.log" 2>&1
[[ "$(tail -n 1 "$IC_BUILD_TEST_CARGO_CALLS")" == 'publish -p ic-build --locked --registry crates-io' ]]
expect_failure env IC_BUILD_TEST_PUBLISH_STATUS=24 make --no-print-directory publish
printf '\nnewer source\n' >> README.md
git add -- README.md
git commit --quiet -m 'Newer source'
expect_failure make --no-print-directory publish
# Late recovery validates the selected release commit, not the newer branch tip.
make --no-print-directory release-committed-check > "$fixture/older-commit.log" 2>&1
[[ "$(cat target/retained-artifact)" == 'retained release artifact' ]]
complete=true
echo 'IC Build metadata, gate routing, recovery and publication fixtures passed (no live release effects).'
