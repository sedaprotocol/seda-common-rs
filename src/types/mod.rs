use semver::Version;
use sha3::{Digest, Keccak256};

mod to_hex;
pub use to_hex::ToHexStr;

mod hash_self;
pub use hash_self::{HashSelf, TryHashSelf};

mod sign_self;
pub(crate) use sign_self::SignSelf;
use vrf_rs::Secp256k1Sha256;

#[cfg(feature = "cosmwasm")]
pub(crate) type U128 = cosmwasm_std::Uint128;
#[cfg(not(feature = "cosmwasm"))]
pub(crate) type U128 = u128;

pub fn serialize_as_str<S, V>(value: V, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    V: ToString,
{
    serializer.serialize_str(&value.to_string())
}

#[cfg(feature = "cosmwasm")]
pub(crate) type Bytes = cosmwasm_std::Binary;
#[cfg(not(feature = "cosmwasm"))]
pub(crate) type Bytes = String;

pub type Hash = [u8; 32];

lazy_static::lazy_static! {
    pub static ref VRF: Secp256k1Sha256 = Secp256k1Sha256::default();
}
