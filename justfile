set export

REPO_DIR := `git rev-parse --show-toplevel`

default:
  @just --list

# Cargo build everything.
build:
  cargo build --workspace --all-targets --all-features

# Cargo check everything.
check:
  cargo check --workspace --all-targets --all-features

# Lint everything.
lint:
  cargo +$(cat ./nightly-version) clippy --workspace --all-targets --all-features -- --deny warnings
  cd $REPO_DIR/node > /dev/null; cargo +$(cat ../nightly-version) clippy --all-targets --all-features -- --deny warnings

# Run cargo fmt
fmt:
  cargo +$(cat ./nightly-version) fmt --all
  cd $REPO_DIR/node > /dev/null; cargo +$(cat ../nightly-version) fmt

# Check the formatting
format:
  cargo +$(cat ./nightly-version) fmt --all --check

# Generate documentation.
docsrs *flags:
  RUSTDOCFLAGS="--cfg docsrs -D warnings -D rustdoc::broken-intra-doc-links" cargo +$(cat ./nightly-version) doc --all-features {{flags}}

# Update the recent and minimal lock files.
update-lock-files:
  contrib/update-lock-files.sh
