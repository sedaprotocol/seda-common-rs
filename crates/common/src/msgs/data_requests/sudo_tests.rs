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
                        "kind": {
                            "burn": {
                                "amount": "100"
                            }
                        },
                        "type": "executor_reward"
                    },
                    {
                        "kind": {
                            "send": {
                                "amount": "100",
                                "to": to
                            }
                        },
                        "type": "executor_reward"
                    }
                ],
                "refund_type": "remainder_refund"
            },
        }
    }
    });
    let mut requests = HashMap::new();
    requests.insert(
        "dr_id1".to_string(),
        DistributionMessages {
            messages:    vec![
                DistributionMessage {
                    kind:  DistributionKind::Burn(DistributionBurn { amount: 100u128.into() }),
                    type_: DistributionType::ExecutorReward,
                },
                DistributionMessage {
                    kind:  DistributionKind::Send(DistributionSend {
                        amount: 100u128.into(),
                        to,
                    }),
                    type_: DistributionType::ExecutorReward,
                },
            ],
            refund_type: DistributionType::RemainderRefund,
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
