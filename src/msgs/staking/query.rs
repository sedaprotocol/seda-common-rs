use super::*;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(feature = "cosmwasm", derive(QueryResponses))]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Debug, PartialEq))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub enum QueryMsg {
    #[cfg_attr(feature = "cosmwasm", returns(Option<Staker>))]
    GetStaker { public_key: String },
    #[cfg_attr(feature = "cosmwasm", returns(U128))]
    GetAccountSeq { public_key: String },
    #[cfg_attr(feature = "cosmwasm", returns(StakerAndSeq))]
    GetStakerAndSeq { public_key: String },
    #[cfg_attr(feature = "cosmwasm", returns(bool))]
    IsExecutorEligible { proof: String, dr_id: String },
    #[cfg_attr(feature = "cosmwasm", returns(StakingConfig))]
    GetStakingConfig {},
}

impl From<QueryMsg> for super::QueryMsg {
    fn from(value: QueryMsg) -> Self {
        Self::Staking(value)
    }
}
