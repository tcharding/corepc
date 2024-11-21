# Dev notes


## TODOs

- Scrape all the integration tests from Bitcoin Core python code and run them here.
- Change in-specific to non-specific
- Use `std::` everywhere instead of `core::`


## HOWTO

### Add a JSON type for an RPC method

An example workflow to add a type for a new method,  `getrawchangeaddress` as an example.

- Add the type in `json/src/v17/wallet.rs`
- Add a type in `json/src/model/wallet.rs`
- Implement `into_json` to convert the first to the second

(Note the second two steps are only needed if the type has fields that should use rust-bitcoin types.)

### Add a function to the client and an test to the integration tests

An example workflow to add a method to the client,  `getrawchangeaddress` as an example.

- Add macro to `client/src/client_syn/v17/wallet.rs`
- Call it in `client/src/client_syn/v17/mod.rs`
- Add macro to `integration_test/src/v17/wallet.rs`
- Call it in `integration_test/tests/v17_api.rs`
