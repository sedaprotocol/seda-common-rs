use semver::Version;
use serde_json::json;

#[cfg(feature = "cosmwasm")]
use super::Bytes;
use super::{execute::*, ExecuteMsg, PostDataRequestArgs, RevealBody};
use crate::msgs::*;

#[test]
fn json_commit_result() {
    let expected_json = json!({
      "commit_data_result": {
        "dr_id": "dr_id",
        "commitment": "commitment",
        "public_key": "public_key",
        "proof": "proof"
      }
    });
    let msg: ExecuteMsg = commit_result::Execute {
        dr_id:      "dr_id".to_string(),
        commitment: "commitment".to_string(),
        public_key: "public_key".to_string(),
        proof:      "proof".to_string(),
    }
    .into();
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}

#[test]
fn json_post_request() {
    #[cfg(not(feature = "cosmwasm"))]
    let dr_inputs = "dr_inputs".to_string();
    #[cfg(feature = "cosmwasm")]
    let dr_inputs: Bytes = "dr_inputs".as_bytes().into();

    #[cfg(not(feature = "cosmwasm"))]
    let tally_inputs = "tally_inputs".to_string();
    #[cfg(feature = "cosmwasm")]
    let tally_inputs: Bytes = "tally_inputs".as_bytes().into();

    #[cfg(not(feature = "cosmwasm"))]
    let consensus_filter = "consensus_filter".to_string();
    #[cfg(feature = "cosmwasm")]
    let consensus_filter: Bytes = "consensus_filter".as_bytes().into();

    let gas_price = 100u128.into();

    let gas_limit = 100u128.into();

    #[cfg(not(feature = "cosmwasm"))]
    let memo = "memo".to_string();
    #[cfg(feature = "cosmwasm")]
    let memo: Bytes = "memo".as_bytes().into();

    #[cfg(not(feature = "cosmwasm"))]
    let seda_payload = "seda_payload".to_string();
    #[cfg(feature = "cosmwasm")]
    let seda_payload: Bytes = "seda_payload".as_bytes().into();

    #[cfg(not(feature = "cosmwasm"))]
    let payback_address = "payback_address".to_string();
    #[cfg(feature = "cosmwasm")]
    let payback_address: Bytes = "payback_address".as_bytes().into();

    let args = PostDataRequestArgs {
        version: Version::new(1, 0, 0),
        dr_binary_id: "dr_binary_id".to_string(),
        dr_inputs: dr_inputs.clone(),
        tally_binary_id: "tally_binary_id".to_string(),
        tally_inputs: tally_inputs.clone(),
        replication_factor: 1,
        consensus_filter: consensus_filter.clone(),
        gas_price,
        gas_limit,
        memo: memo.clone(),
    };
    let expected_json = json!({
      "post_data_request": {
        "posted_dr": {
          "version": "1.0.0",
          "dr_binary_id": "dr_binary_id",
          "dr_inputs": dr_inputs,
          "tally_binary_id": "tally_binary_id",
          "tally_inputs": tally_inputs,
          "replication_factor": 1,
          "consensus_filter": consensus_filter,
          "gas_price": gas_price.to_string(),
          "gas_limit": gas_limit.to_string(),
          "memo": memo
        },
        "seda_payload": seda_payload,
        "payback_address": payback_address,
      }
    });
    let msg: ExecuteMsg = post_request::Execute {
        posted_dr: args,
        seda_payload,
        payback_address,
    }
    .into();
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}

#[test]
fn json_reveal_result() {
    let gas_used = 100u128.into();

    #[cfg(not(feature = "cosmwasm"))]
    let reveal = "reveal".to_string();
    #[cfg(feature = "cosmwasm")]
    let reveal: Bytes = "reveal".as_bytes().into();

    let reveal_body = RevealBody {
        salt: "salt".to_string(),
        exit_code: 0,
        gas_used,
        reveal: reveal.clone(),
        proxy_public_keys: vec!["proxy_public_key".to_string()],
    };
    let expected_json = json!({
      "reveal_data_result": {
        "dr_id": "dr_id",
        "reveal_body": {
          "salt": "salt",
          "exit_code": 0,
          "gas_used": gas_used.to_string(),
          "reveal": reveal,
          "proxy_public_keys": ["proxy_public_key"]
        },
        "public_key": "public_key",
        "proof": "proof",
        "stderr": vec!["error".to_string()],
        "stdout": vec!["some-output".to_string()],
      }
    });
    let msg: ExecuteMsg = reveal_result::Execute {
        dr_id: "dr_id".to_string(),
        reveal_body,
        public_key: "public_key".to_string(),
        proof: "proof".to_string(),
        stderr: vec!["error".to_string()],
        stdout: vec!["some-output".to_string()],
    }
    .into();
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}
