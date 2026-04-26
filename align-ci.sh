#!/bin/bash
################################################################################
#
#    Copyright (c) 2026.
#    Haixing Hu, Qubit Co. Ltd.
#
#    All rights reserved.
#
################################################################################
#
# One-shot auto-fix to match local CI (fmt + clippy on all targets, then verify).
# Run from repo root: ./align-ci.sh
#

set -euo pipefail

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$SCRIPT_DIR"

if ! command -v cargo > /dev/null 2>&1; then
    echo "cargo is required but was not found"
    exit 1
fi

if ! command -v rustup > /dev/null 2>&1; then
    echo "rustup is required but was not found"
    exit 1
fi

if ! rustup toolchain list | grep -q '^nightly'; then
    echo "Installing nightly toolchain..."
    rustup toolchain install nightly
fi

echo "==> ensuring nightly rustfmt and clippy components"
rustup component add rustfmt clippy --toolchain nightly

echo "==> cargo +nightly fmt"
cargo +nightly fmt

echo "==> cargo +nightly clippy --fix (all targets / features)"
cargo +nightly clippy --fix --allow-dirty --allow-staged --all-targets --all-features

echo "==> cargo +nightly clippy (verify, -D warnings)"
cargo +nightly clippy --all-targets --all-features -- -D warnings

echo "Done. CI-style checks should pass; run ./ci-check.sh for the full pipeline."
