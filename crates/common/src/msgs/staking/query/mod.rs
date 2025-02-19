pub mod is_executor_eligible;
#[cfg(feature = "cosmwasm")]
use super::types::*;
#[cfg(feature = "cosmwasm")]
use crate::types::U128;

#[cfg_attr(feature = "cosmwasm", cosmwasm_schema::cw_serde)]
#[cfg_attr(feature = "cosmwasm", derive(cosmwasm_schema::QueryResponses))]
#[cfg_attr(not(feature = "cosmwasm"), derive(serde::Serialize, Debug, PartialEq))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub enum QueryMsg {
    #[cfg_attr(feature = "cosmwasm", returns(Option<Staker>))]
    GetStaker { public_key: String },
    #[cfg_attr(feature = "cosmwasm", returns(U128))]
    GetAccountSeq { public_key: String },
    #[cfg_attr(feature = "cosmwasm", returns(StakerAndSeq))]
    GetStakerAndSeq { public_key: String },
    #[cfg_attr(feature = "cosmwasm", returns(bool))]
    IsStakerExecutor { public_key: String },
    #[cfg_attr(feature = "cosmwasm", returns(bool))]
    IsExecutorEligible(is_executor_eligible::Query),
    #[cfg_attr(feature = "cosmwasm", returns(StakingConfig))]
    GetStakingConfig {},
    #[cfg_attr(feature = "cosmwasm", returns(GetExecutorsResponse))]
    GetExecutors { offset: u32, limit: u32 },
}

impl From<QueryMsg> for crate::msgs::QueryMsg {
    fn from(value: QueryMsg) -> Self {
        Self::Staking(value)
    }
}
