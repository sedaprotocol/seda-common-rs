#[cfg(feature = "data_proxy")]
#[path = "sedachain.data_proxy.v1.rs"]
#[rustfmt::skip]
pub mod sedachain_data_proxy;

#[cfg(feature = "randomness")]
#[path = "sedachain.randomness.v1.rs"]
#[rustfmt::skip]
pub mod sedachain_randomness;

#[cfg(feature = "staking")]
#[path = "sedachain.staking.v1.rs"]
#[rustfmt::skip]
pub mod sedachain_staking;

#[cfg(feature = "vesting")]
#[path = "sedachain.vesting.v1.rs"]
#[rustfmt::skip]
pub mod sedachain_vesting;

#[cfg(feature = "wasm_storage")]
#[path = "sedachain.wasm_storage.v1.rs"]
#[rustfmt::skip]
pub mod sedachain_wasm_storage;

// re-export prost
#[cfg(feature = "cosmos")]
pub use cosmos_sdk_proto;
pub use prost;
