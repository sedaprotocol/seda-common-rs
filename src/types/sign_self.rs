use super::*;
use crate::{crypto::verify_proof, error::Result};

pub(crate) trait SignSelf {
    type Extra;

    fn proof(&self) -> Result<Vec<u8>>;

    fn msg_hash(&self, chain_id: &str, contract_addr: &str, extra: Self::Extra) -> Result<Hash>;

    fn sign(&self, signing_key: &[u8], chain_id: &str, contract_addr: &str, extra: Self::Extra) -> Result<Vec<u8>> {
        let proof = VRF.prove(signing_key, &self.msg_hash(chain_id, contract_addr, extra)?)?;

        Ok(proof)
    }

    fn verify_inner(&self, public_key: &[u8], chain_id: &str, contract_addr: &str, extra: Self::Extra) -> Result<()> {
        let msg_hash = self.msg_hash(chain_id, contract_addr, extra)?;
        verify_proof(public_key, &self.proof()?, msg_hash)
    }
}
