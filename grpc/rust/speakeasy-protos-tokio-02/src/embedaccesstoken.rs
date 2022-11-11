/// EmbedAccessTokenRequest is the request message for the Get rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedAccessTokenRequest {
    /// filters is the list of filters to apply to the request.
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<embed_access_token_request::Filter>,
    /// customer_id is the customer id associated with the access token.
    #[prost(string, optional, tag = "2")]
    pub customer_id: ::core::option::Option<::prost::alloc::string::String>,
    /// display_name is the display name associated with the access token.
    #[prost(string, optional, tag = "3")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    /// jwt_custom_claims is the custom claims associated with the access token.
    #[prost(map = "string, string", tag = "4")]
    pub jwt_custom_claims:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// permissions is the permissions associated with the access token.
    #[prost(map = "string, bool", tag = "5")]
    pub permissions: ::std::collections::HashMap<::prost::alloc::string::String, bool>,
}
/// Nested message and enum types in `EmbedAccessTokenRequest`.
pub mod embed_access_token_request {
    /// Filter represents a filter to be applied to the request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        /// key is the key of the filter.
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        /// operator is the operator of the filter.
        #[prost(string, tag = "2")]
        pub operator: ::prost::alloc::string::String,
        /// value is the value of the filter.
        #[prost(string, tag = "3")]
        pub value: ::prost::alloc::string::String,
    }
}
/// EmbedAccessTokenResponse is the response message for the Get rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedAccessTokenResponse {
    /// access_token is a token that allows access to an embed.
    #[prost(string, tag = "1")]
    pub access_token: ::prost::alloc::string::String,
}
