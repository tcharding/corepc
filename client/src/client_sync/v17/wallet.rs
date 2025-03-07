// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `createwallet`.
#[macro_export]
macro_rules! impl_client_v17__addmultisigaddress {
    () => {
        impl Client {
            pub fn add_multisig_address_with_keys(
                &self,
                nrequired: u32,
                keys: Vec<PublicKey>,
            ) -> Result<AddMultisigAddress> {
                self.call("addmultisigaddress", &[nrequired.into(), into_json(keys)?])
            }

            pub fn add_multisig_address_with_addresses(
                &self,
                nrequired: u32,
                keys: Vec<Address>,
            ) -> Result<AddMultisigAddress> {
                self.call("addmultisigaddress", &[nrequired.into(), into_json(keys)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `bumpfee`.
#[macro_export]
macro_rules! impl_client_v17__bumpfee {
    () => {
        impl Client {
            pub fn bump_fee(&self, txid: Txid) -> Result<BumpFee> {
                self.call("bumpfee", &[into_json(txid)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `createwallet`.
#[macro_export]
macro_rules! impl_client_v17__createwallet {
    () => {
        impl Client {
            pub fn create_wallet(&self, wallet: &str) -> Result<CreateWallet> {
                self.call("createwallet", &[wallet.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `dumpprivkey`.
#[macro_export]
macro_rules! impl_client_v17__dumpprivkey {
    () => {
        impl Client {
            pub fn dump_priv_key(&self, address: &Address) -> Result<DumpPrivKey> {
                self.call("dumpprivkey", &[into_json(address)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `dumpwallet`.
#[macro_export]
macro_rules! impl_client_v17__dumpwallet {
    () => {
        impl Client {
            // filename is either absolute or relative to bitcoind.
            pub fn dump_wallet(&self, filename: &Path) -> Result<DumpWallet> {
                self.call("dumpwallet", &[into_json(filename)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getaddressesbylabel`.
#[macro_export]
macro_rules! impl_client_v17__getaddressesbylabel {
    () => {
        impl Client {
            pub fn get_addresses_by_label(&self, label: &str) -> Result<GetAddressesByLabel> {
                self.call("getaddressesbylabel", &[label.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getaddressinfo`.
#[macro_export]
macro_rules! impl_client_v17__getaddressinfo {
    () => {
        impl Client {
            pub fn get_address_info(&self, address: &Address) -> Result<GetAddressInfo> {
                self.call("getaddressinfo", &[into_json(address)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getbalance`.
#[macro_export]
macro_rules! impl_client_v17__getbalance {
    () => {
        impl Client {
            pub fn get_balance(&self) -> Result<GetBalance> { self.call("getbalance", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getnewaddress`.
#[macro_export]
macro_rules! impl_client_v17__getnewaddress {
    () => {
        impl Client {
            /// Gets a new address from `bitcoind` and parses it assuming its correct.
            pub fn new_address(&self) -> Result<bitcoin::Address> {
                use core::str::FromStr;

                let json = self.get_new_address(None, None)?;
                let address = bitcoin::Address::from_str(&json.0)
                    .expect("assume the address is valid")
                    .assume_checked(); // Assume bitcoind will return an valid address for the network its on.
                Ok(address)
            }

            /// Gets a new address from `bitcoind` and parses it assuming its correct.
            pub fn new_address_with_type(&self, ty: AddressType) -> Result<bitcoin::Address> {
                use core::str::FromStr;

                let json = self.get_new_address(None, Some(ty))?;
                let address = bitcoin::Address::from_str(&json.0)
                    .expect("assume the address is valid")
                    .assume_checked(); // Assume bitcoind will return an valid address for the network its on.
                Ok(address)
            }

            /// Gets a new address with label from `bitcoind` and parses it assuming its correct.
            // FIXME: unchecked network here is ugly and not uniform with other functions.
            pub fn new_address_with_label(
                &self,
                label: &str,
            ) -> Result<bitcoin::Address<bitcoin::address::NetworkUnchecked>> {
                use core::str::FromStr;

                let json = self.get_new_address(Some(label), None)?;
                let address =
                    bitcoin::Address::from_str(&json.0).expect("assume the address is valid");
                Ok(address)
            }

            fn get_new_address(
                &self,
                label: Option<&str>,
                ty: Option<AddressType>,
            ) -> Result<GetNewAddress> {
                match (label, ty) {
                    (Some(label), Some(ty)) =>
                        self.call("getnewaddress", &[into_json(label)?, into_json(ty)?]),
                    (Some(label), None) => self.call("getnewaddress", &[into_json(label)?]),
                    (None, Some(ty)) => self.call("getnewaddress", &["".into(), into_json(ty)?]),
                    (None, None) => self.call("getnewaddress", &[]),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getrawchangeaddress`.
#[macro_export]
macro_rules! impl_client_v17__getrawchangeaddress {
    () => {
        impl Client {
            pub fn get_raw_change_address(&self) -> Result<GetRawChangeAddress> {
                self.call("getrawchangeaddress", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getreceivedbyaddress`.
#[macro_export]
macro_rules! impl_client_v17__getreceivedbyaddress {
    () => {
        impl Client {
            pub fn get_received_by_address(
                &self,
                address: &Address<NetworkChecked>,
            ) -> Result<GetReceivedByAddress> {
                self.call("getreceivedbyaddress", &[address.to_string().into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettransaction`.
#[macro_export]
macro_rules! impl_client_v17__gettransaction {
    () => {
        impl Client {
            pub fn get_transaction(&self, txid: Txid) -> Result<GetTransaction> {
                self.call("gettransaction", &[into_json(txid)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getunconfirmedbalance`.
#[macro_export]
macro_rules! impl_client_v17__getunconfirmedbalance {
    () => {
        impl Client {
            pub fn get_unconfirmed_balance(&self) -> Result<GetUnconfirmedBalance> {
                self.call("getunconfirmedbalance", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getwalletinfo`.
#[macro_export]
macro_rules! impl_client_v17__getwalletinfo {
    () => {
        impl Client {
            pub fn get_wallet_info(&self) -> Result<GetWalletInfo> {
                self.call("getwalletinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listaddressgroupings`.
#[macro_export]
macro_rules! impl_client_v17__listaddressgroupings {
    () => {
        impl Client {
            pub fn list_address_groupings(&self) -> Result<ListAddressGroupings> {
                self.call("listaddressgroupings", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listlabels`.
#[macro_export]
macro_rules! impl_client_v17__listlabels {
    () => {
        impl Client {
            pub fn list_labels(&self) -> Result<ListLabels> { self.call("listlabels", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listlockunspent`.
#[macro_export]
macro_rules! impl_client_v17__listlockunspent {
    () => {
        impl Client {
            pub fn list_lock_unspent(&self) -> Result<ListLockUnspent> {
                self.call("listlockunspent", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listreceivedbyaddress`.
#[macro_export]
macro_rules! impl_client_v17__listreceivedbyaddress {
    () => {
        impl Client {
            pub fn list_received_by_address(&self) -> Result<ListReceivedByAddress> {
                self.call("listreceivedbyaddress", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listsinceblock`.
#[macro_export]
macro_rules! impl_client_v17__listsinceblock {
    () => {
        impl Client {
            pub fn list_since_block(&self) -> Result<ListSinceBlock> {
                self.call("listsinceblock", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listtransactions`.
#[macro_export]
macro_rules! impl_client_v17__listtransactions {
    () => {
        impl Client {
            pub fn list_transactions(&self) -> Result<ListTransactions> {
                self.call("listtransactions", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listunspent`.
#[macro_export]
macro_rules! impl_client_v17__listunspent {
    () => {
        impl Client {
            pub fn list_unspent(&self) -> Result<ListUnspent> { self.call("listunspent", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listwallets`.
#[macro_export]
macro_rules! impl_client_v17__listwallets {
    () => {
        impl Client {
            pub fn list_wallets(&self) -> Result<ListWallets> { self.call("listwallets", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `loadwallet`.
#[macro_export]
macro_rules! impl_client_v17__loadwallet {
    () => {
        impl Client {
            pub fn load_wallet(&self, filename: &str) -> Result<LoadWallet> {
                self.call("loadwallet", &[into_json(filename)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `rescanblockchain`.
#[macro_export]
macro_rules! impl_client_v17__rescanblockchain {
    () => {
        impl Client {
            pub fn rescan_blockchain(&self) -> Result<RescanBlockchain> {
                self.call("rescanblockchain", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sendmany`.
#[macro_export]
macro_rules! impl_client_v17__sendmany {
    () => {
        impl Client {
            pub fn send_many(&self, amounts: BTreeMap<Address, crate::AmountSerBtc>) -> Result<SendMany> {
                let dummy = ""; // Must be set to "" for backwards compatibility.
                self.call("sendmany", &[into_json(dummy)?, into_json(amounts)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sendtoaddress`.
#[macro_export]
macro_rules! impl_client_v17__sendtoaddress {
    () => {
        impl Client {
            // Send to address - no RBF.
            pub fn send_to_address(
                &self,
                address: &Address<NetworkChecked>,
                amount: Amount,
            ) -> Result<SendToAddress> {
                let args = [address.to_string().into(), into_json(amount.to_btc())?];
                self.call("sendtoaddress", &args)
            }

            // Send to address - with RBF.
            pub fn send_to_address_rbf(
                &self,
                address: &Address<NetworkChecked>,
                amount: Amount,
            ) -> Result<SendToAddress> {
                let comment = "";
                let comment_to = "";
                let subtract_fee_from_amount = false;
                let replaceable = true;

                let args = [
                    address.to_string().into(),
                    into_json(amount.to_btc())?,
                    comment.into(),
                    comment_to.into(),
                    subtract_fee_from_amount.into(),
                    replaceable.into(),
                ];
                self.call("sendtoaddress", &args)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `signmessage`.
#[macro_export]
macro_rules! impl_client_v17__signmessage {
    () => {
        impl Client {
            pub fn sign_message(&self, address: &Address, message: &str) -> Result<SignMessage> {
                self.call("signmessage", &[into_json(address)?, into_json(message)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `signrawtransactionwithwallet`.
#[macro_export]
macro_rules! impl_client_v17__signrawtransactionwithwallet {
    () => {
        impl Client {
            // `hexstring`: The transaction hex string.
            pub fn sign_raw_transaction_with_wallet(
                &self,
                hex: &str,
            ) -> Result<SignRawTransactionWithWallet> {
                self.call("signrawtransactionwithwallet", &[into_json(hex)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletcreatefundedpsbt`.
#[macro_export]
macro_rules! impl_client_v17__walletcreatefundedpsbt {
    () => {
        impl Client {
            pub fn wallet_create_funded_psbt(
                &self,
                inputs: Vec<WalletCreateFundedPsbtInput>,
                outputs: Vec<BTreeMap<Address, crate::AmountSerBtc>>,
            ) -> Result<WalletCreateFundedPsbt> {
                self.call("walletcreatefundedpsbt", &[into_json(inputs)?, into_json(outputs)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletprocesspsbt`.
#[macro_export]
macro_rules! impl_client_v17__walletprocesspsbt {
    () => {
        impl Client {
            pub fn wallet_process_psbt(&self, psbt: &bitcoin::Psbt) -> Result<WalletProcessPsbt> {
                self.call("walletprocesspsbt", &[into_json(psbt)?])
            }
        }
    };
}
