use super::*;
use crate::{crypto::verify_proof, error::Result};

pub trait SignSelf {
    type Extra;
    fn msg_hash(&self, chain_id: &str, contract_addr: &str, extra: Self::Extra) -> Result<Hash>;

    fn sign(&self, signing_key: &[u8], msg_hash: &[u8]) -> Result<Vec<u8>> {
        let proof = VRF.prove(signing_key, msg_hash)?;

        Ok(proof)
    }

    fn verify(
        &self,
        public_key: &[u8],
        proof: &[u8],
        chain_id: &str,
        contract_addr: &str,
        extra: Self::Extra,
    ) -> Result<()> {
        let msg_hash = self.msg_hash(chain_id, contract_addr, extra)?;
        verify_proof(public_key, proof, msg_hash)
    }
}
