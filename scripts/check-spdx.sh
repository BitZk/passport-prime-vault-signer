#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 BitZk contributors
# SPDX-License-Identifier: MIT

set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

failed=0

while IFS= read -r -d '' path; do
    if ! head -n 8 "$path" | grep -q 'SPDX-License-Identifier:'; then
        printf 'missing SPDX identifier: %s\n' "$path" >&2
        failed=1
    fi
done < <(find src ui resources scripts .githooks .github/workflows build.rs Cargo.toml app-config.toml permission_templates.toml \
    -path ui/gen -prune -o \
    -type f \
    \( -name '*.rs' -o -name '*.slint' -o -name '*.toml' -o -name '*.sh' -o -name '*.yml' -o -name 'pre-commit' \) \
    -print0)

if [[ "$failed" -ne 0 ]]; then
    exit 1
fi

printf 'SPDX checks passed\n'
