/// EmbedAccessTokenRequest is the request message for the Get rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedAccessTokenRequest {
    /// filters is the list of filters to apply to the request.
    #[prost(message, repeated, tag = "1")]
    pub filters: ::std::vec::Vec<embed_access_token_request::Filter>,
    /// customer_id is the customer id associated with the access token.
    #[prost(string, optional, tag = "2")]
    pub customer_id: ::core::option::Option<::std::string::String>,
    /// display_name is the display name associated with the access token.
    #[prost(string, optional, tag = "3")]
    pub display_name: ::core::option::Option<::std::string::String>,
    /// jwt_custom_claims is the custom claims associated with the access token.
    #[prost(map = "string, string", tag = "4")]
    pub jwt_custom_claims:
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    /// permissions is the permissions associated with the access token.
    #[prost(map = "string, bool", tag = "5")]
    pub permissions: ::std::collections::HashMap<::std::string::String, bool>,
}
/// Nested message and enum types in `EmbedAccessTokenRequest`.
pub mod embed_access_token_request {
    /// Filter represents a filter to be applied to the request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        /// key is the key of the filter.
        #[prost(string, tag = "1")]
        pub key: ::std::string::String,
        /// operator is the operator of the filter.
        #[prost(string, tag = "2")]
        pub operator: ::std::string::String,
        /// value is the value of the filter.
        #[prost(string, tag = "3")]
        pub value: ::std::string::String,
    }
}
/// EmbedAccessTokenResponse is the response message for the Get rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedAccessTokenResponse {
    /// access_token is a token that allows access to an embed.
    #[prost(string, tag = "1")]
    pub access_token: ::std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod embed_access_token_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " EmbedAccessTokenService is the service definition for the registry embed-access-token endpoint."]
    pub struct EmbedAccessTokenServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EmbedAccessTokenServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> EmbedAccessTokenServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = "  Get is the rpc handling access token retrieval from the SDK"]
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::EmbedAccessTokenRequest>,
        ) -> Result<tonic::Response<super::EmbedAccessTokenResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/embedaccesstoken.EmbedAccessTokenService/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }

}
