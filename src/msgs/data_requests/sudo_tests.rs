use semver::Version;
use serde_json::json;

use super::{sudo::*, DataResult, SudoMsg};
#[cfg(feature = "cosmwasm")]
use super::{Bytes, U128};
use crate::msgs::assert_json_ok;

#[test]
fn json_post_result() {
    #[cfg(not(feature = "cosmwasm"))]
    let result_bytes = "result".to_string();
    #[cfg(feature = "cosmwasm")]
    let result_bytes: Bytes = "result".as_bytes().into();

    #[cfg(not(feature = "cosmwasm"))]
    let gas_used = 100;
    #[cfg(feature = "cosmwasm")]
    let gas_used: U128 = 100u128.into();

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
    assert_json_ok(&msg, &expected_json);
}
