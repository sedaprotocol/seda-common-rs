use serde_json::json;

use super::{query::QueryMsg as StakingQueryMsg, QueryMsg};
use crate::msgs::assert_json_ok;

#[test]
fn json_get_staker() {
    let expected_json = json!({
      "get_staker": {
        "public_key": "public_key"
      }
    });
    let msg: QueryMsg = StakingQueryMsg::GetStaker {
        public_key: "public_key".to_string(),
    }
    .into();
    assert_json_ok(&msg, &expected_json);
}

#[test]
fn json_get_account_seq() {
    let expected_json = json!({
      "get_account_seq": {
        "public_key": "public_key"
      }
    });
    let msg: QueryMsg = StakingQueryMsg::GetAccountSeq {
        public_key: "public_key".to_string(),
    }
    .into();
    assert_json_ok(&msg, &expected_json);
}

#[test]
fn json_get_staker_and_seq() {
    let expected_json = json!({
      "get_staker_and_seq": {
        "public_key": "public_key"
      }
    });
    let msg: QueryMsg = StakingQueryMsg::GetStakerAndSeq {
        public_key: "public_key".to_string(),
    }
    .into();
    assert_json_ok(&msg, &expected_json);
}

#[test]
fn json_is_executor_eligible() {
    let expected_json = json!({
    "is_executor_eligible": {
      "public_key": "public_key"
    }
    });
    let msg: QueryMsg = StakingQueryMsg::IsExecutorEligible {
        public_key: "public_key".to_string(),
    }
    .into();
    assert_json_ok(&msg, &expected_json);
}

#[test]
fn json_get_staking_config() {
    let expected_json = json!({
      "get_staking_config": {}
    });
    let msg: QueryMsg = StakingQueryMsg::GetStakingConfig {}.into();
    assert_json_ok(&msg, &expected_json);
}
