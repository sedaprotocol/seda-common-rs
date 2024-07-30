use super::*;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Debug, PartialEq))]
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
    type Extra = (u64, Hash);

    fn proof(&self) -> Result<Vec<u8>> {
        Ok(hex::decode(&self.proof)?)
    }

    fn msg_hash(
        &self,
        chain_id: &str,
        contract_addr: &str,
        (dr_height, reveal_body_hash): Self::Extra,
    ) -> Result<Hash> {
        Ok(hash([
            "reveal_data_result".as_bytes(),
            self.dr_id.as_bytes(),
            &dr_height.to_be_bytes(),
            &reveal_body_hash,
            chain_id.as_bytes(),
            contract_addr.as_bytes(),
        ]))
    }
}

impl Execute {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        dr_id: String,
        reveal_body: RevealBody,
        public_key: String,
        stderr: Vec<String>,
        stdout: Vec<String>,
        signing_key: &[u8],
        chain_id: &str,
        contract_addr: &str,
        dr_height: u64,
        reveal_body_hash: Hash,
    ) -> Result<Self> {
        let mut ex = Self {
            dr_id,
            reveal_body,
            public_key,
            stderr,
            stdout,
            proof: Default::default(),
        };
        ex.proof = ex
            .sign(signing_key, chain_id, contract_addr, (dr_height, reveal_body_hash))?
            .to_hex();

        Ok(ex)
    }

    pub fn verify(
        &self,
        public_key: &[u8],
        chain_id: &str,
        contract_addr: &str,
        dr_height: u64,
        reveal_body_hash: Hash,
    ) -> Result<()> {
        self.verify_inner(public_key, chain_id, contract_addr, (dr_height, reveal_body_hash))
    }
}

impl From<Execute> for crate::msgs::ExecuteMsg {
    fn from(value: Execute) -> Self {
        super::ExecuteMsg::RevealDataResult(value).into()
    }
}
