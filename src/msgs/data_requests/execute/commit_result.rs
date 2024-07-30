use super::*;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Debug, PartialEq))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct Execute {
    pub dr_id:      String,
    pub commitment: String,
    pub public_key: String,
    pub proof:      String,
}

impl SignSelf for Execute {
    type Extra = u64;

    fn proof(&self) -> Result<Vec<u8>> {
        Ok(hex::decode(&self.proof)?)
    }

    fn msg_hash(&self, chain_id: &str, contract_addr: &str, dr_height: Self::Extra) -> Result<Hash> {
        Ok(hash([
            "commit_data_result".as_bytes(),
            self.dr_id.as_bytes(),
            &dr_height.to_be_bytes(),
            self.commitment.as_bytes(),
            chain_id.as_bytes(),
            contract_addr.as_bytes(),
        ]))
    }
}

impl Execute {
    pub fn new(
        dr_id: String,
        commitment: String,
        public_key: String,
        signing_key: &[u8],
        chain_id: &str,
        contract_addr: &str,
        dr_height: u64,
    ) -> Result<Self> {
        let mut ex = Self {
            dr_id,
            commitment,
            public_key,
            proof: Default::default(),
        };
        ex.proof = ex.sign(signing_key, chain_id, contract_addr, dr_height)?.to_hex();

        Ok(ex)
    }

    pub fn verify(&self, public_key: &[u8], chain_id: &str, contract_addr: &str, dr_height: u64) -> Result<()> {
        self.verify_inner(public_key, chain_id, contract_addr, dr_height)
    }
}

impl From<Execute> for crate::msgs::ExecuteMsg {
    fn from(value: Execute) -> Self {
        super::ExecuteMsg::CommitDataResult(value).into()
    }
}
