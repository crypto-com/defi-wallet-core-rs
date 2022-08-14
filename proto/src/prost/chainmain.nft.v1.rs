/// MsgIssueDenom defines an SDK message for creating a new denom.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIssueDenom {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub schema: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub sender: ::prost::alloc::string::String,
}
/// MsgIssueDenomResponse defines the Msg/IssueDenom response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIssueDenomResponse {
}
/// MsgTransferNFT defines an SDK message for transferring an NFT to recipient.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferNft {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub recipient: ::prost::alloc::string::String,
}
/// MsgTransferNFTResponse defines the Msg/TransferNFT response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferNftResponse {
}
/// MsgEditNFT defines an SDK message for editing a nft.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEditNft {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub sender: ::prost::alloc::string::String,
}
/// MsgEditNFTResponse defines the Msg/EditNFT response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEditNftResponse {
}
/// MsgMintNFT defines an SDK message for creating a new NFT.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintNft {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub recipient: ::prost::alloc::string::String,
}
/// MsgMintNFTResponse defines the Msg/MintNFT response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintNftResponse {
}
/// MsgBurnNFT defines an SDK message for burning a NFT.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnNft {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub sender: ::prost::alloc::string::String,
}
/// MsgBurnNFTResponse defines the Msg/BurnNFT response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnNftResponse {
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the NFT Msg service.
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
        /// IssueDenom defines a method for issue a denom.
        pub async fn issue_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgIssueDenom>,
        ) -> Result<tonic::Response<super::MsgIssueDenomResponse>, tonic::Status> {
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
                "/chainmain.nft.v1.Msg/IssueDenom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// MintNFT defines a method for mint a new nft
        pub async fn mint_nft(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMintNft>,
        ) -> Result<tonic::Response<super::MsgMintNftResponse>, tonic::Status> {
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
                "/chainmain.nft.v1.Msg/MintNFT",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// EditNFT defines a method for editing a nft.
        pub async fn edit_nft(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgEditNft>,
        ) -> Result<tonic::Response<super::MsgEditNftResponse>, tonic::Status> {
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
                "/chainmain.nft.v1.Msg/EditNFT",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// TransferNFT defines a method for transferring a nft.
        pub async fn transfer_nft(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgTransferNft>,
        ) -> Result<tonic::Response<super::MsgTransferNftResponse>, tonic::Status> {
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
                "/chainmain.nft.v1.Msg/TransferNFT",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// BurnNFT defines a method for burning a nft.
        pub async fn burn_nft(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBurnNft>,
        ) -> Result<tonic::Response<super::MsgBurnNftResponse>, tonic::Status> {
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
                "/chainmain.nft.v1.Msg/BurnNFT",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// BaseNFT defines a non-fungible token
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseNft {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub owner: ::prost::alloc::string::String,
}
/// Denom defines a type of NFT
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Denom {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub schema: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub creator: ::prost::alloc::string::String,
}
/// IDCollection defines a type of collection with specified ID
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdCollection {
    #[prost(string, tag="1")]
    pub denom_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Owner defines a type of owner
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Owner {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub id_collections: ::prost::alloc::vec::Vec<IdCollection>,
}
/// Collection defines a type of collection
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collection {
    #[prost(message, optional, tag="1")]
    pub denom: ::core::option::Option<Denom>,
    #[prost(message, repeated, tag="2")]
    pub nfts: ::prost::alloc::vec::Vec<BaseNft>,
}
/// QuerySupplyRequest is the request type for the Query/HTLC RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyRequest {
    #[prost(string, tag="1")]
    pub denom_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
}
/// QuerySupplyResponse is the response type for the Query/Supply RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyResponse {
    #[prost(uint64, tag="1")]
    pub amount: u64,
}
/// QueryOwnerRequest is the request type for the Query/Owner RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerRequest {
    #[prost(string, tag="1")]
    pub denom_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="3")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryOwnerResponse is the response type for the Query/Owner RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerResponse {
    #[prost(message, optional, tag="1")]
    pub owner: ::core::option::Option<Owner>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryCollectionRequest is the request type for the Query/Collection RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCollectionRequest {
    #[prost(string, tag="1")]
    pub denom_id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryCollectionResponse is the response type for the Query/Collection RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCollectionResponse {
    #[prost(message, optional, tag="1")]
    pub collection: ::core::option::Option<Collection>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryDenomRequest is the request type for the Query/Denom RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomRequest {
    #[prost(string, tag="1")]
    pub denom_id: ::prost::alloc::string::String,
}
/// QueryDenomResponse is the response type for the Query/Denom RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomResponse {
    #[prost(message, optional, tag="1")]
    pub denom: ::core::option::Option<Denom>,
}
/// QueryDenomByNameRequest is the request type for the Query/DenomByName RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomByNameRequest {
    #[prost(string, tag="1")]
    pub denom_name: ::prost::alloc::string::String,
}
/// QueryDenomByNameResponse is the response type for the Query/DenomByName RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomByNameResponse {
    #[prost(message, optional, tag="1")]
    pub denom: ::core::option::Option<Denom>,
}
/// QueryDenomsRequest is the request type for the Query/Denoms RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryDenomsResponse is the response type for the Query/Denoms RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomsResponse {
    #[prost(message, repeated, tag="1")]
    pub denoms: ::prost::alloc::vec::Vec<Denom>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryNFTRequest is the request type for the Query/NFT RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNftRequest {
    #[prost(string, tag="1")]
    pub denom_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_id: ::prost::alloc::string::String,
}
/// QueryNFTResponse is the response type for the Query/NFT RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNftResponse {
    #[prost(message, optional, tag="1")]
    pub nft: ::core::option::Option<BaseNft>,
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query defines the gRPC querier service for NFT module
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
        /// Supply queries the total supply of a given denom or owner
        pub async fn supply(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySupplyRequest>,
        ) -> Result<tonic::Response<super::QuerySupplyResponse>, tonic::Status> {
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
                "/chainmain.nft.v1.Query/Supply",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Owner queries the NFTs of the specified owner
        pub async fn owner(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOwnerRequest>,
        ) -> Result<tonic::Response<super::QueryOwnerResponse>, tonic::Status> {
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
                "/chainmain.nft.v1.Query/Owner",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Collection queries the NFTs of the specified denom
        pub async fn collection(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCollectionRequest>,
        ) -> Result<tonic::Response<super::QueryCollectionResponse>, tonic::Status> {
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
                "/chainmain.nft.v1.Query/Collection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Denom queries the definition of a given denom
        pub async fn denom(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomRequest>,
        ) -> Result<tonic::Response<super::QueryDenomResponse>, tonic::Status> {
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
                "/chainmain.nft.v1.Query/Denom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DenomByName queries the definition of a given denom by name
        pub async fn denom_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomByNameRequest>,
        ) -> Result<tonic::Response<super::QueryDenomByNameResponse>, tonic::Status> {
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
                "/chainmain.nft.v1.Query/DenomByName",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Denoms queries all the denoms
        pub async fn denoms(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomsRequest>,
        ) -> Result<tonic::Response<super::QueryDenomsResponse>, tonic::Status> {
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
                "/chainmain.nft.v1.Query/Denoms",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// NFT queries the NFT for the given denom and token ID
        pub async fn nft(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryNftRequest>,
        ) -> Result<tonic::Response<super::QueryNftResponse>, tonic::Status> {
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
                "/chainmain.nft.v1.Query/NFT",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState defines the NFT module's genesis state
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag="1")]
    pub collections: ::prost::alloc::vec::Vec<Collection>,
}
