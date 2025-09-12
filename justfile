set export

REPO_DIR := `git rev-parse --show-toplevel`

alias ulf := update-lock-files
alias l := lint
alias li := lint-integration-tests
alias lv := lint-verify

default:
  @just --list

# Cargo build everything.
build:
  cargo build --workspace --all-targets --all-features

# Cargo check everything.
check:
  cargo check --workspace --all-targets --all-features

# Lint everything.
lint: lint-verify lint-integration-tests
  cargo +$(cat ./nightly-version) clippy --workspace --all-targets --all-features -- --deny warnings

lint-verify:
  $REPO_DIR/contrib/lint-verify.sh

lint-integration-tests:
  $REPO_DIR/contrib/lint-integtation-tests.sh

# Run cargo fmt
fmt:
  cargo +$(cat ./nightly-version) fmt --all
  cargo +$(cat ./nightly-version) fmt --manifest-path $REPO_DIR/integration_test/Cargo.toml
  cargo +$(cat ./nightly-version) fmt --manifest-path $REPO_DIR/verify/Cargo.toml

# Check the formatting
format:
  cargo +$(cat ./nightly-version) fmt --all --check

# Generate documentation.
docsrs *flags:
  RUSTDOCFLAGS="--cfg docsrs -D warnings -D rustdoc::broken-intra-doc-links" cargo +$(cat ./nightly-version) doc --all-features {{flags}}

# Update the recent and minimal lock files.
update-lock-files:
  contrib/update-lock-files.sh
