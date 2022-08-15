/// MsgSwap represents a message to swap coin to another denom.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwap {
    #[prost(string, tag="1")]
    pub trader: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub offer_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="3")]
    pub ask_denom: ::prost::alloc::string::String,
}
/// MsgSwapResponse defines the Msg/Swap response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapResponse {
    #[prost(message, optional, tag="1")]
    pub swap_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="2")]
    pub swap_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSwapSend represents a message to swap coin and send all result coin to recipient
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapSend {
    #[prost(string, tag="1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub offer_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="4")]
    pub ask_denom: ::prost::alloc::string::String,
}
/// MsgSwapSendResponse defines the Msg/SwapSend response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapSendResponse {
    #[prost(message, optional, tag="1")]
    pub swap_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="2")]
    pub swap_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the market Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    // Workaround, add feature manually, it could be fixed after https://github.com/hyperium/tonic/issues/491
    #[cfg(feature = "transport")]
    impl MsgClient<tonic::transport::Channel> {
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
    impl<T> MsgClient<T>
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
        ) -> MsgClient<InterceptedService<T, F>>
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
            MsgClient::new(InterceptedService::new(inner, interceptor))
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
        /// Swap defines a method for swapping coin from one denom to another
        /// denom.
        pub async fn swap(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSwap>,
        ) -> Result<tonic::Response<super::MsgSwapResponse>, tonic::Status> {
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
                "/terra.market.v1beta1.Msg/Swap",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// SwapSend defines a method for swapping and sending coin from a account to other
        /// account.
        pub async fn swap_send(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSwapSend>,
        ) -> Result<tonic::Response<super::MsgSwapSendResponse>, tonic::Status> {
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
                "/terra.market.v1beta1.Msg/SwapSend",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Params defines the parameters for the market module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(bytes="vec", tag="1")]
    pub base_pool: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub pool_recovery_period: u64,
    #[prost(bytes="vec", tag="3")]
    pub min_stability_spread: ::prost::alloc::vec::Vec<u8>,
}
/// QuerySwapRequest is the request type for the Query/Swap RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySwapRequest {
    /// offer_coin defines the coin being offered (i.e. 1000000uluna)
    #[prost(string, tag="1")]
    pub offer_coin: ::prost::alloc::string::String,
    /// ask_denom defines the denom of the coin to swap to
    #[prost(string, tag="2")]
    pub ask_denom: ::prost::alloc::string::String,
}
/// QuerySwapResponse is the response type for the Query/Swap RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySwapResponse {
    /// return_coin defines the coin returned as a result of the swap simulation.
    #[prost(message, optional, tag="1")]
    pub return_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryTerraPoolDeltaRequest is the request type for the Query/TerraPoolDelta RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTerraPoolDeltaRequest {
}
/// QueryTerraPoolDeltaResponse is the response type for the Query/TerraPoolDelta RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTerraPoolDeltaResponse {
    /// terra_pool_delta defines the gap between the TerraPool and the TerraBasePool
    #[prost(bytes="vec", tag="1")]
    pub terra_pool_delta: ::prost::alloc::vec::Vec<u8>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query defines the gRPC querier service.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    // Workaround, add feature manually, it could be fixed after https://github.com/hyperium/tonic/issues/491
    #[cfg(feature = "transport")]
    impl QueryClient<tonic::transport::Channel> {
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
    impl<T> QueryClient<T>
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
        ) -> QueryClient<InterceptedService<T, F>>
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
            QueryClient::new(InterceptedService::new(inner, interceptor))
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
        /// Swap returns simulated swap amount.
        pub async fn swap(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySwapRequest>,
        ) -> Result<tonic::Response<super::QuerySwapResponse>, tonic::Status> {
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
                "/terra.market.v1beta1.Query/Swap",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// TerraPoolDelta returns terra_pool_delta amount.
        pub async fn terra_pool_delta(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTerraPoolDeltaRequest>,
        ) -> Result<tonic::Response<super::QueryTerraPoolDeltaResponse>, tonic::Status> {
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
                "/terra.market.v1beta1.Query/TerraPoolDelta",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Params queries all parameters.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
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
                "/terra.market.v1beta1.Query/Params",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState defines the market module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// the gap between the TerraPool and the BasePool
    #[prost(bytes="vec", tag="2")]
    pub terra_pool_delta: ::prost::alloc::vec::Vec<u8>,
}
