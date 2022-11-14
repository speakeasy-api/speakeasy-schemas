/// IngestRequest is the request message for the ingest rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestRequest {
    /// har is string containing a HTTP Archive 1.2 formatted file contents.
    #[prost(string, tag = "1")]
    pub har: ::std::string::String,
    /// path_hint is a hint to the ingest service about the structure of the request path.
    #[prost(string, tag = "2")]
    pub path_hint: ::std::string::String,
    /// api_id is used to associate requests with a particular Api in the Speakeasy platform.
    #[prost(string, tag = "3")]
    pub api_id: ::std::string::String,
    /// version_id is used to associate requests with a particular version of an Api in the Speakeasy platform.
    #[prost(string, tag = "4")]
    pub version_id: ::std::string::String,
    /// customer_id is the id of the customer who is making the request.
    #[prost(string, tag = "5")]
    pub customer_id: ::std::string::String,
    /// masking_metadata contains information about any masking added to the har.
    #[prost(message, optional, tag = "6")]
    pub masking_metadata: ::core::option::Option<ingest_request::MaskingMetadata>,
}
/// Nested message and enum types in `IngestRequest`.
pub mod ingest_request {
    /// MaskingMetadata contains information about any masking added to the har.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MaskingMetadata {
        /// request_header_masks contains a map of header keys to the masks applied to them.
        #[prost(map = "string, string", tag = "1")]
        pub request_header_masks: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        /// request_cookie_masks contains a map of cookie keys to the masks applied to them.
        #[prost(map = "string, string", tag = "2")]
        pub request_cookie_masks: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        /// request_field_masks_string contains a map of string body fields to the masks applied to them.
        #[prost(map = "string, string", tag = "3")]
        pub request_field_masks_string: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        /// request_field_masks_number contains a map of number body fields to the masks applied to them.
        #[prost(map = "string, string", tag = "4")]
        pub request_field_masks_number: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        /// response_header_masks contains a map of header keys to the masks applied to them.
        #[prost(map = "string, string", tag = "5")]
        pub response_header_masks: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        /// response_cookie_masks contains a map of cookie keys to the masks applied to them.
        #[prost(map = "string, string", tag = "6")]
        pub response_cookie_masks: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        /// response_field_masks_string contains a map of string body fields to the masks applied to them.
        #[prost(map = "string, string", tag = "7")]
        pub response_field_masks_string: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        /// response_field_masks_number contains a map of number body fields to the masks applied to them.
        #[prost(map = "string, string", tag = "8")]
        pub response_field_masks_number: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
        /// query_string_masks contains a map of query string keys to the masks applied to them.
        #[prost(map = "string, string", tag = "9")]
        pub query_string_masks: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >,
    }
}
/// IngestResponse is the response message for the ingest rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestResponse {}
