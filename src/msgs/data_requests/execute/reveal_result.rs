use super::*;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct Execute {
    pub dr_id:       String,
    pub reveal_body: RevealBody,
    pub public_key:  String,
    pub proof:       String,
    pub stderr:      Vec<String>,
    pub stdout:      Vec<String>,
}

impl SignSelf for Execute {
    fn msg_hash(&self, chain_height: u64, chain_id: &str, contract_addr: &str) -> Result<Hash> {
        let reveal_body_hash = self.reveal_body.try_hash()?;
        Ok(hash([
            "reveal_data_result".as_bytes(),
            self.dr_id.as_bytes(),
            &chain_height.to_be_bytes(),
            &reveal_body_hash,
            chain_id.as_bytes(),
            contract_addr.as_bytes(),
        ]))
    }
}

impl From<Execute> for crate::msgs::ExecuteMsg {
    fn from(value: Execute) -> Self {
        super::ExecuteMsg::RevealDataResult(value).into()
    }
}
