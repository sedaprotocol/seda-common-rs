// @generated
// This file is @generated by prost-build.
/// Module parameters which can be changed through governance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Params {
    /// min_fee_update_delay is the minimum number of blocks after which a fee
    /// update comes into effect.
    #[prost(uint32, tag="1")]
    pub min_fee_update_delay: u32,
}
/// ProxyConfig defines a data-proxy entry in the registry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyConfig {
    /// payout_address defines the address to which the data proxy fees should be
    /// transferred.
    #[prost(string, tag="1")]
    pub payout_address: ::prost::alloc::string::String,
    /// fee defines the amount in aseda this data-proxy charges when utilised.
    #[prost(message, optional, tag="2")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// memo defines an optional string which is not used by the protocol.
    #[prost(string, tag="3")]
    pub memo: ::prost::alloc::string::String,
    /// only the admin address of a data proxy can submit config updates.
    #[prost(string, tag="4")]
    pub admin_address: ::prost::alloc::string::String,
    /// fee_update defines an upcoming fee change which will take effect at a
    /// future height.
    #[prost(message, optional, tag="5")]
    pub fee_update: ::core::option::Option<FeeUpdate>,
}
/// FeeUpdate defines a new fee amount and the height at which it will take
/// effect.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeUpdate {
    /// new_fee defines the new fee for the data proxy.
    #[prost(message, optional, tag="1")]
    pub new_fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// update_height defines the height after which the new fee comes into effect.
    #[prost(int64, tag="2")]
    pub update_height: i64,
}
/// GenesisState defines data_proxy module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag="2")]
    pub data_proxy_configs: ::prost::alloc::vec::Vec<DataProxyConfig>,
    #[prost(message, repeated, tag="3")]
    pub fee_update_queue: ::prost::alloc::vec::Vec<FeeUpdateQueueRecord>,
}
/// DataProxyConfigs define the data proxy entries in the registry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataProxyConfig {
    #[prost(bytes="bytes", tag="1")]
    pub data_proxy_pubkey: ::prost::bytes::Bytes,
    #[prost(message, optional, tag="2")]
    pub config: ::core::option::Option<ProxyConfig>,
}
/// FeeUpdateQueueRecord defines an entry in the data proxy update queue.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeUpdateQueueRecord {
    #[prost(bytes="bytes", tag="1")]
    pub data_proxy_pubkey: ::prost::bytes::Bytes,
    #[prost(int64, tag="2")]
    pub update_height: i64,
}
/// The request message for QueryDataProxyConfig RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDataProxyConfigRequest {
    /// A hex encoded string of the public key of the data proxy.
    #[prost(string, tag="1")]
    pub pub_key: ::prost::alloc::string::String,
}
/// The response message for QueryDataProxyConfig RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDataProxyConfigResponse {
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<ProxyConfig>,
}
/// All data required for a new data proxy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterDataProxy {
    /// admin_address is the address that can update the proxy config.
    #[prost(string, tag="1")]
    pub admin_address: ::prost::alloc::string::String,
    /// payout_address defines the address to which the data proxy fees should be
    /// transferred.
    #[prost(string, tag="2")]
    pub payout_address: ::prost::alloc::string::String,
    /// fee defines the amount in aseda this data-proxy charges when utilised.
    #[prost(message, optional, tag="3")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// memo defines an optional string which is not used by the protocol.
    #[prost(string, tag="4")]
    pub memo: ::prost::alloc::string::String,
    /// hex encoded bytes as the expected flow already uses hex encoded bytes to go
    /// from the CLI to the browser where the transaction is signed.
    #[prost(string, tag="5")]
    pub pub_key: ::prost::alloc::string::String,
    /// hex encoded bytes as the expected flow already uses hex encoded bytes to go
    /// from the CLI to the browser where the transaction is signed.
    #[prost(string, tag="6")]
    pub signature: ::prost::alloc::string::String,
}
/// No response required.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgRegisterDataProxyResponse {
}
/// Allow updating memo and payout address instantly and/or scheduling a fee
/// update.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEditDataProxy {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_payout_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_memo: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub new_fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// 0 will default to the minimum delay configured in the params
    #[prost(uint32, tag="5")]
    pub fee_update_delay: u32,
    /// hex encoded bytes as the expected flow is users sending updates from the
    /// browser
    #[prost(string, tag="6")]
    pub pub_key: ::prost::alloc::string::String,
}
/// Allow transferring the admin role to a different address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferAdmin {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_admin_address: ::prost::alloc::string::String,
    /// hex encoded bytes as the expected flow is users sending updates from the
    /// browser
    #[prost(string, tag="3")]
    pub pub_key: ::prost::alloc::string::String,
}
/// No response required.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgTransferAdminResponse {
}
/// Returns the height after which the fee update will go into effect.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgEditDataProxyResponse {
    #[prost(int64, tag="1")]
    pub fee_update_height: i64,
}
/// The request message for the UpdateParams method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless
    /// overwritten).
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// No response required.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
// @@protoc_insertion_point(module)
