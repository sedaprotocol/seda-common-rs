use serde_json::json;

use super::{query::QueryMsg as OwnerQueryMsg, QueryMsg};
use crate::msgs::*;

#[test]
fn json_get_owner() {
    let expected_json = json!(
    {
      "get_owner": {}
    });
    let msg: QueryMsg = OwnerQueryMsg::GetOwner {}.into();
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}

#[test]
fn json_get_pending_owner() {
    let expected_json = json!(
    {
        "get_pending_owner": {}
    });
    let msg: QueryMsg = OwnerQueryMsg::GetPendingOwner {}.into();
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}
