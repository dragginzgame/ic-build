#!/usr/bin/env bash
set -euo pipefail

# The planning workspace has no package graph or Cargo.lock. Validate its
# declarations with Shared Tooling's canonical predicates. At extraction,
# replace this adapter with check-dependency-pins.sh --cargo-inheritance.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd -P)"
cd "$root"
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
cargo metadata --offline --locked --no-deps --format-version 1 |
    jq -e '.packages == [] and .workspace_members == []' > /dev/null || {
        echo 'Package extraction requires the full shared dependency-pin checker and a tracked Cargo.lock.' >&2
        exit 1
    }

shopt -s nullglob
for path in Cargo.toml .github/workflows/*.yml .github/workflows/*.yaml \
    .github/actions/*/action.yml .github/actions/*/action.yaml; do
    case "$path" in
        Cargo.toml) kind=cargo; format=toml ;;
        .github/workflows/*) kind=workflow; format=yaml ;;
        *) kind=action; format=yaml ;;
    esac
    [[ -f "$path" && ! -L "$path" ]] || exit 1
    yq -p "$format" -o json -I 0 '.' "$path" |
        jq -es -L scripts/ci --arg file "$path" --arg kind "$kind" '
            include "dependency-pins";
            if length == 1 and (.[0] | type == "object") then
                {file: $file, kind: $kind, data: .[0]}
            else error("expected one configuration object: " + $file) end
            | [checks]
            | if length == 0 then true else error(tojson) end
        ' > /dev/null
done
echo 'Bootstrap Cargo and workflow/action declarations passed shared pin checks (no package graph yet).'
