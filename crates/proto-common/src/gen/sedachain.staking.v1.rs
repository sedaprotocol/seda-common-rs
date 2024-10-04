// @generated
// This file is @generated by prost-build.
/// MsgCreateValidator defines a SDK message for creating a new validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateValidatorWithVrf {
    #[prost(message, optional, tag="1")]
    pub description: ::core::option::Option<::cosmos_sdk_proto::cosmos::staking::v1beta1::Description>,
    #[prost(message, optional, tag="2")]
    pub commission: ::core::option::Option<::cosmos_sdk_proto::cosmos::staking::v1beta1::CommissionRates>,
    #[prost(string, tag="3")]
    pub min_self_delegation: ::prost::alloc::string::String,
    /// Deprecated: Use of Delegator Address in MsgCreateValidator is deprecated.
    /// The validator address bytes and delegator address bytes refer to the same
    /// account while creating validator (defer only in bech32 notation).
    #[deprecated]
    #[prost(string, tag="4")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub pubkey: ::core::option::Option<::prost_types::Any>,
    #[prost(message, optional, tag="7")]
    pub value: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="8")]
    pub vrf_pubkey: ::core::option::Option<::prost_types::Any>,
}
impl ::prost::Name for MsgCreateValidatorWithVrf {
const NAME: &'static str = "MsgCreateValidatorWithVRF";
const PACKAGE: &'static str = "sedachain.staking.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.staking.v1.MsgCreateValidatorWithVRF".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.staking.v1.MsgCreateValidatorWithVRF".into() }}
/// MsgCreateValidatorResponse defines the Msg/CreateValidator response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgCreateValidatorWithVrfResponse {
}
impl ::prost::Name for MsgCreateValidatorWithVrfResponse {
const NAME: &'static str = "MsgCreateValidatorWithVRFResponse";
const PACKAGE: &'static str = "sedachain.staking.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.staking.v1.MsgCreateValidatorWithVRFResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.staking.v1.MsgCreateValidatorWithVRFResponse".into() }}
// @@protoc_insertion_point(module)
