use vrf_rs::Secp256k1Sha256;

use super::*;
use crate::{crypto::verify_proof, error::Result};

pub trait SignSelf {
    type Extra;
    fn msg_hash(&self, chain_id: &str, contract_addr: &str, extra: Self::Extra) -> Result<Hash>;

    fn sign(&mut self, signing_key: &[u8], chain_id: &str, contract_addr: &str, extra: Self::Extra) -> Result<Vec<u8>> {
        let msg_hash = self.msg_hash(chain_id, contract_addr, extra)?;

        let vrf = Secp256k1Sha256::default();
        let proof = vrf.prove(signing_key, &msg_hash)?;

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
        verify_proof(public_key, proof, self.msg_hash(chain_id, contract_addr, extra)?)
    }
}
