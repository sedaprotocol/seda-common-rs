use serde_json::json;

use super::{execute::*, ExecuteMsg, U128};
use crate::msgs::assert_json_ok;

#[test]
fn json_stake() {
    let serialized_no_memo = json!({
      "stake": {
        "memo": null,
        "proof": "proof",
        "public_key": "public",
      }
    });
    let msg_no_memo: ExecuteMsg = stake::Execute {
        public_key: "public".to_string(),
        proof:      "proof".to_string(),
        memo:       None,
    }
    .into();
    assert_json_ok(msg_no_memo, serialized_no_memo);

    #[cfg(not(feature = "cosmwasm"))]
    let memo = "memo".to_string();
    #[cfg(feature = "cosmwasm")]
    let memo = "memo".as_bytes().into();

    let serialized_with_memo = json!({
        "stake": {
            "public_key": "public",
            "proof": "proof",
            "memo": memo,
        }
    });
    let msg_with_memo: ExecuteMsg = stake::Execute {
        public_key: "public".to_string(),
        proof:      "proof".to_string(),
        memo:       Some(memo),
    }
    .into();
    assert_json_ok(msg_with_memo, serialized_with_memo);
}

#[test]
fn json_unstake() {
    #[cfg(not(feature = "cosmwasm"))]
    let amount: U128 = 0;
    #[cfg(feature = "cosmwasm")]
    let amount: U128 = 0u128.into();
    let serialized = json!({
      "unstake": {
        "proof": "proof",
        "public_key": "public",
        "amount": amount.to_string(),
      }
    });
    let msg: ExecuteMsg = unstake::Execute {
        public_key: "public".to_string(),
        proof: "proof".to_string(),
        amount,
    }
    .into();
    assert_json_ok(msg, serialized);
}

#[test]
fn json_withdraw() {
    #[cfg(not(feature = "cosmwasm"))]
    let amount: U128 = 0;
    #[cfg(feature = "cosmwasm")]
    let amount: U128 = 0u128.into();
    let serialized = json!({
      "withdraw": {
        "proof": "proof",
        "public_key": "public",
        "amount": amount.to_string(),
      }
    });
    let msg: ExecuteMsg = withdraw::Execute {
        public_key: "public".to_string(),
        proof: "proof".to_string(),
        amount,
    }
    .into();
    assert_json_ok(msg, serialized);
}
