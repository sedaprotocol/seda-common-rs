use k256::{
    ecdsa::{SigningKey, VerifyingKey},
    elliptic_curve::rand_core::OsRng,
};
use serde_json::json;

use super::{
    query::{
        is_executor_eligible::{self, Query, QueryFactory},
        QueryMsg as StakingQueryMsg,
    },
    QueryMsg,
};
use crate::{crypto::VRF, msgs::*};

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
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
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
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
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
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}

#[test]
fn json_is_executor_committee_eligible() {
    let expected_json = json!({
    "is_executor_committee_eligible": {
      "public_key": "public_key",
    }
    });
    let msg: QueryMsg = StakingQueryMsg::IsExecutorCommitteeEligible {
        public_key: "public_key".to_string(),
    }
    .into();
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}

#[test]
fn json_is_executor_eligible() {
    #[cfg(not(feature = "cosmwasm"))]
    let data = "data".to_string();
    #[cfg(feature = "cosmwasm")]
    let data: Bytes = "data".as_bytes().into();

    let expected_json = json!({
    "is_executor_eligible": {
      "data": data,
    }
    });
    let msg: QueryMsg = is_executor_eligible::Query { data }.into();
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}

#[test]
fn json_get_staking_config() {
    let expected_json = json!({
      "get_staking_config": {}
    });
    let msg: QueryMsg = StakingQueryMsg::GetStakingConfig {}.into();
    #[cfg(not(feature = "cosmwasm"))]
    assert_json_ser(msg, expected_json);
    #[cfg(feature = "cosmwasm")]
    assert_json_deser(msg, expected_json);
}

fn new_public_key() -> (SigningKey, [u8; 33]) {
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);
    let public_key = verifying_key.to_encoded_point(true).as_bytes().try_into().unwrap();

    (signing_key, public_key)
}

fn prove(signing_key: &[u8], hash: &[u8]) -> Vec<u8> {
    VRF.prove(signing_key, hash).unwrap()
}

impl QueryFactory {
    fn create_query(self, proof: Vec<u8>) -> Query {
        let data = format!("{}:{}:{}", self.public_key, self.dr_id, proof.to_hex());

        Query {
            data: Self::encode_data(&data),
        }
    }
}

#[test]
fn is_executor_eligible_decode_correctly() {
    let (sk, pk) = new_public_key();
    let pk_hex = hex::encode(pk);

    let dr_id = "dr_id".hash();
    let dr_id_hex = dr_id.to_hex();

    let factory = is_executor_eligible::Query::factory(pk_hex, dr_id_hex, "foo", "bar");
    let proof = prove(sk.to_bytes().as_slice(), factory.get_hash());
    let query = factory.create_query(proof.clone());

    let (decoded_pk, decoded_dr_id_hash, decoded_proof) = query.parts().unwrap();
    assert_eq!(pk, decoded_pk);
    assert_eq!(dr_id, decoded_dr_id_hash);
    assert_eq!(proof, decoded_proof);
}
