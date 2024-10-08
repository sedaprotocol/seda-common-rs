use serde_json::json;

use super::sudo::*;
use crate::msgs;
#[cfg(feature = "cosmwasm")]
use crate::msgs::assert_json_deser;
#[cfg(not(feature = "cosmwasm"))]
use crate::msgs::assert_json_ser;

#[test]
fn json_remove_request() {
    let expected_json = json!({
    "remove_data_request": {
      "dr_id": "dr_id",
    }
    });
    let msg: msgs::SudoMsg = RemoveDataRequest {
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
    let msg: msgs::SudoMsg = expire_data_requests::Sudo {}.into();
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}
