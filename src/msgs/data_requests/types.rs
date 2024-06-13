#[cfg(feature = "cosmwasm")]
use cw_storage_plus::{Key, Prefixer, PrimaryKey};
use semver::Version;
use sha3::{Digest, Keccak256};

use super::*;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub enum DataRequestStatus {
    Committing,
    Revealing,
    Tallying,
}

#[cfg(feature = "cosmwasm")]
impl<'a> PrimaryKey<'a> for DataRequestStatus {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = &'static str;
    type SuperSuffix = &'static str;

    fn key(&self) -> Vec<Key> {
        vec![Key::Val8(match self {
            DataRequestStatus::Committing => [0],
            DataRequestStatus::Revealing => [1],
            DataRequestStatus::Tallying => [2],
        })]
    }
}

#[cfg(feature = "cosmwasm")]
impl<'a> Prefixer<'a> for DataRequestStatus {
    fn prefix(&self) -> Vec<Key> {
        self.key()
    }
}

/// Represents a data request at creation time
#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Deserialize, Clone))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct DataRequest {
    /// Identifier
    pub id: String,

    // DR definition
    /// Semantic Version String
    pub version:            Version,
    /// Identifier of DR WASM binary
    pub dr_binary_id:       String,
    /// Inputs for DR WASM binary
    pub dr_inputs:          Bytes,
    /// Identifier of Tally WASM binary
    pub tally_binary_id:    String,
    /// Inputs for Tally WASM binary
    pub tally_inputs:       Bytes,
    /// Amount of required DR executors
    pub replication_factor: u16,
    /// Amount of SEDA tokens per gas unit
    pub gas_price:          U128,
    /// Maximum of gas units to be used by data request executors to resolve a data request
    pub gas_limit:          U128,
    /// Public info attached to DR
    pub memo:               Bytes,

    // Execution Information
    /// Payback address set by the relayer
    pub payback_address: Bytes,
    /// Payload set by SEDA Protocol (e.g. OEV-enabled data requests)
    pub seda_payload:    Bytes,
    /// Commitments submitted by executors
    pub commits:         HashMap<String, String>,
    /// Reveals submitted by executors
    pub reveals:         HashMap<String, RevealBody>,

    /// The height data request was posted. Used for commitment.
    pub height: u64,
}

impl DataRequest {
    pub fn has_committer(&self, public_key: &str) -> bool {
        self.commits.contains_key(public_key)
    }

    pub fn get_commitment(&self, public_key: &str) -> Option<&str> {
        self.commits.get(public_key).map(|s| s.as_str())
    }

    pub fn has_revealer(&self, public_key: &str) -> bool {
        self.reveals.contains_key(public_key)
    }

    pub fn reveal_started(&self) -> bool {
        self.commits.len() >= self.replication_factor as usize
    }

    pub fn reveal_over(&self) -> bool {
        self.reveals.len() >= self.replication_factor as usize
    }

    pub fn get_reveal(&self, public_key: &str) -> Option<&RevealBody> {
        self.reveals.get(public_key)
    }
}

/// Represents a resolved data result
#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Deserialize))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct DataResult {
    // DR Result
    /// Semantic Version String
    pub version: Version,

    /// Data Request Identifier
    pub dr_id:        String,
    /// Block Height at which data request was finalized
    pub block_height: u64,
    /// Exit code of Tally WASM binary execution
    pub exit_code:    u8,
    pub gas_used:     U128,
    /// Result from Tally WASM binary execution
    pub result:       Bytes,

    // Fields from Data Request Execution
    /// Payback address set by the relayer
    pub payback_address: Bytes,
    /// Payload set by SEDA Protocol (e.g. OEV-enabled data requests)
    pub seda_payload:    Bytes,

    ///  Represents Whether or not the result was in consensus or not (≥ 66%)
    pub consensus: bool,
}

