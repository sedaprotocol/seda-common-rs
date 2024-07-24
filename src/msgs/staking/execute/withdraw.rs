use super::*;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct Execute {
    pub public_key: String,
    pub proof:      String,
    pub amount:     U128,
}

impl SignSelf for Execute {
    fn msg_hash(&self, chain_height: u64, chain_id: &str, contract_addr: &str) -> Result<Hash> {
        Ok(hash([
            "withdraw".as_bytes(),
            #[cfg(not(feature = "cosmwasm"))]
            self.amount.as_bytes(),
            #[cfg(feature = "cosmwasm")]
            &self.amount.to_be_bytes(),
            &chain_height.to_be_bytes(),
            chain_id.as_bytes(),
            contract_addr.as_bytes(),
            // todo sequence number
        ]))
    }
}

impl From<Execute> for crate::msgs::ExecuteMsg {
    fn from(value: Execute) -> Self {
        super::ExecuteMsg::Withdraw(value).into()
    }
}
