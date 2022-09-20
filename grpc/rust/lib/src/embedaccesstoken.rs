/// EmbedAccessTokenRequest is the request message for the Get rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedAccessTokenRequest {
    #[prost(message, repeated, tag = "1")]
    pub filters: ::std::vec::Vec<embed_access_token_request::Filter>,
}
pub mod embed_access_token_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        #[prost(string, tag = "1")]
        pub key: std::string::String,
        #[prost(string, tag = "2")]
        pub operator: std::string::String,
        #[prost(string, tag = "3")]
        pub value: std::string::String,
    }
}
/// EmbedAccessTokenResponse is the response message for the Get rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedAccessTokenResponse {
    #[prost(string, tag = "1")]
    pub access_token: std::string::String,
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
    impl<T: Clone> Clone for EmbedAccessTokenServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for EmbedAccessTokenServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "EmbedAccessTokenServiceClient {{ ... }}")
        }
    }
}
