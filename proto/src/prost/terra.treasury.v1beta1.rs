/// Params defines the parameters for the oracle module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag="1")]
    pub tax_policy: ::core::option::Option<PolicyConstraints>,
    #[prost(message, optional, tag="2")]
    pub reward_policy: ::core::option::Option<PolicyConstraints>,
    #[prost(string, tag="3")]
    pub seigniorage_burden_target: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub mining_increment: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub window_short: u64,
    #[prost(uint64, tag="6")]
    pub window_long: u64,
    #[prost(uint64, tag="7")]
    pub window_probation: u64,
}
/// PolicyConstraints - defines policy constraints can be applied in tax & reward policies
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyConstraints {
    #[prost(string, tag="1")]
    pub rate_min: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rate_max: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub cap: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="4")]
    pub change_rate_max: ::prost::alloc::string::String,
}
/// EpochTaxProceeds represents the tax amount
/// collected at the current epoch
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochTaxProceeds {
    #[prost(message, repeated, tag="1")]
    pub tax_proceeds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// EpochInitialIssuance represents initial issuance
/// of the currrent epoch
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochInitialIssuance {
    #[prost(message, repeated, tag="1")]
    pub issuance: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryTaxRateRequest is the request type for the Query/TaxRate RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxRateRequest {
}
/// QueryTaxRateResponse is response type for the
/// Query/TaxRate RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxRateResponse {
    #[prost(string, tag="1")]
    pub tax_rate: ::prost::alloc::string::String,
}
/// QueryTaxCapRequest is the request type for the Query/TaxCap RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxCapRequest {
    /// denom defines the denomination to query for.
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryTaxCapResponse is response type for the
/// Query/TaxCap RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxCapResponse {
    #[prost(string, tag="1")]
    pub tax_cap: ::prost::alloc::string::String,
}
/// QueryTaxCapsRequest is the request type for the Query/TaxCaps RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxCapsRequest {
}
/// QueryTaxCapsResponseItem is response item type for the
/// Query/TaxCaps RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxCapsResponseItem {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tax_cap: ::prost::alloc::string::String,
}
/// QueryTaxCapsResponse is response type for the
/// Query/TaxCaps RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxCapsResponse {
    #[prost(message, repeated, tag="1")]
    pub tax_caps: ::prost::alloc::vec::Vec<QueryTaxCapsResponseItem>,
}
/// QueryRewardWeightRequest is the request type for the Query/RewardWeight RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardWeightRequest {
}
/// QueryRewardWeightResponse is response type for the
/// Query/RewardWeight RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardWeightResponse {
    #[prost(string, tag="1")]
    pub reward_weight: ::prost::alloc::string::String,
}
/// QueryTaxProceedsRequest is the request type for the Query/TaxProceeds RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxProceedsRequest {
}
/// QueryTaxProceedsResponse is response type for the
/// Query/TaxProceeds RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTaxProceedsResponse {
    #[prost(message, repeated, tag="1")]
    pub tax_proceeds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QuerySeigniorageProceedsRequest is the request type for the Query/SeigniorageProceeds RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySeigniorageProceedsRequest {
}
/// QuerySeigniorageProceedsResponse is response type for the
/// Query/SeigniorageProceeds RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySeigniorageProceedsResponse {
    #[prost(string, tag="1")]
    pub seigniorage_proceeds: ::prost::alloc::string::String,
}
/// QueryIndicatorsRequest is the request type for the Query/Indicators RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIndicatorsRequest {
}
/// QueryIndicatorsResponse is response type for the
/// Query/Indicators RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIndicatorsResponse {
    #[prost(string, tag="1")]
    pub trl_year: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub trl_month: ::prost::alloc::string::String,
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
        /// TaxRate return the current tax rate
        pub async fn tax_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTaxRateRequest>,
        ) -> Result<tonic::Response<super::QueryTaxRateResponse>, tonic::Status> {
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
                "/terra.treasury.v1beta1.Query/TaxRate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// TaxCap returns the tax cap of a denom
        pub async fn tax_cap(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTaxCapRequest>,
        ) -> Result<tonic::Response<super::QueryTaxCapResponse>, tonic::Status> {
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
                "/terra.treasury.v1beta1.Query/TaxCap",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// TaxCaps returns the all tax caps
        pub async fn tax_caps(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTaxCapsRequest>,
        ) -> Result<tonic::Response<super::QueryTaxCapsResponse>, tonic::Status> {
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
                "/terra.treasury.v1beta1.Query/TaxCaps",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RewardWeight return the current reward weight
        pub async fn reward_weight(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRewardWeightRequest>,
        ) -> Result<tonic::Response<super::QueryRewardWeightResponse>, tonic::Status> {
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
                "/terra.treasury.v1beta1.Query/RewardWeight",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// SeigniorageProceeds return the current seigniorage proceeds
        pub async fn seigniorage_proceeds(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySeigniorageProceedsRequest>,
        ) -> Result<
            tonic::Response<super::QuerySeigniorageProceedsResponse>,
            tonic::Status,
        > {
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
                "/terra.treasury.v1beta1.Query/SeigniorageProceeds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// TaxProceeds return the current tax proceeds
        pub async fn tax_proceeds(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTaxProceedsRequest>,
        ) -> Result<tonic::Response<super::QueryTaxProceedsResponse>, tonic::Status> {
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
                "/terra.treasury.v1beta1.Query/TaxProceeds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Indicators return the current trl informations
        pub async fn indicators(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryIndicatorsRequest>,
        ) -> Result<tonic::Response<super::QueryIndicatorsResponse>, tonic::Status> {
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
                "/terra.treasury.v1beta1.Query/Indicators",
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
                "/terra.treasury.v1beta1.Query/Params",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState defines the oracle module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(string, tag="2")]
    pub tax_rate: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub reward_weight: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub tax_caps: ::prost::alloc::vec::Vec<TaxCap>,
    #[prost(message, repeated, tag="5")]
    pub tax_proceeds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag="6")]
    pub epoch_initial_issuance: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag="7")]
    pub epoch_states: ::prost::alloc::vec::Vec<EpochState>,
}
/// TaxCap is the max tax amount can be charged for the given denom
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaxCap {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tax_cap: ::prost::alloc::string::String,
}
/// EpochState is the record for each epoch state
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochState {
    #[prost(uint64, tag="1")]
    pub epoch: u64,
    #[prost(string, tag="2")]
    pub tax_reward: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub seigniorage_reward: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub total_staked_luna: ::prost::alloc::string::String,
}
