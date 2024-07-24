use super::*;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct Execute {
    pub dr_id:      String,
    pub commitment: String,
    pub public_key: String,
    pub proof:      String,
}

impl SignSelf for Execute {
    fn msg_hash(&self, chain_height: u64, chain_id: &str, contract_addr: &str) -> Result<Hash> {
        Ok(hash([
            "commit_data_result".as_bytes(),
            self.dr_id.as_bytes(),
            &chain_height.to_be_bytes(),
            self.commitment.as_bytes(),
            chain_id.as_bytes(),
            contract_addr.as_bytes(),
        ]))
    }
}

impl From<Execute> for crate::msgs::ExecuteMsg {
    fn from(value: Execute) -> Self {
        super::ExecuteMsg::CommitDataResult(value).into()
    }
}
