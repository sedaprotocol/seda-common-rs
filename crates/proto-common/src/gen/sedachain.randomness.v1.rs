// @generated
// This file is @generated by prost-build.
/// GenesisState defines the randomness module's genesis state with a seed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(string, tag="1")]
    pub seed: ::prost::alloc::string::String,
}
impl ::prost::Name for GenesisState {
const NAME: &'static str = "GenesisState";
const PACKAGE: &'static str = "sedachain.randomness.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.randomness.v1.GenesisState".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.randomness.v1.GenesisState".into() }}
/// The message for getting the random modules seed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QuerySeedRequest {
}
impl ::prost::Name for QuerySeedRequest {
const NAME: &'static str = "QuerySeedRequest";
const PACKAGE: &'static str = "sedachain.randomness.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.randomness.v1.QuerySeedRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.randomness.v1.QuerySeedRequest".into() }}
/// The message for returning the random modules seed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySeedResponse {
    #[prost(string, tag="1")]
    pub seed: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub block_height: i64,
}
impl ::prost::Name for QuerySeedResponse {
const NAME: &'static str = "QuerySeedResponse";
const PACKAGE: &'static str = "sedachain.randomness.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.randomness.v1.QuerySeedResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.randomness.v1.QuerySeedResponse".into() }}
/// ValidatorVRF is the randomness validator's VRF key information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorVrf {
    /// operator_address defines the address of the validator's operator; bech
    /// encoded in JSON.
    #[prost(string, tag="1")]
    pub operator_address: ::prost::alloc::string::String,
    /// vrf_pubkey is the public key of the validator's VRF key pair
    #[prost(message, optional, tag="2")]
    pub vrf_pubkey: ::core::option::Option<::prost_types::Any>,
}
impl ::prost::Name for ValidatorVrf {
const NAME: &'static str = "ValidatorVRF";
const PACKAGE: &'static str = "sedachain.randomness.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.randomness.v1.ValidatorVRF".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.randomness.v1.ValidatorVRF".into() }}
/// The message for submitting a new seed to the chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgNewSeed {
    /// address of VRF key used to produce proof
    #[prost(string, tag="1")]
    pub prover: ::prost::alloc::string::String,
    /// VRF proof
    #[prost(string, tag="2")]
    pub pi: ::prost::alloc::string::String,
    /// VRF hash
    #[prost(string, tag="3")]
    pub beta: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgNewSeed {
const NAME: &'static str = "MsgNewSeed";
const PACKAGE: &'static str = "sedachain.randomness.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.randomness.v1.MsgNewSeed".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.randomness.v1.MsgNewSeed".into() }}
/// The response message for submitting a new seed to the chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgNewSeedResponse {
}
impl ::prost::Name for MsgNewSeedResponse {
const NAME: &'static str = "MsgNewSeedResponse";
const PACKAGE: &'static str = "sedachain.randomness.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.randomness.v1.MsgNewSeedResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.randomness.v1.MsgNewSeedResponse".into() }}
// @@protoc_insertion_point(module)
