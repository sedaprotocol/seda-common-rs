use semver::Version;
use sha3::{Digest, Keccak256};

mod to_hex;
pub use to_hex::ToHexStr;

mod hash_self;
pub use hash_self::{HashSelf, TryHashSelf};

mod verify_self;
pub(crate) use verify_self::VerifySelf;

#[cfg(feature = "cosmwasm")]
pub(crate) type U128 = cosmwasm_std::Uint128;

// Is required to be a String, JSON does not support u128 numbers
#[cfg(not(feature = "cosmwasm"))]
pub(crate) type U128 = String;

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
