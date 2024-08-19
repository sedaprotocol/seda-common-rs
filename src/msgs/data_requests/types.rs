#[cfg(not(feature = "cosmwasm"))]
use base64::{prelude::BASE64_STANDARD, Engine};
#[cfg(feature = "cosmwasm")]
use cw_storage_plus::{Key, Prefixer, PrimaryKey};
use semver::Version;
use sha3::{Digest, Keccak256};

use super::*;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Debug, PartialEq))]
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
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Deserialize, Clone, Debug, PartialEq))]
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
    /// Filter applied before tally execution
    pub consensus_filter:   Bytes,
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
    pub commits:         HashMap<String, Hash>,
    /// Reveals submitted by executors
    pub reveals:         HashMap<String, RevealBody>,

    /// The height data request was posted. Used for commitment.
    pub height: u64,
}

impl DataRequest {
    pub fn has_committer(&self, public_key: &str) -> bool {
        self.commits.contains_key(public_key)
    }

    pub fn get_commitment(&self, public_key: &str) -> Option<&Hash> {
        self.commits.get(public_key)
    }

    pub fn has_revealer(&self, public_key: &str) -> bool {
        self.reveals.contains_key(public_key)
    }

    pub fn reveal_started(&self) -> bool {
        self.commits.len() >= self.replication_factor as usize
    }

    pub fn is_tallying(&self) -> bool {
        self.reveals.len() >= self.replication_factor as usize
    }

    pub fn get_reveal(&self, public_key: &str) -> Option<&RevealBody> {
        self.reveals.get(public_key)
    }
}

/// Represents a resolved data result
#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Deserialize, Debug, PartialEq))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct DataResult {
    // DR Result
    /// Semantic Version String
    pub version: Version,

    /// Data Request Identifier
    pub dr_id:        String,
    ///  Represents whether the result was in consensus or not (≥ 66%)
    pub consensus:    bool,
    /// Exit code of Tally WASM binary execution
    pub exit_code:    u8,
    /// Result from Tally WASM binary execution
    pub result:       Bytes,
    /// Block Height at which data request was finalized
    pub block_height: u64,
    /// Gas used by the complete data request execution
    #[cfg_attr(not(feature = "cosmwasm"), serde(serialize_with = "crate::types::serialize_as_str"))]
    pub gas_used:     U128,

    // Fields from Data Request Execution
    /// Payback address set by the relayer
    pub payback_address: Bytes,
    /// Payload set by SEDA Protocol (e.g. OEV-enabled data requests)
    pub seda_payload:    Bytes,
}

impl TryHashSelf for DataResult {
    fn try_hash(&self) -> Result<Hash> {
        let version = self.version.hash();
        let dr_id = hex::decode(&self.dr_id)?;
        let consensus: [u8; 1] = [self.consensus.into()];
        let exit_code = self.exit_code.to_be_bytes();

        let mut result_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        result_hasher.update(self.result.as_slice());
        #[cfg(not(feature = "cosmwasm"))]
        result_hasher.update(BASE64_STANDARD.decode(&self.result)?);
        let result_hash = result_hasher.finalize();

        let block_height = self.block_height.to_be_bytes();
        #[cfg(feature = "cosmwasm")]
        let gas_used = self.gas_used.to_be_bytes();
        #[cfg(not(feature = "cosmwasm"))]
        let gas_used = self
            .gas_used
            .parse::<u128>()
            .expect("gas used should be parseable to u128")
            .to_be_bytes();

        let mut payback_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        payback_hasher.update(self.payback_address.as_slice());
        #[cfg(not(feature = "cosmwasm"))]
        payback_hasher.update(BASE64_STANDARD.decode(&self.payback_address)?);
        let seda_payback_hash = payback_hasher.finalize();

        let mut seda_payload_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        seda_payload_hasher.update(self.seda_payload.as_slice());
        #[cfg(not(feature = "cosmwasm"))]
        seda_payload_hasher.update(BASE64_STANDARD.decode(&self.seda_payload)?);
        let seda_payload_hash = seda_payload_hasher.finalize();

        let bytes = [
            version.as_slice(),
            &dr_id,
            &consensus,
            &exit_code,
            &result_hash,
            &block_height,
            &gas_used,
            &seda_payback_hash,
            &seda_payload_hash,
        ]
        .concat();

        Ok(Keccak256::digest(bytes).into())
    }
}

