#[cfg(cosmwasm)]
use serde_json::to_string;
use serde_json::to_value;

// assert that the msg M serializes to the expected json J
// if cosmwasm feature is enabled, also assert that we can deserialize the json back to the msg
#[track_caller]
pub fn assert_json_ok<M, J>(msg: &M, json: &J)
where
    M: serde::Serialize,
    J: serde::Serialize,
{
    let msg_json = to_value(msg).unwrap();
    let expected_json = to_value(json).unwrap();
    assert_eq!(msg_json, expected_json);

    #[cfg(cosmwasm)]
    {
        let msg_str = to_string(msg).unwrap();
        let serialized = to_string(msg_str).unwrap();
        let deserialized: QueryMsg = serde_json::from_str(&serialized).unwrap();
        assert_eq!(msg, deserialized);
    }
}
