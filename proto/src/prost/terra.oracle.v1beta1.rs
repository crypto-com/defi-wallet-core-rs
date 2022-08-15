/// MsgAggregateExchangeRatePrevote represents a message to submit
/// aggregate exchange rate prevote.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAggregateExchangeRatePrevote {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub feeder: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub validator: ::prost::alloc::string::String,
}
/// MsgAggregateExchangeRatePrevoteResponse defines the Msg/AggregateExchangeRatePrevote response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAggregateExchangeRatePrevoteResponse {
}
/// MsgAggregateExchangeRateVote represents a message to submit
/// aggregate exchange rate vote.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAggregateExchangeRateVote {
    #[prost(string, tag="1")]
    pub salt: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub exchange_rates: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub feeder: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub validator: ::prost::alloc::string::String,
}
/// MsgAggregateExchangeRateVoteResponse defines the Msg/AggregateExchangeRateVote response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAggregateExchangeRateVoteResponse {
}
/// MsgDelegateFeedConsent represents a message to
/// delegate oracle voting rights to another address.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelegateFeedConsent {
    #[prost(string, tag="1")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub delegate: ::prost::alloc::string::String,
}
/// MsgDelegateFeedConsentResponse defines the Msg/DelegateFeedConsent response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelegateFeedConsentResponse {
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the oracle Msg service.
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
        /// AggregateExchangeRatePrevote defines a method for submitting
        /// aggregate exchange rate prevote
        pub async fn aggregate_exchange_rate_prevote(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAggregateExchangeRatePrevote>,
        ) -> Result<
            tonic::Response<super::MsgAggregateExchangeRatePrevoteResponse>,
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
                "/terra.oracle.v1beta1.Msg/AggregateExchangeRatePrevote",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AggregateExchangeRateVote defines a method for submitting
        /// aggregate exchange rate vote
        pub async fn aggregate_exchange_rate_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAggregateExchangeRateVote>,
        ) -> Result<
            tonic::Response<super::MsgAggregateExchangeRateVoteResponse>,
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
                "/terra.oracle.v1beta1.Msg/AggregateExchangeRateVote",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DelegateFeedConsent defines a method for setting the feeder delegation
        pub async fn delegate_feed_consent(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDelegateFeedConsent>,
        ) -> Result<
            tonic::Response<super::MsgDelegateFeedConsentResponse>,
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
                "/terra.oracle.v1beta1.Msg/DelegateFeedConsent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Params defines the parameters for the oracle module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag="1")]
    pub vote_period: u64,
    #[prost(string, tag="2")]
    pub vote_threshold: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub reward_band: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub reward_distribution_window: u64,
    #[prost(message, repeated, tag="5")]
    pub whitelist: ::prost::alloc::vec::Vec<Denom>,
    #[prost(string, tag="6")]
    pub slash_fraction: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub slash_window: u64,
    #[prost(string, tag="8")]
    pub min_valid_per_window: ::prost::alloc::string::String,
}
/// Denom - the object to hold configurations of each denom
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Denom {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tobin_tax: ::prost::alloc::string::String,
}
/// struct for aggregate prevoting on the ExchangeRateVote.
/// The purpose of aggregate prevote is to hide vote exchange rates with hash
/// which is formatted as hex string in SHA256("{salt}:{exchange rate}{denom},...,{exchange rate}{denom}:{voter}")
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateExchangeRatePrevote {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub voter: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub submit_block: u64,
}
/// MsgAggregateExchangeRateVote - struct for voting on
/// the exchange rates of Luna denominated in various Terra assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateExchangeRateVote {
    #[prost(message, repeated, tag="1")]
    pub exchange_rate_tuples: ::prost::alloc::vec::Vec<ExchangeRateTuple>,
    #[prost(string, tag="2")]
    pub voter: ::prost::alloc::string::String,
}
/// ExchangeRateTuple - struct to store interpreted exchange rates data to store
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeRateTuple {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub exchange_rate: ::prost::alloc::string::String,
}
/// QueryExchangeRateRequest is the request type for the Query/ExchangeRate RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeRateRequest {
    /// denom defines the denomination to query for.
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryExchangeRateResponse is response type for the
/// Query/ExchangeRate RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeRateResponse {
    /// exchange_rate defines the exchange rate of Luna denominated in various Terra
    #[prost(string, tag="1")]
    pub exchange_rate: ::prost::alloc::string::String,
}
/// QueryExchangeRatesRequest is the request type for the Query/ExchangeRates RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeRatesRequest {
}
/// QueryExchangeRatesResponse is response type for the
/// Query/ExchangeRates RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeRatesResponse {
    /// exchange_rates defines a list of the exchange rate for all whitelisted denoms.
    #[prost(message, repeated, tag="1")]
    pub exchange_rates: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
}
/// QueryTobinTaxRequest is the request type for the Query/TobinTax RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTobinTaxRequest {
    /// denom defines the denomination to query for.
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryTobinTaxResponse is response type for the
/// Query/TobinTax RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTobinTaxResponse {
    /// tobin_taxe defines the tobin tax of a denom
    #[prost(string, tag="1")]
    pub tobin_tax: ::prost::alloc::string::String,
}
/// QueryTobinTaxesRequest is the request type for the Query/TobinTaxes RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTobinTaxesRequest {
}
/// QueryTobinTaxesResponse is response type for the
/// Query/TobinTaxes RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTobinTaxesResponse {
    /// tobin_taxes defines a list of the tobin tax of all whitelisted denoms
    #[prost(message, repeated, tag="1")]
    pub tobin_taxes: ::prost::alloc::vec::Vec<Denom>,
}
/// QueryActivesRequest is the request type for the Query/Actives RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActivesRequest {
}
/// QueryActivesResponse is response type for the
/// Query/Actives RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActivesResponse {
    /// actives defines a list of the denomination which oracle prices aggreed upon.
    #[prost(string, repeated, tag="1")]
    pub actives: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryVoteTargetsRequest is the request type for the Query/VoteTargets RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVoteTargetsRequest {
}
/// QueryVoteTargetsResponse is response type for the
/// Query/VoteTargets RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVoteTargetsResponse {
    /// vote_targets defines a list of the denomination in which everyone
    /// should vote in the current vote period.
    #[prost(string, repeated, tag="1")]
    pub vote_targets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryFeederDelegationRequest is the request type for the Query/FeederDelegation RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeederDelegationRequest {
    /// validator defines the validator address to query for.
    #[prost(string, tag="1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryFeederDelegationResponse is response type for the
/// Query/FeederDelegation RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeederDelegationResponse {
    /// feeder_addr defines the feeder delegation of a validator
    #[prost(string, tag="1")]
    pub feeder_addr: ::prost::alloc::string::String,
}
/// QueryMissCounterRequest is the request type for the Query/MissCounter RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMissCounterRequest {
    /// validator defines the validator address to query for.
    #[prost(string, tag="1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryMissCounterResponse is response type for the
