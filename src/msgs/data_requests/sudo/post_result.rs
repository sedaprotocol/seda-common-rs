use super::*;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct Sudo {
    pub dr_id:     String,
    pub result:    DataResult,
    pub exit_code: u8,
}

impl From<Sudo> for crate::msgs::SudoMsg {
    fn from(value: Sudo) -> Self {
        super::SudoMsg::PostDataResult(value).into()
    }
}
