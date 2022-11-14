/// IngestRequest is the request message for the ingest rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestRequest {
    /// har is string containing a HTTP Archive 1.2 formatted file contents.
    #[prost(string, tag = "1")]
    pub har: ::prost::alloc::string::String,
    /// path_hint is a hint to the ingest service about the structure of the request path.
    #[prost(string, tag = "2")]
    pub path_hint: ::prost::alloc::string::String,
    /// api_id is used to associate requests with a particular Api in the Speakeasy platform.
    #[prost(string, tag = "3")]
    pub api_id: ::prost::alloc::string::String,
    /// version_id is used to associate requests with a particular version of an Api in the Speakeasy platform.
    #[prost(string, tag = "4")]
    pub version_id: ::prost::alloc::string::String,
    /// customer_id is the id of the customer who is making the request.
    #[prost(string, tag = "5")]
    pub customer_id: ::prost::alloc::string::String,
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
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// request_cookie_masks contains a map of cookie keys to the masks applied to them.
        #[prost(map = "string, string", tag = "2")]
        pub request_cookie_masks: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// request_field_masks_string contains a map of string body fields to the masks applied to them.
        #[prost(map = "string, string", tag = "3")]
        pub request_field_masks_string: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// request_field_masks_number contains a map of number body fields to the masks applied to them.
        #[prost(map = "string, string", tag = "4")]
        pub request_field_masks_number: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// response_header_masks contains a map of header keys to the masks applied to them.
        #[prost(map = "string, string", tag = "5")]
        pub response_header_masks: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// response_cookie_masks contains a map of cookie keys to the masks applied to them.
        #[prost(map = "string, string", tag = "6")]
        pub response_cookie_masks: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// response_field_masks_string contains a map of string body fields to the masks applied to them.
        #[prost(map = "string, string", tag = "7")]
        pub response_field_masks_string: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// response_field_masks_number contains a map of number body fields to the masks applied to them.
        #[prost(map = "string, string", tag = "8")]
        pub response_field_masks_number: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// query_string_masks contains a map of query string keys to the masks applied to them.
        #[prost(map = "string, string", tag = "9")]
        pub query_string_masks: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
}
/// IngestResponse is the response message for the ingest rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestResponse {}
/// Generated client implementations.
pub mod ingest_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// IngestService is the service definition for the registry ingest endpoint.
    #[derive(Debug, Clone)]
    pub struct IngestServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IngestServiceClient<tonic::transport::Channel> {
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
    impl<T> IngestServiceClient<T>
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
        ) -> IngestServiceClient<InterceptedService<T, F>>
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
            IngestServiceClient::new(InterceptedService::new(inner, interceptor))
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
        ///  Ingest is the rpc handling ingest from the SDK.
        pub async fn ingest(
            &mut self,
            request: impl tonic::IntoRequest<super::IngestRequest>,
        ) -> Result<tonic::Response<super::IngestResponse>, tonic::Status> {
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
                "/ingest.IngestService/Ingest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
