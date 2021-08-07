#!/usr/bin/env bash

set -euo pipefail

# Hack to be able to build with the stable compiler
export RUSTC_BOOTSTRAP=1

cbindgen \
    --config cbindgen.toml \
    --output include/mpv_socket.h
