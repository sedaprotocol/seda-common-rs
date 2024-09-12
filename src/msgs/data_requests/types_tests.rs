use std::collections::HashMap;

use serde_json::json;

#[cfg(feature = "cosmwasm")]
use super::Bytes;
use super::{DataRequest, DataResult, HashSelf, PostDataRequestArgs, RevealBody, U128};
use crate::msgs::*;

#[test]
fn json_data_request() {
    let id = "id".to_string();
    let version = "1.0.0".to_string();
    let dr_binary_id = "dr_binary_id".to_string();
    #[cfg(not(feature = "cosmwasm"))]
    let dr_inputs = "dr_inputs".to_string();
    #[cfg(feature = "cosmwasm")]
    let dr_inputs: Bytes = "dr_inputs".as_bytes().into();
    let tally_binary_id = "tally_binary_id".to_string();
    #[cfg(not(feature = "cosmwasm"))]
    let tally_inputs = "tally_inputs".to_string();
    #[cfg(feature = "cosmwasm")]
    let tally_inputs: Bytes = "tally_inputs".as_bytes().into();
    let replication_factor = 1;
    #[cfg(not(feature = "cosmwasm"))]
    let consensus_filter = "consensus_filter".to_string();
    #[cfg(feature = "cosmwasm")]
    let consensus_filter: Bytes = "consensus_filter".as_bytes().into();
    let gas_price: U128 = 1u128.into();
    let gas_limit: U128 = 1u128.into();
    #[cfg(not(feature = "cosmwasm"))]
    let memo = "memo".to_string();
    #[cfg(feature = "cosmwasm")]
    let memo: Bytes = "memo".as_bytes().into();
    #[cfg(not(feature = "cosmwasm"))]
    let payback_address = "payback_address".to_string();
    #[cfg(feature = "cosmwasm")]
    let payback_address: Bytes = "payback_address".as_bytes().into();
    #[cfg(not(feature = "cosmwasm"))]
    let seda_payload = "seda_payload".to_string();
    #[cfg(feature = "cosmwasm")]
    let seda_payload: Bytes = "seda_payload".as_bytes().into();
    let commits = HashMap::from([("key".to_string(), "value".hash())]);
    let reveals = HashMap::new();
    let height = 1;

    let expected_json = json!({
      "id": id,
      "version": version,
      "dr_binary_id": dr_binary_id,
      "dr_inputs": dr_inputs,
      "tally_binary_id": tally_binary_id,
      "tally_inputs": tally_inputs,
      "replication_factor": replication_factor,
      "consensus_filter": consensus_filter,
      "gas_price": gas_price,
      "gas_limit": gas_limit,
      "memo": memo,
      "payback_address": payback_address,
      "seda_payload": seda_payload,
      "commits": commits,
      "reveals": {},
      "height": height,
    });

    let msg = DataRequest {
        id,
        version: version.parse().unwrap(),
        dr_binary_id,
        dr_inputs,
        tally_binary_id,
        tally_inputs,
        replication_factor,
        consensus_filter,
        gas_price,
        gas_limit,
        memo,
        payback_address,
        seda_payload,
        commits,
        reveals,
        height,
    };

    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}

#[test]
fn json_data_result() {
    let version = "1.0.0".to_string();
    let dr_id = "dr_id".to_string();
    let consensus = true;
    let exit_code = 0;
    #[cfg(not(feature = "cosmwasm"))]
    let result = "result".to_string();
    #[cfg(feature = "cosmwasm")]
    let result: Bytes = "result".as_bytes().into();
    let block_height = 1;
    let gas_used: U128 = 1u128.into();
    #[cfg(not(feature = "cosmwasm"))]
    let payback_address = "payback_address".to_string();
    #[cfg(feature = "cosmwasm")]
    let payback_address: Bytes = "payback_address".as_bytes().into();
    #[cfg(not(feature = "cosmwasm"))]
    let seda_payload = "seda_payload".to_string();
    #[cfg(feature = "cosmwasm")]
    let seda_payload: Bytes = "seda_payload".as_bytes().into();

    let expected_json = json!({
      "version": version,
      "dr_id": dr_id,
      "consensus": consensus,
      "exit_code": exit_code,
      "result": result,
      "block_height": block_height,
      "gas_used": gas_used,
      "payback_address": payback_address,
      "seda_payload": seda_payload,
    });
    let msg = DataResult {
        version: version.parse().unwrap(),
        dr_id,
        consensus,
        exit_code,
        result,
        block_height,
        gas_used,
        payback_address,
        seda_payload,
    };

    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}