impl HashSelf for DataResult {
    fn hash(&self) -> Hash {
        let mut result_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        result_hasher.update(&self.result.to_base64());
        #[cfg(not(feature = "cosmwasm"))]
        result_hasher.update(&self.result);
        let result_hash = result_hasher.finalize();

        let mut seda_payload_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        #[cfg(feature = "cosmwasm")]
        seda_payload_hasher.update(&self.seda_payload.to_base64());
        #[cfg(not(feature = "cosmwasm"))]
        seda_payload_hasher.update(&self.seda_payload);
        let seda_payload_hash = seda_payload_hasher.finalize();

        let mut hasher = Keccak256::new();
        hasher.update(self.version.hash());
        hasher.update(&self.dr_id);
        hasher.update(self.block_height.to_be_bytes());
        hasher.update(self.exit_code.to_be_bytes());
        hasher.update(result_hash);
        #[cfg(feature = "cosmwasm")]
        hasher.update(self.gas_used.to_be_bytes());
        #[cfg(not(feature = "cosmwasm"))]
        hasher.update(
            self.gas_used
                .parse::<u128>()
                .expect("`gas_used` should be parseable to u128")
                .to_be_bytes(),
        );
        #[cfg(feature = "cosmwasm")]
        hasher.update(&self.payback_address.to_base64());
        #[cfg(not(feature = "cosmwasm"))]
        hasher.update(&self.payback_address);
        hasher.update(seda_payload_hash);
        hasher.update([self.consensus.into()]);
        hasher.finalize().into()
    }
}

/// A revealed data request result that is hashed and signed by the executor
#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Deserialize, Clone))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct RevealBody {
    pub salt:      String,
    pub exit_code: u8,
    pub gas_used:  U128,
    pub reveal:    Bytes,
}

impl HashSelf for RevealBody {
    fn hash(&self) -> Hash {
        let mut reveal_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        reveal_hasher.update(&self.reveal.to_base64());
        #[cfg(not(feature = "cosmwasm"))]
        reveal_hasher.update(&self.reveal);
        let reveal_hash = reveal_hasher.finalize();

        let mut hasher = Keccak256::new();
        hasher.update(&self.salt);
        hasher.update(self.exit_code.to_be_bytes());
        #[cfg(feature = "cosmwasm")]
        hasher.update(self.gas_used.to_be_bytes());
        #[cfg(not(feature = "cosmwasm"))]
        hasher.update(
            self.gas_used
                .parse::<u128>()
                .expect("`gas_used` should be parseable to u128")
                .to_be_bytes(),
        );
        hasher.update(reveal_hash);
        hasher.finalize().into()
    }
}

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct PostDataRequestArgs {
    pub version:            Version,
    pub dr_binary_id:       String,
    pub dr_inputs:          Bytes,
    pub tally_binary_id:    String,
    pub tally_inputs:       Bytes,
    pub replication_factor: u16,
    pub gas_price:          U128,
    pub gas_limit:          U128,
    pub memo:               Bytes,
}

impl HashSelf for PostDataRequestArgs {
    fn hash(&self) -> Hash {
        // hash non-fixed-length inputs
        let mut dr_inputs_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        dr_inputs_hasher.update(&self.dr_inputs.to_base64());
        #[cfg(not(feature = "cosmwasm"))]
        dr_inputs_hasher.update(&self.dr_inputs);
        let dr_inputs_hash = dr_inputs_hasher.finalize();

        let mut tally_inputs_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        tally_inputs_hasher.update(&self.tally_inputs.to_base64());
        #[cfg(not(feature = "cosmwasm"))]
        tally_inputs_hasher.update(&self.tally_inputs);
        let tally_inputs_hash = tally_inputs_hasher.finalize();

        let mut memo_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        memo_hasher.update(&self.memo.to_base64());
        #[cfg(not(feature = "cosmwasm"))]
        memo_hasher.update(&self.memo);
        let memo_hash = memo_hasher.finalize();

        // hash data request
        let mut dr_hasher = Keccak256::new();
        dr_hasher.update(self.version.hash());
        // I don't think we should decode to hash... expensive in cosmwasm no?
        dr_hasher.update(&self.dr_binary_id);
        dr_hasher.update(dr_inputs_hash);
        dr_hasher.update(&self.tally_binary_id);
        dr_hasher.update(tally_inputs_hash);
        dr_hasher.update(self.replication_factor.to_be_bytes());
        #[cfg(feature = "cosmwasm")]
        dr_hasher.update(self.gas_price.to_be_bytes());
        #[cfg(not(feature = "cosmwasm"))]
        dr_hasher.update(
            self.gas_price
                .parse::<u128>()
                .expect("`gas_price` should be parseable to u128")
                .to_be_bytes(),
        );
        #[cfg(feature = "cosmwasm")]
        dr_hasher.update(self.gas_limit.to_be_bytes());
        #[cfg(not(feature = "cosmwasm"))]
        dr_hasher.update(
            self.gas_price
                .parse::<u128>()
                .expect("`gas_limit` should be parseable to u128")
                .to_be_bytes(),
        );
        dr_hasher.update(memo_hash);
        dr_hasher.finalize().into()
    }
}
