// @generated
// This file is @generated by prost-build.
/// Batch is an aggregation of data request results along with validator
/// signatures used to prove these results on destination chains.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Batch {
    /// batch_number is a unique identifier of the batch incremented
    /// every time a batch is created.
    #[prost(uint64, tag="1")]
    pub batch_number: u64,
    /// block_height is the height at which the batch was created.
    #[prost(int64, tag="2")]
    pub block_height: i64,
    /// current_data_result_root is the hex-encoded root of the data result
    /// merkle tree.
    #[prost(string, tag="3")]
    pub current_data_result_root: ::prost::alloc::string::String,
    /// data_result_root is the hex-encoded "super root" of the previous
    /// data result and current data result roots.
    #[prost(string, tag="4")]
    pub data_result_root: ::prost::alloc::string::String,
    /// validator_root is the hex-encoded root of the validator merkle
    /// tree.
    #[prost(string, tag="5")]
    pub validator_root: ::prost::alloc::string::String,
    /// batch_id is the Keccack-256 hash of the batch content.
    #[prost(bytes="bytes", tag="6")]
    pub batch_id: ::prost::bytes::Bytes,
    /// proving_medatada is a field for additional proving data.
    #[prost(bytes="bytes", tag="7")]
    pub proving_medatada: ::prost::bytes::Bytes,
}
impl ::prost::Name for Batch {
const NAME: &'static str = "Batch";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.Batch".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.Batch".into() }}
/// TreeEntries are the given batch's data result tree entries and
/// validator tree entries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TreeEntries {
    /// batch_number is the identifier of the batch the tree entries from.
    #[prost(uint64, tag="1")]
    pub batch_number: u64,
    /// data_result_entries are the entries (unhashed leaf contents) of
    /// the data result tree.
    #[prost(bytes="bytes", repeated, tag="2")]
    pub data_result_entries: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
    /// validator_entries are the entries (unhashed leaf contents) of
    /// the validator tree.
    #[prost(bytes="bytes", repeated, tag="3")]
    pub validator_entries: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
}
impl ::prost::Name for TreeEntries {
const NAME: &'static str = "TreeEntries";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.TreeEntries".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.TreeEntries".into() }}
/// BatchSignatures contains basic validator data and its batch signatures
/// under various cryptographic schemes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchSignatures {
    #[prost(string, tag="1")]
    pub validator_addr: ::prost::alloc::string::String,
    #[prost(bytes="bytes", tag="2")]
    pub signatures: ::prost::bytes::Bytes,
}
impl ::prost::Name for BatchSignatures {
const NAME: &'static str = "BatchSignatures";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.BatchSignatures".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.BatchSignatures".into() }}
/// Params is a list of parameters which can be changed through governance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Params {
    /// validator_set_trim_percent is the percentage of the validator
    /// set to store in the validator merkle tree in the batch.
    #[prost(uint32, tag="1")]
    pub validator_set_trim_percent: u32,
}
impl ::prost::Name for Params {
const NAME: &'static str = "Params";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.Params".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.Params".into() }}
/// DataResult represents the result of a resolved data request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataResult {
    /// id is the Keccack-256 hash of the data result.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// dr_id is the data request identifier.
    #[prost(string, tag="2")]
    pub dr_id: ::prost::alloc::string::String,
    /// version is a semantic version string.
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    /// block_height is the height at which the data request was tallied.
    #[prost(uint64, tag="4")]
    pub block_height: u64,
    /// exit_code is the exit code of the tally wasm binary execution.
    #[prost(uint32, tag="5")]
    pub exit_code: u32,
    /// gas_used is the gas used by the data request execution.
    #[prost(uint64, tag="6")]
    pub gas_used: u64,
    /// result is the result of the tally wasm binary execution.
    #[prost(bytes="bytes", tag="7")]
    pub result: ::prost::bytes::Bytes,
    /// payback_address is the payback address set by the relayer.
    #[prost(string, tag="8")]
    pub payback_address: ::prost::alloc::string::String,
    /// seda_payload is the payload set by SEDA Protocol (e.g. OEV-enabled
    /// data requests)
    #[prost(string, tag="9")]
    pub seda_payload: ::prost::alloc::string::String,
    /// consensus indicates whether consensus was reached in the tally
    /// process.
    #[prost(bool, tag="10")]
    pub consensus: bool,
}
impl ::prost::Name for DataResult {
const NAME: &'static str = "DataResult";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.DataResult".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.DataResult".into() }}
/// GenesisState defines the batching module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// current_batch_number is the batch number of the most recently-
    /// created batch.
    #[prost(uint64, tag="1")]
    pub current_batch_number: u64,
    #[prost(message, repeated, tag="2")]
    pub batches: ::prost::alloc::vec::Vec<Batch>,
    #[prost(message, repeated, tag="3")]
    pub tree_entries: ::prost::alloc::vec::Vec<TreeEntries>,
    #[prost(message, repeated, tag="4")]
    pub data_results: ::prost::alloc::vec::Vec<DataResult>,
    #[prost(message, repeated, tag="5")]
    pub batch_assignments: ::prost::alloc::vec::Vec<BatchAssignment>,
    #[prost(message, optional, tag="6")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for GenesisState {
const NAME: &'static str = "GenesisState";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.GenesisState".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.GenesisState".into() }}
/// BatchAssignment represents a batch assignment for genesis export
/// and import.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchAssignment {
    #[prost(uint64, tag="1")]
    pub batch_number: u64,
    #[prost(string, tag="2")]
    pub data_request_id: ::prost::alloc::string::String,
}
impl ::prost::Name for BatchAssignment {
const NAME: &'static str = "BatchAssignment";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.BatchAssignment".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.BatchAssignment".into() }}
/// The request message for QueryBatch RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QueryBatchRequest {
    #[prost(uint64, tag="1")]
    pub batch_number: u64,
}
impl ::prost::Name for QueryBatchRequest {
const NAME: &'static str = "QueryBatchRequest";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryBatchRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryBatchRequest".into() }}
/// The response message for QueryBatch RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchResponse {
    #[prost(message, optional, tag="1")]
    pub batch: ::core::option::Option<Batch>,
}
impl ::prost::Name for QueryBatchResponse {
const NAME: &'static str = "QueryBatchResponse";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryBatchResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryBatchResponse".into() }}
/// The request message for BatchForHeight RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QueryBatchForHeightRequest {
    #[prost(int64, tag="1")]
    pub block_height: i64,
}
impl ::prost::Name for QueryBatchForHeightRequest {
const NAME: &'static str = "QueryBatchForHeightRequest";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryBatchForHeightRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryBatchForHeightRequest".into() }}
/// The response message for BatchForHeight RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchForHeightResponse {
    #[prost(message, optional, tag="1")]
    pub batch: ::core::option::Option<Batch>,
}
impl ::prost::Name for QueryBatchForHeightResponse {
const NAME: &'static str = "QueryBatchForHeightResponse";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryBatchForHeightResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryBatchForHeightResponse".into() }}
/// The request message for QueryBatches RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryBatchesRequest {
const NAME: &'static str = "QueryBatchesRequest";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryBatchesRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryBatchesRequest".into() }}
/// The response message for QueryBatches RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchesResponse {
    #[prost(message, repeated, tag="1")]
    pub batches: ::prost::alloc::vec::Vec<Batch>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryBatchesResponse {
const NAME: &'static str = "QueryBatchesResponse";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryBatchesResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryBatchesResponse".into() }}
/// The request message for QueryTreeEntries RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QueryTreeEntriesRequest {
    #[prost(uint64, tag="1")]
    pub batch_number: u64,
}
impl ::prost::Name for QueryTreeEntriesRequest {
const NAME: &'static str = "QueryTreeEntriesRequest";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryTreeEntriesRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryTreeEntriesRequest".into() }}
/// The response message for QueryTreeEntries RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTreeEntriesResponse {
    #[prost(message, optional, tag="1")]
    pub entries: ::core::option::Option<TreeEntries>,
}
impl ::prost::Name for QueryTreeEntriesResponse {
const NAME: &'static str = "QueryTreeEntriesResponse";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryTreeEntriesResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryTreeEntriesResponse".into() }}
/// The request message for QueryBatchSignaturesRequest RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QueryBatchSignaturesRequest {
    #[prost(uint64, tag="1")]
    pub batch_number: u64,
}
impl ::prost::Name for QueryBatchSignaturesRequest {
const NAME: &'static str = "QueryBatchSignaturesRequest";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryBatchSignaturesRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryBatchSignaturesRequest".into() }}
/// The response message for QueryQueryBatchSignatures RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchSignaturesResponse {
    #[prost(message, repeated, tag="1")]
    pub batch_sigs: ::prost::alloc::vec::Vec<BatchSignatures>,
}
impl ::prost::Name for QueryBatchSignaturesResponse {
const NAME: &'static str = "QueryBatchSignaturesResponse";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryBatchSignaturesResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryBatchSignaturesResponse".into() }}
/// The request message for QueryDataResult RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDataResultRequest {
    #[prost(string, tag="1")]
    pub data_request_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryDataResultRequest {
const NAME: &'static str = "QueryDataResultRequest";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryDataResultRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryDataResultRequest".into() }}
/// The response message for QueryDataResult RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDataResultResponse {
    #[prost(message, optional, tag="1")]
    pub data_result: ::core::option::Option<DataResult>,
}
impl ::prost::Name for QueryDataResultResponse {
const NAME: &'static str = "QueryDataResultResponse";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryDataResultResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryDataResultResponse".into() }}
/// The request message for QueryBatchAssignment RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchAssignmentRequest {
    #[prost(string, tag="1")]
    pub data_request_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryBatchAssignmentRequest {
const NAME: &'static str = "QueryBatchAssignmentRequest";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryBatchAssignmentRequest".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryBatchAssignmentRequest".into() }}
/// The response message for QueryBatchAssignment RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QueryBatchAssignmentResponse {
    #[prost(uint64, tag="1")]
    pub batch_number: u64,
}
impl ::prost::Name for QueryBatchAssignmentResponse {
const NAME: &'static str = "QueryBatchAssignmentResponse";
const PACKAGE: &'static str = "sedachain.batching.v1";
fn full_name() -> ::prost::alloc::string::String { "sedachain.batching.v1.QueryBatchAssignmentResponse".into() }fn type_url() -> ::prost::alloc::string::String { "/sedachain.batching.v1.QueryBatchAssignmentResponse".into() }}
// @@protoc_insertion_point(module)