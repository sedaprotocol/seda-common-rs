use serde_json::json;

#[cfg(feature = "cosmwasm")]
use super::Bytes;
use super::{sudo::*, SudoMsg};
use crate::msgs::*;

#[test]
fn json_post_result() {
    let expected_json = json!({
    "post_data_result": {
      "dr_id": "dr_id",
    }
    });
    let msg: SudoMsg = PostResult {
        dr_id: "dr_id".to_string(),
    }
    .into();
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}

#[test]
fn json_remove_timed_out_data_requests() {
    let expected_json = json!({"expire_data_requests": {}});
    let msg: SudoMsg = expire_data_requests::Sudo {}.into();
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}
