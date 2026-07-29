// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v30` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod into;

use serde::{Deserialize, Serialize};

pub use super::GetMempoolInfoError;

/// Result of JSON-RPC method `getmempoolinfo` with verbose set to `true`.
///
/// > getmempoolinfo
/// >
/// > Returns details on the active state of the TX memory pool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolInfo {
    /// True if the initial load attempt of the persisted mempool finished.
    pub loaded: bool,
    /// Current tx count.
    pub size: i64,
    /// Sum of all virtual transaction sizes as defined in BIP 141.
    ///
    /// Differs from actual serialized size because witness data is discounted.
    pub bytes: i64,
    /// Total memory usage for the mempool.
    pub usage: i64,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction.
    pub total_fee: f64,
    /// Maximum memory usage for the mempool.
    #[serde(rename = "maxmempool")]
    pub max_mempool: i64,
    /// Minimum fee rate in BTC/kB for a transaction to be accepted.
    ///
    /// This is the maximum of `minrelaytxfee` and the minimum mempool fee.
    #[serde(rename = "mempoolminfee")]
    pub mempool_min_fee: f64,
    /// Current minimum relay fee for transactions.
    #[serde(rename = "minrelaytxfee")]
    pub min_relay_tx_fee: f64,
    /// Minimum fee rate increment for mempool limiting or replacement in BTC/kvB.
    #[serde(rename = "incrementalrelayfee")]
    pub incremental_relay_fee: f64,
    /// Current number of transactions that haven't passed initial broadcast yet.
    #[serde(rename = "unbroadcastcount")]
    pub unbroadcast_count: i64,
    /// True if the mempool accepts RBF without replaceability signaling inspection.
    #[serde(rename = "fullrbf")]
    pub full_rbf: bool,
    /// True if the mempool accepts transactions with bare multisig outputs.
    ///
    /// Returned by Bitcoin Core v30; absent on Bitcoin Knots (v29.x).
    #[serde(rename = "permitbaremultisig")]
    pub permit_bare_multisig: Option<bool>,
    /// Maximum number of bytes that can be used by OP_RETURN outputs in the mempool.
    ///
    /// Returned by Bitcoin Core v30; absent on Bitcoin Knots (v29.x).
    #[serde(rename = "maxdatacarriersize")]
    pub max_data_carrier_size: Option<u64>,
    /// Minimum fee rate floor in BTC/kvB for dust outputs.
    ///
    /// Bitcoin Knots only.
    #[serde(rename = "dustrelayfeefloor")]
    pub dust_relay_fee_floor: Option<f64>,
    /// Dynamic dust fee rate configuration (e.g. "off", "target:N", "mempool:N").
    ///
    /// Bitcoin Knots only.
    #[serde(rename = "dustdynamic")]
    pub dust_dynamic: Option<String>,
    /// Transaction replacement policy (e.g. "always", "opt-in", "never").
    ///
    /// Bitcoin Knots only.
    pub rbf_policy: Option<String>,
    /// Policy for TRUC (v3) transactions (e.g. "accept", "reject", "enforce").
    ///
    /// Bitcoin Knots only.
    pub truc_policy: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::GetMempoolInfo;

    // Captured from Bitcoin Knots v29.3 (no permitbaremultisig/maxdatacarriersize,
    // extra dust/rbf/truc policy fields).
    #[test]
    fn deserialize_get_mempool_info_knots() {
        let json = r#"{"loaded":true,"size":2706,"bytes":799688,"usage":5006432,"total_fee":0.01656274,"maxmempool":300000000,"mempoolminfee":0.00001000,"minrelaytxfee":0.00001000,"incrementalrelayfee":0.00001000,"dustrelayfee":0.00003000,"dustrelayfeefloor":0.00003000,"dustdynamic":"off","unbroadcastcount":0,"fullrbf":true,"rbf_policy":"always","truc_policy":"accept"}"#;
        let info: GetMempoolInfo = serde_json::from_str(json).expect("knots response");
        assert_eq!(info.permit_bare_multisig, None);
        assert_eq!(info.max_data_carrier_size, None);
        assert_eq!(info.dust_relay_fee_floor, Some(0.00003));
        assert_eq!(info.dust_dynamic.as_deref(), Some("off"));
        assert_eq!(info.rbf_policy.as_deref(), Some("always"));
        assert_eq!(info.truc_policy.as_deref(), Some("accept"));
    }

    #[test]
    fn deserialize_get_mempool_info_core_v30() {
        let json = r#"{"loaded":true,"size":1,"bytes":100,"usage":1000,"total_fee":0.00000100,"maxmempool":300000000,"mempoolminfee":0.00001000,"minrelaytxfee":0.00001000,"incrementalrelayfee":0.00001000,"unbroadcastcount":0,"fullrbf":true,"permitbaremultisig":true,"maxdatacarriersize":100000}"#;
        let info: GetMempoolInfo = serde_json::from_str(json).expect("core v30 response");
        assert_eq!(info.permit_bare_multisig, Some(true));
        assert_eq!(info.max_data_carrier_size, Some(100000));
        assert_eq!(info.rbf_policy, None);
    }
}
