# Bitcoin Core JSON-RPC types

This crate provides data types return by Bitcoin Core's JSON-RPC API. Each type is specific to the
version of Core e.g., if you run the `getblockchaininfo` method against a Bitcoin Core v28 instance
you will get back the data described by `types::v28::GetBlockChainInfo`. In a similar fashion any
method `corerpcmethod` will return type `CoreRpcMethod` - snake-case as is conventional in Rust.

## Status

This crate is Work In Progress - not all methods for all Core versions are done yet. The single
source of truth (SSOT) for a methods status can be found in the version specific module e.g.,
`types/src/v17/mod.rs`. The HTML version can be found online and has nice drop down menus.

See for example: https://docs.rs/corepc-types/0.5.0/corepc_types/v18/index.html

### As of `v0.5.0`

- All types to support `rust-miniscript` exist for Core versions 17-28 inclusive.
- Support for Core Versions v17 and v18 is more fully fleshed out.
- Nice docs and `verify` tool for v17 and v18 only.

### Testing and Verification

In order to prove the data structures we do integration testing in `integration_test`. The tests are
version specific e.g., `cargo test --features=0_18_1`. In CI we test against all supported versions.
If you are using this crate in CI you may want to imitate the job structure. See the `Integration`
job in `.github/workflows/rust.yaml`.

In order to back up method status our claims we provide the `verify` tool that parses the SSOT and
checks the claims. Run it using `verify v17` (also `verify all`).

The tool only currently verifies the `v17` and `v18` modules.

## Minimum Supported Rust Version (MSRV)

This library should always compile with any combination of features on **Rust 1.63.0**.

## Licensing

The code in this project is licensed under the [Creative Commons CC0 1.0 Universal license](LICENSE).
We use the [SPDX license list](https://spdx.org/licenses/) and [SPDX IDs](https://spdx.dev/ids/).
