use super::*;

pub mod commit_result;
pub mod post_request;
pub mod reveal_result;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Debug, PartialEq))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub enum ExecuteMsg {
    CommitDataResult(commit_result::Execute),
    PostDataRequest(post_request::Execute),
    RevealDataResult(reveal_result::Execute),
}

impl From<ExecuteMsg> for super::ExecuteMsg {
    fn from(value: ExecuteMsg) -> Self {
        Self::DataRequest(Box::new(value))
    }
}