#[test]
fn json_reveal_body() {
    let id = "id".to_string();
    let salt = "salt".to_string();
    let exit_code = 0;
    let gas_used: U128 = 1u128.into();
    #[cfg(not(feature = "cosmwasm"))]
    let reveal = "reveal".to_string();
    #[cfg(feature = "cosmwasm")]
    let reveal: Bytes = "reveal".as_bytes().into();
    let proxy_public_keys = vec!["key1".to_string(), "key2".to_string()];

    let expected_json = json!({
      "salt": salt,
      "exit_code": exit_code,
      "gas_used": gas_used,
      "reveal": reveal,
      "proxy_public_keys": proxy_public_keys,
    });

    let msg = RevealBody {
        id,
        salt,
        exit_code,
        gas_used,
        reveal,
        proxy_public_keys,
    };

    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}

// pub struct PostDataRequestArgs {
// 	pub version:            Version,
// 	pub dr_binary_id:       String,
// 	pub dr_inputs:          Bytes,
// 	pub tally_binary_id:    String,
// 	pub tally_inputs:       Bytes,
// 	pub replication_factor: u16,
// 	pub consensus_filter:   Bytes,
// 	pub gas_price:          U128,
// 	pub gas_limit:          U128,
// 	pub memo:               Bytes,
// }

#[test]
fn json_post_data_request_args() {
    let version = "1.0.0".to_string();
    let dr_binary_id = "dr_binary_id".to_string();
    #[cfg(not(feature = "cosmwasm"))]
    let dr_inputs = "dr_inputs".to_string();
    #[cfg(feature = "cosmwasm")]
    let dr_inputs: Bytes = "dr_inputs".as_bytes().into();
    let tally_binary_id = "tally_binary_id".to_string();
    #[cfg(not(feature = "cosmwasm"))]
    let tally_inputs = "tally_inputs".to_string();
    #[cfg(feature = "cosmwasm")]
    let tally_inputs: Bytes = "tally_inputs".as_bytes().into();
    let replication_factor = 1;
    #[cfg(not(feature = "cosmwasm"))]
    let consensus_filter = "consensus_filter".to_string();
    #[cfg(feature = "cosmwasm")]
    let consensus_filter: Bytes = "consensus_filter".as_bytes().into();
    let gas_price: U128 = 1u128.into();
    let gas_limit: U128 = 1u128.into();
    #[cfg(not(feature = "cosmwasm"))]
    let memo = "memo".to_string();
    #[cfg(feature = "cosmwasm")]
    let memo: Bytes = "memo".as_bytes().into();

    let expected_json = json!({
        "version": version,
        "dr_binary_id": dr_binary_id,
        "dr_inputs": dr_inputs,
        "tally_binary_id": tally_binary_id,
        "tally_inputs": tally_inputs,
        "replication_factor": replication_factor,
        "consensus_filter": consensus_filter,
        "gas_price": gas_price,
        "gas_limit": gas_limit,
        "memo": memo,
    });

    let msg = PostDataRequestArgs {
        version: version.parse().unwrap(),
        dr_binary_id,
        dr_inputs,
        tally_binary_id,
        tally_inputs,
        replication_factor,
        consensus_filter,
        gas_price,
        gas_limit,
        memo,
    };

    assert_json_ser(msg, expected_json);
}
