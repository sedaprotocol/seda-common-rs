// @generated
// This file is @generated by prost-build.
/// IndexPubKeyPair defines an index - public key pair.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexedPubKey {
    #[prost(uint32, tag="1")]
    pub index: u32,
    #[prost(message, optional, tag="2")]
    pub pub_key: ::core::option::Option<::prost_types::Any>,
}
impl ::prost::Name for IndexedPubKey {
const NAME: &'static str = "IndexedPubKey";
const PACKAGE: &'static str = "sedachain.pubkey.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.pubkey.v1.IndexedPubKey".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.pubkey.v1.IndexedPubKey".into() }}
/// GenesisState defines pubkey module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag="1")]
    pub validator_pub_keys: ::prost::alloc::vec::Vec<ValidatorPubKeys>,
}
impl ::prost::Name for GenesisState {
const NAME: &'static str = "GenesisState";
const PACKAGE: &'static str = "sedachain.pubkey.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.pubkey.v1.GenesisState".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.pubkey.v1.GenesisState".into() }}
/// ValidatorPubKeys defines a validator's list of registered public keys
/// primarily used in the x/pubkey genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorPubKeys {
    #[prost(string, tag="1")]
    pub validator_addr: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub indexed_pub_keys: ::prost::alloc::vec::Vec<IndexedPubKey>,
}
impl ::prost::Name for ValidatorPubKeys {
const NAME: &'static str = "ValidatorPubKeys";
const PACKAGE: &'static str = "sedachain.pubkey.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.pubkey.v1.ValidatorPubKeys".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.pubkey.v1.ValidatorPubKeys".into() }}
/// QueryValidatorKeysRequest is request type for the Query/ValidatorKeys RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorKeysRequest {
    #[prost(string, tag="1")]
    pub validator_addr: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryValidatorKeysRequest {
const NAME: &'static str = "QueryValidatorKeysRequest";
const PACKAGE: &'static str = "sedachain.pubkey.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.pubkey.v1.QueryValidatorKeysRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.pubkey.v1.QueryValidatorKeysRequest".into() }}
/// QueryValidatorKeysResponse is response type for the Query/ValidatorKeys RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorKeysResponse {
    #[prost(message, optional, tag="1")]
    pub validator_pub_keys: ::core::option::Option<ValidatorPubKeys>,
}
impl ::prost::Name for QueryValidatorKeysResponse {
const NAME: &'static str = "QueryValidatorKeysResponse";
const PACKAGE: &'static str = "sedachain.pubkey.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.pubkey.v1.QueryValidatorKeysResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.pubkey.v1.QueryValidatorKeysResponse".into() }}
/// MsgAddKey defines a message for registering a new public key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddKey {
    #[prost(string, tag="1")]
    pub validator_addr: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub indexed_pub_keys: ::prost::alloc::vec::Vec<IndexedPubKey>,
}
impl ::prost::Name for MsgAddKey {
const NAME: &'static str = "MsgAddKey";
const PACKAGE: &'static str = "sedachain.pubkey.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.pubkey.v1.MsgAddKey".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.pubkey.v1.MsgAddKey".into() }}
/// MsgAddKeyResponse defines the Msg/MsgAddKey response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgAddKeyResponse {
}
impl ::prost::Name for MsgAddKeyResponse {
const NAME: &'static str = "MsgAddKeyResponse";
const PACKAGE: &'static str = "sedachain.pubkey.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.pubkey.v1.MsgAddKeyResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.pubkey.v1.MsgAddKeyResponse".into() }}
// @@protoc_insertion_point(module)