/// A revealed data request result that is hashed and signed by the executor
#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Deserialize, Clone, Debug, PartialEq))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct RevealBody {
    pub salt:      String,
    pub exit_code: u8,
    #[cfg_attr(not(feature = "cosmwasm"), serde(serialize_with = "crate::types::serialize_as_str"))]
    pub gas_used:  U128,
    pub reveal:    Bytes,
}

impl TryHashSelf for RevealBody {
    fn try_hash(&self) -> Result<Hash> {
        let mut reveal_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        reveal_hasher.update(self.reveal.as_slice());
        #[cfg(not(feature = "cosmwasm"))]
        reveal_hasher.update(BASE64_STANDARD.decode(&self.reveal)?);
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

        Ok(hasher.finalize().into())
    }
}

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Debug, PartialEq))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct PostDataRequestArgs {
    pub version:            Version,
    pub dr_binary_id:       String,
    pub dr_inputs:          Bytes,
    pub tally_binary_id:    String,
    pub tally_inputs:       Bytes,
    pub replication_factor: u16,
    pub consensus_filter:   Bytes,
    #[cfg_attr(not(feature = "cosmwasm"), serde(serialize_with = "crate::types::serialize_as_str"))]
    pub gas_price:          U128,
    #[cfg_attr(not(feature = "cosmwasm"), serde(serialize_with = "crate::types::serialize_as_str"))]
    pub gas_limit:          U128,
    pub memo:               Bytes,
}

impl TryHashSelf for PostDataRequestArgs {
    fn try_hash(&self) -> Result<Hash> {
        // hash non-fixed-length inputs
        let mut dr_inputs_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        dr_inputs_hasher.update(self.dr_inputs.as_slice());
        #[cfg(not(feature = "cosmwasm"))]
        dr_inputs_hasher.update(BASE64_STANDARD.decode(&self.dr_inputs)?);
        let dr_inputs_hash = dr_inputs_hasher.finalize();

        let mut tally_inputs_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        tally_inputs_hasher.update(self.tally_inputs.as_slice());
        #[cfg(not(feature = "cosmwasm"))]
        tally_inputs_hasher.update(BASE64_STANDARD.decode(&self.tally_inputs)?);
        let tally_inputs_hash = tally_inputs_hasher.finalize();

        let mut consensus_filter_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        consensus_filter_hasher.update(self.consensus_filter.as_slice());
        #[cfg(not(feature = "cosmwasm"))]
        consensus_filter_hasher.update(BASE64_STANDARD.decode(&self.consensus_filter)?);
        let consensus_filter_hash = consensus_filter_hasher.finalize();

        let mut memo_hasher = Keccak256::new();
        #[cfg(feature = "cosmwasm")]
        memo_hasher.update(self.memo.as_slice());
        #[cfg(not(feature = "cosmwasm"))]
        memo_hasher.update(BASE64_STANDARD.decode(&self.memo)?);
        let memo_hash = memo_hasher.finalize();

        // hash data request
        let mut dr_hasher = Keccak256::new();
        dr_hasher.update(self.version.hash());
        // I don't think we should decode to hash... expensive in cosmwasm no?
        dr_hasher.update(hex::decode(&self.dr_binary_id)?);
        dr_hasher.update(dr_inputs_hash);
        dr_hasher.update(hex::decode(&self.tally_binary_id)?);
        dr_hasher.update(tally_inputs_hash);
        dr_hasher.update(self.replication_factor.to_be_bytes());
        dr_hasher.update(consensus_filter_hash);
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
            self.gas_limit
                .parse::<u128>()
                .expect("`gas_limit` should be parseable to u128")
                .to_be_bytes(),
        );
        dr_hasher.update(memo_hash);

        Ok(dr_hasher.finalize().into())
    }
}
