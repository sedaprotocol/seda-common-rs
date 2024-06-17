use serde_json::json;

use super::{query::QueryMsg as OwnerQueryMsg, QueryMsg};
use crate::msgs::assert_json_ok;

#[test]
fn json_get_owner() {
    let expected_json = json!(
    {
      "get_owner": {}
    });
    let msg: QueryMsg = OwnerQueryMsg::GetOwner {}.into();
    assert_json_ok(&msg, &expected_json);
}

#[test]
fn json_get_pending_owner() {
    let expected_json = json!(
    {
        "get_pending_owner": {}
    });
    let msg: QueryMsg = OwnerQueryMsg::GetPendingOwner {}.into();
    assert_json_ok(&msg, &expected_json);
}
