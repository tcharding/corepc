# Verify

TL;DR: `cargo run -- --help`

This crate provides a tool to help verify the rest of the repository.
Specifically the Single Source Of Truth for a specific Core version is
the version module in `types` e.g., `types/src/v17/mod.rs`.

Verification is warranted because there are many pieces involved and
during development much cut'n'paste. We want to catch our mistakes and
have some confidence that our claims are valid.

Please see `types/README.md` for more information.

## Checking test output

To check the test output (using the `--tests`) flag one needs to first
generate a test output file. Do so by running the integration tests
and saving the output.

`cargo test --features=28_0 2>&1 > /tmp/test-28.out`

Be sure to be in the `integration_test` directory when you run this
command otherwise the tests will not be run correctly. (I don't know
why but cargo features are not passed along to all crates how I would
expect them to be.)