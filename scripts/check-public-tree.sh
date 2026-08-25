#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 BitZk contributors
# SPDX-License-Identifier: MIT

set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

failed=0
mode="${1:---worktree}"
if [[ "$mode" != "--worktree" && "$mode" != "--staged" ]]; then
    printf 'usage: %s [--worktree|--staged]\n' "$0" >&2
    exit 2
fi

list_paths() {
    if [[ "$mode" == "--staged" ]]; then
        git ls-files -z
    else
        find . -path './.git' -prune -o -path './target' -prune -o -type f -print0
    fi
}

while IFS= read -r -d '' path; do
    base="$(basename "$path")"
    case "$base" in
        .env.example) ;;
        private.pem|cosign2.toml|.env|.env.*|*.key|*.p8|*.p12|*.pfx|*.pem|*.crt|*.cer|*.seed|*.mnemonic|*.psbt|*.app|*.elf|*.bin)
            printf 'forbidden public-tree filename: %s\n' "$path" >&2
            failed=1
            ;;
    esac
    if [[ "$mode" == "--staged" && "$path" == .foundation-sdk* ]]; then
        printf 'generated SDK mapping must not be tracked: %s\n' "$path" >&2
        failed=1
    fi
done < <(list_paths)

pattern='BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY|xprv[1-9A-HJ-NP-Za-km-z]{50,}|AGE-SECRET-KEY-[A-Z0-9]{20,}'

if [[ "$mode" == "--staged" ]]; then
    # Report filenames only: matching content must never enter public CI logs.
    if git grep --cached -l -I -E "$pattern" -- .; then
        printf 'possible secret-like staged content found\n' >&2
        failed=1
    else
        status=$?
        [[ "$status" -eq 1 ]] || exit "$status"
    fi
else
    while IFS= read -r -d '' path; do
        if grep -lI -E "$pattern" "$path"; then
            printf 'possible secret-like content found in %s\n' "$path" >&2
            failed=1
        else
            status=$?
            [[ "$status" -eq 1 ]] || exit "$status"
        fi
    done < <(list_paths)
fi

if [[ "$failed" -ne 0 ]]; then
    exit 1
fi

printf 'public-tree safety checks passed\n'
