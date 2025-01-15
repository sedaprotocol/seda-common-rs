use std::collections::HashMap;

use serde_json::json;

use super::sudo::*;
#[cfg(feature = "cosmwasm")]
use crate::msgs::assert_json_deser;
#[cfg(not(feature = "cosmwasm"))]
use crate::msgs::assert_json_ser;
use crate::{msgs, types::Bytes};

#[test]
fn json_remove_requests() {
    #[cfg(not(feature = "cosmwasm"))]
    let to: Bytes = "to".to_string();
    #[cfg(feature = "cosmwasm")]
    let to: Bytes = "to".as_bytes().into();

    let expected_json = json!({
    "remove_data_requests": {
        "requests": {
            "dr_id1": {
                "messages": [
                    {
                        "burn": {
                            "amount": "100"
                        }
                    },
                    {
                        "executor_reward": {
                            "amount": "100",
                            "identity": "identity"
                        }
                    },
                    {
                        "data_proxy_reward": {
                            "amount": "100",
                            "to": to
                        },
                    }
                ],
                "refund_type": "remainder"
            },
            "dr_id2": {
                "messages": [],
                "refund_type": "timeout"
            },
            "dr_id3": {
                "messages": [],
                "refund_type": "no_consensus"
            }
        }
    }});
    let mut requests = HashMap::new();
    requests.insert(
        "dr_id1".to_string(),
        DistributionMessages {
            messages:    vec![
                DistributionMessage::Burn(DistributionBurn { amount: 100u128.into() }),
                DistributionMessage::ExecutorReward(DistributionExecutorReward {
                    amount:   100u128.into(),
                    identity: "identity".to_string(),
                }),
                DistributionMessage::DataProxyReward(DistributionDataProxyReward {
                    amount: 100u128.into(),
                    to,
                }),
            ],
            refund_type: RefundType::Remainder,
        },
    );
    requests.insert(
        "dr_id2".to_string(),
        DistributionMessages {
            messages:    vec![],
            refund_type: RefundType::Timeout,
        },
    );
    requests.insert(
        "dr_id3".to_string(),
        DistributionMessages {
            messages:    vec![],
            refund_type: RefundType::NoConsensus,
        },
    );
    let msg: msgs::SudoMsg = remove_requests::Sudo { requests }.into();
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
