use vrf_rs::Secp256k1Sha256;

use super::*;
use crate::crypto::verify_proof;

pub trait SignSelf {
    fn msg_hash(&self, chain_height: u64, chain_id: &str, contract_addr: &str) -> Result<Hash>;

    fn sign(&mut self, signing_key: &[u8], chain_height: u64, chain_id: &str, contract_addr: &str) -> Result<Vec<u8>> {
        let msg_hash = self.msg_hash(chain_height, chain_id, contract_addr)?;

        let vrf = Secp256k1Sha256::default();
        let proof = vrf.prove(signing_key, &msg_hash)?;

        Ok(proof)
    }

    fn verify(
        &self,
        public_key: &[u8],
        proof: &[u8],
        chain_height: u64,
        chain_id: &str,
        contract_addr: &str,
    ) -> Result<()> {
        verify_proof(public_key, proof, self.msg_hash(chain_height, chain_id, contract_addr)?)
    }
}
