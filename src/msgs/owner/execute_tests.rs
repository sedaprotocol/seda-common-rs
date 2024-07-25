use serde_json::json;

use super::{execute::*, ExecuteMsg};
use crate::msgs::assert_json_ok;

#[test]
fn json_accept_ownership() {
    let expected_json = json!(
    {
        "accept_ownership": {}
    });
    let msg: ExecuteMsg = accept_ownership::Execute {}.into();
    assert_json_ok(msg, expected_json);
}

#[test]
fn json_add_to_allowlist() {
    let expected_json = json!({
      "add_to_allowlist": {
        "public_key": "public_key"
      }
    });
    let msg: ExecuteMsg = add_to_allowlist::Execute {
        public_key: "public_key".to_string(),
    }
    .into();
    assert_json_ok(msg, expected_json);
}

#[test]
fn json_remove_from_allowlist() {
    let expected_json = json!({
      "remove_from_allowlist": {
        "public_key": "public_key"
      }
    });
    let msg: ExecuteMsg = remove_from_allowlist::Execute {
        public_key: "public_key".to_string(),
    }
    .into();
    assert_json_ok(msg, expected_json);
}

#[test]
fn json_transfer_ownership() {
    let expected_json = json!({
      "transfer_ownership": {
        "new_owner": "new_owner"
      }
    });
    let msg: ExecuteMsg = transfer_ownership::Execute {
        new_owner: "new_owner".to_string(),
    }
    .into();
    assert_json_ok(msg, expected_json);
}
