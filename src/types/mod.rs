use semver::Version;
use sha3::{Digest, Keccak256};

mod to_hex;
pub use to_hex::ToHexStr;

mod hash_self;
pub use hash_self::{HashSelf, TryHashSelf};

mod sign_self;
pub use sign_self::SignSelf;

use crate::error::Result;

#[cfg(feature = "cosmwasm")]
pub(crate) type U128 = cosmwasm_std::Uint128;
#[cfg(not(feature = "cosmwasm"))]
pub(crate) type U128 = String;

#[cfg(feature = "cosmwasm")]
pub(crate) type Bytes = cosmwasm_std::Binary;
#[cfg(not(feature = "cosmwasm"))]
pub(crate) type Bytes = String;

pub type Hash = [u8; 32];
