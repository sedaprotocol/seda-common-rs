use super::*;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(feature = "cosmwasm", derive(QueryResponses))]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Debug, PartialEq))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub enum QueryMsg {
    #[cfg_attr(feature = "cosmwasm", returns(Option<DataRequest>))]
    GetDataRequest { dr_id: String },
    #[cfg_attr(feature = "cosmwasm",  returns(Option<String>))]
    GetDataRequestCommitment { dr_id: String, public_key: String },
    #[cfg_attr(feature = "cosmwasm",  returns(HashMap<String, String>))]
    GetDataRequestCommitments { dr_id: String },
    #[cfg_attr(feature = "cosmwasm",  returns(Option<RevealBody>))]
    GetDataRequestReveal { dr_id: String, public_key: String },
    #[cfg_attr(feature = "cosmwasm",  returns(HashMap<String, RevealBody>))]
    GetDataRequestReveals { dr_id: String },
    #[cfg_attr(feature = "cosmwasm",  returns(Vec<DataRequest>))]
    GetDataRequestsByStatus {
        status: DataRequestStatus,
        offset: u32,
        limit:  u32,
    },
}

impl From<QueryMsg> for super::QueryMsg {
    fn from(value: QueryMsg) -> Self {
        Self::DataRequest(value)
    }
}
