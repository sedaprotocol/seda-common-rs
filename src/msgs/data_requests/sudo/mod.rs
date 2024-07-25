use super::*;

pub mod post_results;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(serde::Serialize, Debug, PartialEq))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct PostResult {
    pub dr_id:     String,
    pub result:    DataResult,
    pub exit_code: u8,
}

impl From<PostResult> for crate::msgs::SudoMsg {
    fn from(value: PostResult) -> Self {
        SudoMsg::PostDataResult(value).into()
    }
}

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(serde::Serialize, Debug, PartialEq))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub enum SudoMsg {
    PostDataResult(PostResult),
    PostDataResults(post_results::Sudo),
}

impl From<SudoMsg> for super::SudoMsg {
    fn from(value: SudoMsg) -> Self {
        Self::DataRequest(value)
    }
}
