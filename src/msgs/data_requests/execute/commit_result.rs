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

impl Execute {
    fn generate_hash(dr_id: &str, commitment: &str, chain_id: &str, contract_addr: &str, dr_height: u64) -> Hash {
        hash([
            "commit_data_result".as_bytes(),
            dr_id.as_bytes(),
            &dr_height.to_be_bytes(),
            commitment.as_bytes(),
            chain_id.as_bytes(),
            contract_addr.as_bytes(),
        ])
    }
}

impl VerifySelf for Execute {
    type Extra = u64;

    fn proof(&self) -> Result<Vec<u8>> {
        Ok(hex::decode(&self.proof)?)
    }

    fn msg_hash(&self, chain_id: &str, contract_addr: &str, dr_height: Self::Extra) -> Result<Hash> {
        Ok(Self::generate_hash(
            &self.dr_id,
            &self.commitment,
            chain_id,
            contract_addr,
            dr_height,
        ))
    }
}

pub struct ExectueFactory {
    dr_id:      String,
    commitment: String,
    public_key: String,
    hash:       Hash,
}

impl ExectueFactory {
    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    pub fn create_message(self, proof: Vec<u8>) -> Execute {
        Execute {
            dr_id:      self.dr_id,
            commitment: self.commitment,
            public_key: self.public_key,
            proof:      proof.to_hex(),
        }
    }
}

impl Execute {
    pub fn factory(
        dr_id: String,
        commitment: String,
        public_key: String,
        chain_id: &str,
        contract_addr: &str,
        dr_height: u64,
    ) -> ExectueFactory {
        let hash = Self::generate_hash(&dr_id, &commitment, chain_id, contract_addr, dr_height);
        ExectueFactory {
            dr_id,
            commitment,
            public_key,
            hash,
        }
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
