//! Provides a macro that implements the tests.

pub mod v17;
pub mod v19;
pub mod v22;

/// Requires `RPC_PORT` to be in scope.
use node::Node;

/// Initialize a logger (configure with `RUST_LOG=trace cargo test`).
#[allow(dead_code)] // Not all tests use this function.
pub fn init_logger() { let _ = env_logger::try_init(); }

/// Returns a handle to a `bitcoind` instance with "default" wallet loaded.
#[allow(dead_code)] // Not all tests use this function.
pub fn bitcoind_with_default_wallet() -> Node {
    init_logger();

    let exe = node::exe_path().expect("failed to get bitcoind executable");

    let conf = node::Conf::default();
    Node::with_conf(exe, &conf).expect("failed to create node")
}

/// Returns a handle to a `bitcoind` instance without any wallets.
#[allow(dead_code)] // Not all tests use this function.
pub fn bitcoind_with_wallet(wallet: String) -> Node {
    let exe = node::exe_path().expect("failed to get bitcoind executable");

    let mut conf = node::Conf::default();
    conf.wallet = Some(wallet);
    Node::with_conf(exe, &conf).expect("failed to create node")
}

/// Returns a handle to a `bitcoind` instance without any wallet loaded.
#[allow(dead_code)] // Not all tests use this function.
pub fn bitcoind_no_wallet() -> Node {
    let exe = node::exe_path().expect("failed to get bitcoind executable");

    let mut conf = node::Conf::default();
    conf.wallet = None;
    Node::with_conf(exe, &conf).expect("failed to create node")
}
