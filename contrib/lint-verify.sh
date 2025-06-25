#!/usr/bin/env bash
#
# The `verify` crate is not part of the workspace because it is a dev tool.

set -euox pipefail

REPO_DIR=$(git rev-parse --show-toplevel)

cargo +"$(cat ./nightly-version)" clippy \
      --manifest-path "$REPO_DIR/verify/Cargo.toml" \
      --config ./rustfmt.toml \
      --all-targets --all-features \
      -- --deny warnings

