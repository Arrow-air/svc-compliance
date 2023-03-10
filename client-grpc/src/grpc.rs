/// FlightPlanRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightPlanRequest {
    /// Flight Plan Id
    #[prost(string, tag = "1")]
    pub flight_plan_id: ::prost::alloc::string::String,
    /// JSON data of the flight plan
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
/// FlightPlanResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightPlanResponse {
    /// Flight Plan Id
    #[prost(string, tag = "1")]
    pub flight_plan_id: ::prost::alloc::string::String,
    /// JSON data of the flight plan
    #[prost(bool, tag = "2")]
    pub submitted: bool,
    /// Optional error or warning message
    #[prost(string, optional, tag = "3")]
    pub result: ::core::option::Option<::prost::alloc::string::String>,
}
/// FlightReleaseRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightReleaseRequest {
    /// Flight Plan Id
    #[prost(string, tag = "1")]
    pub flight_plan_id: ::prost::alloc::string::String,
    /// JSON data of the flight plan
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
/// FlightReleaseResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightReleaseResponse {
    /// Flight Plan Id
    #[prost(string, tag = "1")]
    pub flight_plan_id: ::prost::alloc::string::String,
    /// JSON data of the flight plan
    #[prost(bool, tag = "2")]
    pub released: bool,
    /// Optional error or warning message
    #[prost(string, optional, tag = "3")]
    pub result: ::core::option::Option<::prost::alloc::string::String>,
}
/// Are you Ready?
///
/// No arguments
#[derive(Eq, Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsReady {}
/// I'm Ready
#[derive(Eq, Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyResponse {
    /// True if ready
    #[prost(bool, tag = "1")]
    pub ready: bool,
}
/// Generated client implementations.
pub mod compliance_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// ComplianceRpc
    #[derive(Debug, Clone)]
    pub struct ComplianceRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ComplianceRpcClient<tonic::transport::Channel> {
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
    impl<T> ComplianceRpcClient<T>
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
        ) -> ComplianceRpcClient<InterceptedService<T, F>>
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
            ComplianceRpcClient::new(InterceptedService::new(inner, interceptor))
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
        /// is ready heartbeat
        pub async fn is_ready(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryIsReady>,
        ) -> Result<tonic::Response<super::ReadyResponse>, tonic::Status> {
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
                "/grpc.ComplianceRpc/isReady",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// submit flight plan
        pub async fn submit_flight_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::FlightPlanRequest>,
        ) -> Result<tonic::Response<super::FlightPlanResponse>, tonic::Status> {
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
                "/grpc.ComplianceRpc/submitFlightPlan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// release flight plan
        pub async fn request_flight_release(
            &mut self,
            request: impl tonic::IntoRequest<super::FlightReleaseRequest>,
        ) -> Result<tonic::Response<super::FlightReleaseResponse>, tonic::Status> {
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
                "/grpc.ComplianceRpc/requestFlightRelease",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
