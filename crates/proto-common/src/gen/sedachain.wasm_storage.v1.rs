// @generated
// This file is @generated by prost-build.
/// OracleProgram is a wasm used for data request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleProgram {
    #[prost(bytes="bytes", tag="1")]
    pub hash: ::prost::bytes::Bytes,
    #[prost(bytes="bytes", tag="2")]
    pub bytecode: ::prost::bytes::Bytes,
    #[prost(message, optional, tag="3")]
    pub added_at: ::core::option::Option<::prost_types::Timestamp>,
    /// ExpirationHeight represents the block height at which the oracle
    /// program will be pruned. The value of zero means no expiration.
    #[prost(int64, tag="4")]
    pub expiration_height: i64,
}
impl ::prost::Name for OracleProgram {
const NAME: &'static str = "OracleProgram";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.OracleProgram".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.OracleProgram".into() }}
/// ExecutorWasm is a wasm used by some component in the protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorWasm {
    #[prost(bytes="bytes", tag="1")]
    pub hash: ::prost::bytes::Bytes,
    #[prost(bytes="bytes", tag="2")]
    pub bytecode: ::prost::bytes::Bytes,
    #[prost(message, optional, tag="3")]
    pub added_at: ::core::option::Option<::prost_types::Timestamp>,
}
impl ::prost::Name for ExecutorWasm {
const NAME: &'static str = "ExecutorWasm";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.ExecutorWasm".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.ExecutorWasm".into() }}
/// Params to define the max wasm size allowed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(int64, tag="1")]
    pub max_wasm_size: i64,
    /// WasmTTL represents the number of blocks a wasm's life is extended when it's
    /// created or used.
    #[prost(int64, tag="2")]
    pub wasm_ttl: i64,
}
impl ::prost::Name for Params {
const NAME: &'static str = "Params";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.Params".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.Params".into() }}
/// GenesisState defines wasm-storage module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag="2")]
    pub oracle_programs: ::prost::alloc::vec::Vec<OracleProgram>,
    #[prost(message, repeated, tag="3")]
    pub executor_wasms: ::prost::alloc::vec::Vec<ExecutorWasm>,
    #[prost(string, tag="4")]
    pub core_contract_registry: ::prost::alloc::string::String,
}
impl ::prost::Name for GenesisState {
const NAME: &'static str = "GenesisState";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.GenesisState".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.GenesisState".into() }}
/// The request message for QueryOracleProgram RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOracleProgramRequest {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryOracleProgramRequest {
const NAME: &'static str = "QueryOracleProgramRequest";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.QueryOracleProgramRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.QueryOracleProgramRequest".into() }}
/// The response message for QueryOracleProgram RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOracleProgramResponse {
    #[prost(message, optional, tag="1")]
    pub oracle_program: ::core::option::Option<OracleProgram>,
}
impl ::prost::Name for QueryOracleProgramResponse {
const NAME: &'static str = "QueryOracleProgramResponse";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.QueryOracleProgramResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.QueryOracleProgramResponse".into() }}
/// The request message for QueryOraclePrograms RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOracleProgramsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryOracleProgramsRequest {
const NAME: &'static str = "QueryOracleProgramsRequest";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.QueryOracleProgramsRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.QueryOracleProgramsRequest".into() }}
/// The response message for QueryOraclePrograms RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOracleProgramsResponse {
    #[prost(string, repeated, tag="1")]
    pub list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryOracleProgramsResponse {
const NAME: &'static str = "QueryOracleProgramsResponse";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.QueryOracleProgramsResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.QueryOracleProgramsResponse".into() }}
/// The request message for QueryExecutorWasm RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExecutorWasmRequest {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryExecutorWasmRequest {
const NAME: &'static str = "QueryExecutorWasmRequest";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.QueryExecutorWasmRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.QueryExecutorWasmRequest".into() }}
/// The response message for QueryExecutorWasm RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExecutorWasmResponse {
    #[prost(message, optional, tag="1")]
    pub wasm: ::core::option::Option<ExecutorWasm>,
}
impl ::prost::Name for QueryExecutorWasmResponse {
const NAME: &'static str = "QueryExecutorWasmResponse";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.QueryExecutorWasmResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.QueryExecutorWasmResponse".into() }}
/// The request message for QueryExecutorWasms RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QueryExecutorWasmsRequest {
}
impl ::prost::Name for QueryExecutorWasmsRequest {
const NAME: &'static str = "QueryExecutorWasmsRequest";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.QueryExecutorWasmsRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.QueryExecutorWasmsRequest".into() }}
/// The response message for QueryExecutorWasms RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExecutorWasmsResponse {
    #[prost(string, repeated, tag="1")]
    pub list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryExecutorWasmsResponse {
const NAME: &'static str = "QueryExecutorWasmsResponse";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.QueryExecutorWasmsResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.QueryExecutorWasmsResponse".into() }}
/// The request message for QueryCoreContractRegistry RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QueryCoreContractRegistryRequest {
}
impl ::prost::Name for QueryCoreContractRegistryRequest {
const NAME: &'static str = "QueryCoreContractRegistryRequest";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.QueryCoreContractRegistryRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.QueryCoreContractRegistryRequest".into() }}
/// The response message for QueryCoreContractRegistry RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCoreContractRegistryResponse {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryCoreContractRegistryResponse {
const NAME: &'static str = "QueryCoreContractRegistryResponse";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.QueryCoreContractRegistryResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.QueryCoreContractRegistryResponse".into() }}
/// The request message for the StoreOracleProgram method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStoreOracleProgram {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(bytes="bytes", tag="2")]
    pub wasm: ::prost::bytes::Bytes,
}
impl ::prost::Name for MsgStoreOracleProgram {
const NAME: &'static str = "MsgStoreOracleProgram";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.MsgStoreOracleProgram".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.MsgStoreOracleProgram".into() }}
/// The response message for the StoreOracleProgram method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStoreOracleProgramResponse {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgStoreOracleProgramResponse {
const NAME: &'static str = "MsgStoreOracleProgramResponse";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.MsgStoreOracleProgramResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.MsgStoreOracleProgramResponse".into() }}
/// The request message for the StoreExecutorWasm method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStoreExecutorWasm {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(bytes="bytes", tag="2")]
    pub wasm: ::prost::bytes::Bytes,
}
impl ::prost::Name for MsgStoreExecutorWasm {
const NAME: &'static str = "MsgStoreExecutorWasm";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.MsgStoreExecutorWasm".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.MsgStoreExecutorWasm".into() }}
/// The response message for the StoreExecutorWasm method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStoreExecutorWasmResponse {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgStoreExecutorWasmResponse {
const NAME: &'static str = "MsgStoreExecutorWasmResponse";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.MsgStoreExecutorWasmResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.MsgStoreExecutorWasmResponse".into() }}
/// The request message for the InstantiateCoreContract method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantiateCoreContract {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub code_id: u64,
    #[prost(string, tag="4")]
    pub label: ::prost::alloc::string::String,
    #[prost(bytes="bytes", tag="5")]
    pub msg: ::prost::bytes::Bytes,
    #[prost(message, repeated, tag="6")]
    pub funds: ::prost::alloc::vec::Vec<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(bytes="bytes", tag="7")]
    pub salt: ::prost::bytes::Bytes,
    #[prost(bool, tag="8")]
    pub fix_msg: bool,
}
impl ::prost::Name for MsgInstantiateCoreContract {
const NAME: &'static str = "MsgInstantiateCoreContract";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.MsgInstantiateCoreContract".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.MsgInstantiateCoreContract".into() }}
/// The response message for the InstantiateCoreContract method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantiateCoreContractResponse {
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgInstantiateCoreContractResponse {
const NAME: &'static str = "MsgInstantiateCoreContractResponse";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.MsgInstantiateCoreContractResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.MsgInstantiateCoreContractResponse".into() }}
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
impl ::prost::Name for MsgUpdateParams {
const NAME: &'static str = "MsgUpdateParams";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.MsgUpdateParams".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.MsgUpdateParams".into() }}
/// no data needs to be returned
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
impl ::prost::Name for MsgUpdateParamsResponse {
const NAME: &'static str = "MsgUpdateParamsResponse";
const PACKAGE: &'static str = "sedachain.wasm_storage.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.wasm_storage.v1.MsgUpdateParamsResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.wasm_storage.v1.MsgUpdateParamsResponse".into() }}
// @@protoc_insertion_point(module)
