use semver::Version;
use serde_json::json;

#[cfg(feature = "cosmwasm")]
use super::Bytes;
use super::{DataResult, SudoMsg, sudo::*};
use crate::msgs::*;

#[test]
fn json_post_result() {
    #[cfg(not(feature = "cosmwasm"))]
    let result_bytes = "result".to_string();
    #[cfg(feature = "cosmwasm")]
    let result_bytes: Bytes = "result".as_bytes().into();

    let gas_used = 100u128.into();

    #[cfg(not(feature = "cosmwasm"))]
    let seda_payload = "seda_payload".to_string();
    #[cfg(feature = "cosmwasm")]
    let seda_payload: Bytes = "seda_payload".as_bytes().into();

    #[cfg(not(feature = "cosmwasm"))]
    let payback_address = "payback_address".to_string();
    #[cfg(feature = "cosmwasm")]
    let payback_address: Bytes = "payback_address".as_bytes().into();
    let result = DataResult {
        version: Version::new(1, 0, 0),
        dr_id: "dr_id".to_string(),
        block_height: 100,
        exit_code: 0,
        gas_used,
        result: result_bytes.clone(),
        payback_address: payback_address.clone(),
        seda_payload: seda_payload.clone(),
        consensus: false,
    };
    let expected_json = json!({
    "post_data_result": {
      "dr_id": "dr_id",
      "result": {
        "version": "1.0.0",
        "dr_id": "dr_id",
        "block_height": 100,
        "exit_code": 0,
        "gas_used": gas_used.to_string(),
        "result": result_bytes,
        "payback_address": payback_address,
        "seda_payload": seda_payload,
        "consensus": false,
      },
      "exit_code": 0,
    }
    });
    let msg: SudoMsg = PostResult {
        dr_id: "dr_id".to_string(),
        result,
        exit_code: 0,
    }
    .into();
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}

#[test]
fn json_remove_timed_out_data_requests() {
    let expected_json = json!("remove_timed_out_data_requests");
    let msg: SudoMsg = data_requests::sudo::SudoMsg::RemoveTimedOutDataRequests.into();
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}
