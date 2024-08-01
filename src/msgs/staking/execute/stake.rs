use super::*;

#[cfg_attr(feature = "cosmwasm", cw_serde)]
#[cfg_attr(not(feature = "cosmwasm"), derive(Serialize, Debug, PartialEq))]
#[cfg_attr(not(feature = "cosmwasm"), serde(rename_all = "snake_case"))]
pub struct Execute {
    pub public_key: String,
    pub proof:      String,
    pub memo:       Option<Bytes>,
}

impl Execute {
    fn generate_hash(memo: Option<&Bytes>, chain_id: &str, contract_addr: &str, sequence: u128) -> Hash {
        hash([
            "stake".as_bytes(),
            &memo.hash(),
            chain_id.as_bytes(),
            contract_addr.as_bytes(),
            &sequence.to_be_bytes(),
        ])
    }
}

impl VerifySelf for Execute {
    type Extra = u128;

    fn proof(&self) -> Result<Vec<u8>> {
        Ok(hex::decode(&self.proof)?)
    }

    fn msg_hash(&self, chain_id: &str, contract_addr: &str, sequence: Self::Extra) -> Result<Hash> {
        Ok(Self::generate_hash(
            self.memo.as_ref(),
            chain_id,
            contract_addr,
            sequence,
        ))
    }
}

pub struct ExecuteFactory {
    public_key: String,
    memo:       Option<Bytes>,
    hash:       Hash,
}

impl ExecuteFactory {
    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    pub fn create_message(self, proof: Vec<u8>) -> Execute {
        Execute {
            public_key: self.public_key,
            proof:      proof.to_hex(),
            memo:       self.memo,
        }
    }
}

impl Execute {
    pub fn factory(
        public_key: String,
        memo: Option<Bytes>,
        chain_id: &str,
        contract_addr: &str,
        sequence: u128,
    ) -> ExecuteFactory {
        let hash = Self::generate_hash(memo.as_ref(), chain_id, contract_addr, sequence);
        ExecuteFactory { public_key, memo, hash }
    }

    pub fn verify(&self, public_key: &[u8], chain_id: &str, contract_addr: &str, sequence: u128) -> Result<()> {
        self.verify_inner(public_key, chain_id, contract_addr, sequence)
    }
}

impl From<Execute> for crate::msgs::ExecuteMsg {
    fn from(value: Execute) -> Self {
        super::ExecuteMsg::Stake(value).into()
    }
}
