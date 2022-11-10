/// EmbedAccessTokenRequest is the request message for the Get rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedAccessTokenRequest {
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<embed_access_token_request::Filter>,
}
/// Nested message and enum types in `EmbedAccessTokenRequest`.
pub mod embed_access_token_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub operator: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub value: ::prost::alloc::string::String,
    }
}
/// EmbedAccessTokenResponse is the response message for the Get rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedAccessTokenResponse {
    #[prost(string, tag = "1")]
    pub access_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod embed_access_token_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// EmbedAccessTokenService is the service definition for the registry embed-access-token endpoint.
    #[derive(Debug, Clone)]
    pub struct EmbedAccessTokenServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EmbedAccessTokenServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> EmbedAccessTokenServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            EmbedAccessTokenServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///  Get is the rpc handling access token retrieval from the SDK
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::EmbedAccessTokenRequest>,
        ) -> Result<tonic::Response<super::EmbedAccessTokenResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