/// Query/MissCounter RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMissCounterResponse {
    /// miss_counter defines the oracle miss counter of a validator
    #[prost(uint64, tag="1")]
    pub miss_counter: u64,
}
/// QueryAggregatePrevoteRequest is the request type for the Query/AggregatePrevote RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatePrevoteRequest {
    /// validator defines the validator address to query for.
    #[prost(string, tag="1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryAggregatePrevoteResponse is response type for the
/// Query/AggregatePrevote RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatePrevoteResponse {
    /// aggregate_prevote defines oracle aggregate prevote submitted by a validator in the current vote period
    #[prost(message, optional, tag="1")]
    pub aggregate_prevote: ::core::option::Option<AggregateExchangeRatePrevote>,
}
/// QueryAggregatePrevotesRequest is the request type for the Query/AggregatePrevotes RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatePrevotesRequest {
}
/// QueryAggregatePrevotesResponse is response type for the
/// Query/AggregatePrevotes RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregatePrevotesResponse {
    /// aggregate_prevotes defines all oracle aggregate prevotes submitted in the current vote period
    #[prost(message, repeated, tag="1")]
    pub aggregate_prevotes: ::prost::alloc::vec::Vec<AggregateExchangeRatePrevote>,
}
/// QueryAggregateVoteRequest is the request type for the Query/AggregateVote RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVoteRequest {
    /// validator defines the validator address to query for.
    #[prost(string, tag="1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryAggregateVoteResponse is response type for the
