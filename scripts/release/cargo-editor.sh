#!/usr/bin/env bash
set -euo pipefail

# Print the admitted editor path; --check never installs or repairs it.
if [[ $# -gt 1 || ( $# -eq 1 && "$1" != --check ) ]]; then
    echo 'usage: cargo-editor.sh [--check]' >&2
    exit 2
fi
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd)"
# shellcheck source=/dev/null
. "$root/ci/release-tools.env"
exec bash "$root/scripts/dev/install-rust-tools.sh" --consumer "$root" \
    --package cargo-edit --version "$IC_BUILD_CARGO_EDIT_VERSION" \
    --bin cargo-set-version --profile release "$@"
