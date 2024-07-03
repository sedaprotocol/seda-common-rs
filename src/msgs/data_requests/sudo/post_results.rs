use super::*;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct Sudo {
    pub results: Vec<PostResult>,
}

impl From<Sudo> for crate::msgs::SudoMsg {
    fn from(value: Sudo) -> Self {
        super::SudoMsg::PostDataResults(value).into()
    }
}
