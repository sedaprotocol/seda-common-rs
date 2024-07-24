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
    type Extra = u128;

    fn msg_hash(&self, chain_id: &str, contract_addr: &str, sequence: Self::Extra) -> Result<Hash> {
        Ok(hash([
            "unstake".as_bytes(),
            // todo in overlay this is an u128 converted to a string we don't want to have to reparse it...
            #[cfg(not(feature = "cosmwasm"))]
            self.amount.as_bytes(),
            #[cfg(feature = "cosmwasm")]
            &self.amount.to_be_bytes(),
            chain_id.as_bytes(),
            contract_addr.as_bytes(),
            &sequence.to_be_bytes(),
        ]))
    }
}

impl From<Execute> for crate::msgs::ExecuteMsg {
    fn from(value: Execute) -> Self {
        super::ExecuteMsg::Unstake(value).into()
    }
}
