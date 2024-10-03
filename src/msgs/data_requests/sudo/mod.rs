pub mod expire_data_requests;
pub mod remove_requests;

#[cfg_attr(feature = "cosmwasm", cosmwasm_schema::cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(serde::Serialize, Debug, PartialEq))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct PostResult {
    pub dr_id: String,
}

impl From<PostResult> for crate::msgs::SudoMsg {
    fn from(value: PostResult) -> Self {
        SudoMsg::RemoveDataRequest(value).into()
    }
}

#[cfg_attr(feature = "cosmwasm", cosmwasm_schema::cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(serde::Serialize, Debug, PartialEq))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub enum SudoMsg {
    RemoveDataRequest(PostResult),
    RemoveDataRequests(remove_requests::Sudo),
    ExpireDataRequests(expire_data_requests::Sudo),
}

impl From<SudoMsg> for super::SudoMsg {
    fn from(value: SudoMsg) -> Self {
        Self::DataRequest(value)
    }
}
