/// ComputeTaxRequest is the request type for the Service.ComputeTax
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeTaxRequest {
    /// tx is the transaction to simulate.
    /// Deprecated. Send raw tx bytes instead.
    #[deprecated]
    #[prost(message, optional, tag="1")]
    pub tx: ::core::option::Option<super::super::super::cosmos::tx::v1beta1::Tx>,
    /// tx_bytes is the raw transaction.
    #[prost(bytes="vec", tag="2")]
    pub tx_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// ComputeTaxResponse is the response type for the Service.ComputeTax
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeTaxResponse {
    /// amount is the amount of coins to be paid as a fee
    #[prost(message, repeated, tag="1")]
    pub tax_amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Generated client implementations.
pub mod service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service defines a gRPC service for interacting with transactions.
    #[derive(Debug, Clone)]
    pub struct ServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServiceClient<tonic::transport::Channel> {
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
    impl<T> ServiceClient<T>
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
        ) -> ServiceClient<InterceptedService<T, F>>
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
            ServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// EstimateFee simulates executing a transaction for estimating gas usage.
        pub async fn compute_tax(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeTaxRequest>,
        ) -> Result<tonic::Response<super::ComputeTaxResponse>, tonic::Status> {
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
                "/terra.tx.v1beta1.Service/ComputeTax",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
