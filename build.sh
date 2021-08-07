#!/usr/bin/env bash

set -euo pipefail

target_args=()
if [[ "$OSTYPE" == "msys"* ]]; then
    target_args=(--target x86_64-pc-windows-msvc)
fi

cargo build "${target_args[@]}" --features ffi "$@" \
    && ./gen_header.sh
