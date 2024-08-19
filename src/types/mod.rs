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
#[cfg(not(feature = "cosmwasm"))]
pub(crate) type U128 = Uint128;

#[cfg(not(feature = "cosmwasm"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Uint128(pub u128);

#[cfg(not(feature = "cosmwasm"))]
impl Uint128 {
    pub fn to_be_bytes(&self) -> [u8; 16] {
        self.0.to_be_bytes()
    }
}

#[cfg(not(feature = "cosmwasm"))]
impl core::fmt::Display for Uint128 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(not(feature = "cosmwasm"))]
impl From<u128> for Uint128 {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

#[cfg(not(feature = "cosmwasm"))]
impl serde::Serialize for Uint128 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

#[cfg(not(feature = "cosmwasm"))]
impl<'de> serde::de::Deserialize<'de> for Uint128 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let u = s.parse().map_err(serde::de::Error::custom)?;
        Ok(Self(u))
    }
}

#[cfg(feature = "cosmwasm")]
pub(crate) type Bytes = cosmwasm_std::Binary;
#[cfg(not(feature = "cosmwasm"))]
pub(crate) type Bytes = String;

pub type Hash = [u8; 32];