/// Query/AggregateVote RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVoteResponse {
    /// aggregate_vote defines oracle aggregate vote submitted by a validator in the current vote period
    #[prost(message, optional, tag="1")]
    pub aggregate_vote: ::core::option::Option<AggregateExchangeRateVote>,
}
/// QueryAggregateVotesRequest is the request type for the Query/AggregateVotes RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVotesRequest {
}
/// QueryAggregateVotesResponse is response type for the
/// Query/AggregateVotes RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVotesResponse {
    /// aggregate_votes defines all oracle aggregate votes submitted in the current vote period
    #[prost(message, repeated, tag="1")]
    pub aggregate_votes: ::prost::alloc::vec::Vec<AggregateExchangeRateVote>,
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
        /// ExchangeRate returns exchange rate of a denom
        pub async fn exchange_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExchangeRateRequest>,
        ) -> Result<tonic::Response<super::QueryExchangeRateResponse>, tonic::Status> {
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
                "/terra.oracle.v1beta1.Query/ExchangeRate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ExchangeRates returns exchange rates of all denoms
        pub async fn exchange_rates(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryExchangeRatesRequest>,
        ) -> Result<tonic::Response<super::QueryExchangeRatesResponse>, tonic::Status> {
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
                "/terra.oracle.v1beta1.Query/ExchangeRates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// TobinTax returns tobin tax of a denom
        pub async fn tobin_tax(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTobinTaxRequest>,
        ) -> Result<tonic::Response<super::QueryTobinTaxResponse>, tonic::Status> {
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
                "/terra.oracle.v1beta1.Query/TobinTax",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// TobinTaxes returns tobin taxes of all denoms
        pub async fn tobin_taxes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTobinTaxesRequest>,
        ) -> Result<tonic::Response<super::QueryTobinTaxesResponse>, tonic::Status> {
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
                "/terra.oracle.v1beta1.Query/TobinTaxes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Actives returns all active denoms
        pub async fn actives(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryActivesRequest>,
        ) -> Result<tonic::Response<super::QueryActivesResponse>, tonic::Status> {
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
                "/terra.oracle.v1beta1.Query/Actives",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// VoteTargets returns all vote target denoms
        pub async fn vote_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVoteTargetsRequest>,
        ) -> Result<tonic::Response<super::QueryVoteTargetsResponse>, tonic::Status> {
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
                "/terra.oracle.v1beta1.Query/VoteTargets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// FeederDelegation returns feeder delegation of a validator
        pub async fn feeder_delegation(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFeederDelegationRequest>,
        ) -> Result<
            tonic::Response<super::QueryFeederDelegationResponse>,
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
                "/terra.oracle.v1beta1.Query/FeederDelegation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// MissCounter returns oracle miss counter of a validator
        pub async fn miss_counter(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryMissCounterRequest>,
        ) -> Result<tonic::Response<super::QueryMissCounterResponse>, tonic::Status> {
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
                "/terra.oracle.v1beta1.Query/MissCounter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AggregatePrevote returns an aggregate prevote of a validator
        pub async fn aggregate_prevote(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregatePrevoteRequest>,
        ) -> Result<
            tonic::Response<super::QueryAggregatePrevoteResponse>,
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
                "/terra.oracle.v1beta1.Query/AggregatePrevote",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AggregatePrevotes returns aggregate prevotes of all validators
        pub async fn aggregate_prevotes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregatePrevotesRequest>,
        ) -> Result<
            tonic::Response<super::QueryAggregatePrevotesResponse>,
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
                "/terra.oracle.v1beta1.Query/AggregatePrevotes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AggregateVote returns an aggregate vote of a validator
        pub async fn aggregate_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregateVoteRequest>,
        ) -> Result<tonic::Response<super::QueryAggregateVoteResponse>, tonic::Status> {
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
                "/terra.oracle.v1beta1.Query/AggregateVote",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AggregateVotes returns aggregate votes of all validators
        pub async fn aggregate_votes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAggregateVotesRequest>,
        ) -> Result<tonic::Response<super::QueryAggregateVotesResponse>, tonic::Status> {
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
                "/terra.oracle.v1beta1.Query/AggregateVotes",
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
                "/terra.oracle.v1beta1.Query/Params",
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
    #[prost(message, repeated, tag="2")]
    pub feeder_delegations: ::prost::alloc::vec::Vec<FeederDelegation>,
    #[prost(message, repeated, tag="3")]
    pub exchange_rates: ::prost::alloc::vec::Vec<ExchangeRateTuple>,
    #[prost(message, repeated, tag="4")]
    pub miss_counters: ::prost::alloc::vec::Vec<MissCounter>,
    #[prost(message, repeated, tag="5")]
    pub aggregate_exchange_rate_prevotes: ::prost::alloc::vec::Vec<AggregateExchangeRatePrevote>,
    #[prost(message, repeated, tag="6")]
    pub aggregate_exchange_rate_votes: ::prost::alloc::vec::Vec<AggregateExchangeRateVote>,
    #[prost(message, repeated, tag="7")]
    pub tobin_taxes: ::prost::alloc::vec::Vec<TobinTax>,
}
/// FeederDelegation is the address for where oracle feeder authority are
/// delegated to. By default this struct is only used at genesis to feed in
/// default feeder addresses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeederDelegation {
    #[prost(string, tag="1")]
    pub feeder_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub validator_address: ::prost::alloc::string::String,
}
/// MissCounter defines an miss counter and validator address pair used in
/// oracle module's genesis state
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissCounter {
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub miss_counter: u64,
}
/// TobinTax defines an denom and tobin_tax pair used in
/// oracle module's genesis state
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TobinTax {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tobin_tax: ::prost::alloc::string::String,
}
