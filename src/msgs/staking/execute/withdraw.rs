use super::*;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, PartialEq, Debug))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct Execute {
    pub public_key: String,
    pub proof:      String,
    #[cfg_attr(not(feature = "cosmwasm"), serde(serialize_with = "crate::types::serialize_as_str"))]
    pub amount:     U128,
}

impl SignSelf for Execute {
    type Extra = u128;

    fn proof(&self) -> Result<Vec<u8>> {
        Ok(hex::decode(&self.proof)?)
    }

    fn msg_hash(&self, chain_id: &str, contract_addr: &str, sequence: Self::Extra) -> Result<Hash> {
        Ok(hash([
            "withdraw".as_bytes(),
            #[cfg(not(feature = "cosmwasm"))]
            &self.amount.to_be_bytes(),
            #[cfg(feature = "cosmwasm")]
            &self.amount.to_be_bytes(),
            chain_id.as_bytes(),
            contract_addr.as_bytes(),
            &sequence.to_be_bytes(),
        ]))
    }
}

impl Execute {
    pub fn new(
        public_key: String,
        amount: U128,
        signing_key: &[u8],
        chain_id: &str,
        contract_addr: &str,
        sequence: u128,
    ) -> Result<Self> {
        let mut ex = Self {
            public_key,
            amount,
            proof: Default::default(),
        };
        ex.proof = ex.sign(signing_key, chain_id, contract_addr, sequence)?.to_hex();

        Ok(ex)
    }

    pub fn verify(&self, public_key: &[u8], chain_id: &str, contract_addr: &str, sequence: u128) -> Result<()> {
        self.verify_inner(public_key, chain_id, contract_addr, sequence)
    }
}

impl From<Execute> for crate::msgs::ExecuteMsg {
    fn from(value: Execute) -> Self {
        super::ExecuteMsg::Withdraw(value).into()
    }
}
