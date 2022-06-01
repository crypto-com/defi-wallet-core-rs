use crate::PrivateKey;
use anyhow::{anyhow, Result};
use defi_wallet_core_common::{transaction, Client};
use defi_wallet_core_proto as proto;
use std::fmt;

#[cxx::bridge(namespace = "org::defi_wallet_core")]
#[allow(clippy::too_many_arguments)]
mod ffi {

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Denom {
        pub id: String,
        pub name: String,
        pub schema: String,
        pub creator: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BaseNft {
        pub id: String,
        pub name: String,
        pub uri: String,
        pub data: String,
        pub owner: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct IdCollection {
        pub denom_id: String,
        pub token_ids: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Owner {
        pub address: String,
        pub id_collections: Vec<IdCollection>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Collection {
        // could be changed to Option if https://github.com/dtolnay/cxx/issues/87 fixed
        pub denom_option: bool,
        pub denom_value: Denom,
        pub nfts: Vec<BaseNft>,
    }

    extern "C++" {
        include!("defi-wallet-core-cpp/src/lib.rs.h");
        type CosmosSDKTxInfoRaw = crate::ffi::CosmosSDKTxInfoRaw;
        type PrivateKey = crate::PrivateKey;
    }

    unsafe extern "C++" {
        include!("defi-wallet-core-cpp/include/nft.h");
        // could be changed to Rust share type if https://github.com/dtolnay/cxx/issues/716 is
        // fixed
        type Pagination;
        // could be changed to Option if https://github.com/dtolnay/cxx/issues/87 is fixed
        fn get_enable(&self) -> bool;
        fn get_key(&self) -> Vec<u8>;
        fn get_offset(&self) -> u64;
        fn get_limit(&self) -> u64;
        fn get_count_total(&self) -> bool;
        fn get_reverse(&self) -> bool;

    }

    extern "Rust" {
        /// creates the signed transaction
        /// for `MsgIssueDenom` from the Chainmain nft module
        fn get_nft_issue_denom_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            id: String,
            name: String,
            schema: String,
        ) -> Result<Vec<u8>>;
        /// creates the signed transaction
        /// for `MsgMintNft` from the Chainmain nft module
        fn get_nft_mint_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            id: String,
            denom_id: String,
            name: String,
            uri: String,
            data: String,
            recipient: String,
        ) -> Result<Vec<u8>>;
        /// creates the signed transaction
        /// for `MsgEditNft` from the Chainmain nft module
        fn get_nft_edit_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            id: String,
            denom_id: String,
            name: String,
            uri: String,
            data: String,
        ) -> Result<Vec<u8>>;
        /// creates the signed transaction
        /// for `MsgTransferNft` from the Chainmain nft module
        fn get_nft_transfer_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            id: String,
            denom_id: String,
            recipient: String,
        ) -> Result<Vec<u8>>;
        /// creates the signed transaction
        /// for `MsgBurnNft` from the Chainmain nft module
        fn get_nft_burn_signed_tx(
            tx_info: CosmosSDKTxInfoRaw,
            private_key: &PrivateKey,
            id: String,
            denom_id: String,
        ) -> Result<Vec<u8>>;
    }

    extern "Rust" {
        type GrpcClient;
        /// Create a new grpc client
        fn new_grpc_client(grpc_url: String) -> Result<Box<GrpcClient>>;
        /// Supply queries the total supply of a given denom or owner
        fn supply(self: &GrpcClient, denom_id: String, owner: String) -> Result<u64>;
        /// Owner queries the NFTs of the specified owner
        fn owner(
            self: &GrpcClient,
            denom_id: String,
            owner: String,
            pagination: &Pagination,
        ) -> Result<Owner>;
        /// Collection queries the NFTs of the specified denom
        fn collection(
            self: &GrpcClient,
            denom_id: String,
            pagination: &Pagination,
        ) -> Result<Collection>;
        /// Denom queries the definition of a given denom
        fn denom(self: &GrpcClient, denom_id: String) -> Result<Denom>;
        /// DenomByName queries the definition of a given denom by name
        fn denom_by_name(self: &GrpcClient, denom_name: String) -> Result<Denom>;
        /// Denoms queries all the denoms
        fn denoms(self: &GrpcClient, pagination: &Pagination) -> Result<Vec<Denom>>;
        /// NFT queries the NFT for the given denom and token ID
        fn nft(self: &GrpcClient, denom_id: String, token_id: String) -> Result<BaseNft>;

        pub fn to_string(self: &Owner) -> String;
        pub fn to_string(self: &Collection) -> String;
        pub fn to_string(self: &Denom) -> String;
        pub fn to_string(self: &BaseNft) -> String;

    }
}

impl From<proto::chainmain::nft::v1::Denom> for ffi::Denom {
    fn from(d: proto::chainmain::nft::v1::Denom) -> ffi::Denom {
        ffi::Denom {
            id: d.id,
            name: d.name,
            schema: d.schema,
            creator: d.creator,
        }
    }
}

impl fmt::Display for ffi::Denom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            serde_json::to_string(&self).map_err(|_| fmt::Error)?
        )
    }
}

impl From<proto::chainmain::nft::v1::BaseNft> for ffi::BaseNft {
    fn from(d: proto::chainmain::nft::v1::BaseNft) -> ffi::BaseNft {
        ffi::BaseNft {
            id: d.id,
            name: d.name,
            uri: d.uri,
            data: d.data,
            owner: d.owner,
        }
    }
}

impl fmt::Display for ffi::BaseNft {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            serde_json::to_string(&self).map_err(|_| fmt::Error)?
        )
    }
}

impl From<proto::chainmain::nft::v1::IdCollection> for ffi::IdCollection {
    fn from(d: proto::chainmain::nft::v1::IdCollection) -> ffi::IdCollection {
        ffi::IdCollection {
            denom_id: d.denom_id,
            token_ids: d.token_ids,
        }
    }
}

impl From<proto::chainmain::nft::v1::Owner> for ffi::Owner {
    fn from(d: proto::chainmain::nft::v1::Owner) -> ffi::Owner {
        ffi::Owner {
            address: d.address,
            id_collections: d.id_collections.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl fmt::Display for ffi::Owner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            serde_json::to_string(&self).map_err(|_| fmt::Error)?
        )
    }
}

impl From<proto::chainmain::nft::v1::Collection> for ffi::Collection {
    fn from(d: proto::chainmain::nft::v1::Collection) -> ffi::Collection {
        match d.denom {
            Some(denom) => ffi::Collection {
                denom_option: true,
                denom_value: denom.into(),
                nfts: d.nfts.into_iter().map(|v| v.into()).collect(),
            },
            None => ffi::Collection {
                denom_option: false,
                denom_value: ffi::Denom {
                    id: "".to_owned(),
                    name: "".to_owned(),
                    schema: "".to_owned(),
                    creator: "".to_owned(),
                },
                nfts: d.nfts.into_iter().map(|v| v.into()).collect(),
            },
        }
    }
}

impl fmt::Display for ffi::Collection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            serde_json::to_string(&self).map_err(|_| fmt::Error)?
        )
    }
}

impl From<&ffi::Pagination>
    for Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>
{
    fn from(
        d: &ffi::Pagination,
    ) -> Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest> {
        if d.get_enable() {
            Some(
                cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest {
                    key: d.get_key(),
                    offset: d.get_offset(),
                    limit: d.get_limit(),
                    count_total: d.get_count_total(),
                    reverse: d.get_reverse(),
                },
            )
        } else {
            None
        }
    }
}

/// Wrapper of `Client`
/// It is a rust opaque type, internals can not be seen in C++
pub struct GrpcClient(Client);

/// Create a new grpc client
// It can only be defined outside the `impl GrpcClient`, otherwise the mod ffi can not find it
pub fn new_grpc_client(grpc_url: String) -> Result<Box<GrpcClient>> {
    let client = Client::new_blocking(grpc_url)?;
    Ok(Box::new(GrpcClient(client)))
}

impl GrpcClient {
    /// Supply queries the total supply of a given denom or owner
    pub fn supply(&self, denom_id: String, owner: String) -> Result<u64> {
        let supply = self.0.supply_blocking(denom_id, owner)?;
        Ok(supply)
    }

    /// Owner queries the NFTs of the specified owner
    pub fn owner(
        &self,
        denom_id: String,
        owner: String,
        pagination: &ffi::Pagination,
    ) -> Result<ffi::Owner> {
        let owner = self
            .0
            .owner_blocking(denom_id, owner, pagination.into())?
            .ok_or_else(|| anyhow!("No Owner"))?;
        Ok(owner.into())
    }

    /// Collection queries the NFTs of the specified denom
    pub fn collection(
        &self,
        denom_id: String,
        pagination: &ffi::Pagination,
    ) -> Result<ffi::Collection> {
        let collection = self
            .0
            .collection_blocking(denom_id, pagination.into())?
            .ok_or_else(|| anyhow!("No Collection"))?;
        Ok(collection.into())
    }

    /// Denom queries the definition of a given denom
    pub fn denom(&self, denom_id: String) -> Result<ffi::Denom> {
        let denom = self
            .0
            .denom_blocking(denom_id)?
            .ok_or_else(|| anyhow!("No denom"))?;
        Ok(denom.into())
    }

    /// DenomByName queries the definition of a given denom by name
    pub fn denom_by_name(&self, denom_name: String) -> Result<ffi::Denom> {
        let denom = self
            .0
            .denom_by_name_blocking(denom_name)?
            .ok_or_else(|| anyhow!("No denom"))?;
        Ok(denom.into())
    }

    /// Denoms queries all the denoms
    pub fn denoms(&self, pagination: &ffi::Pagination) -> Result<Vec<ffi::Denom>> {
        let denoms = self.0.denoms_blocking(pagination.into())?;
        Ok(denoms.into_iter().map(|v| v.into()).collect())
    }

    /// NFT queries the NFT for the given denom and token ID
    pub fn nft(&self, denom_id: String, token_id: String) -> Result<ffi::BaseNft> {
        let nft = self
            .0
            .nft_blocking(denom_id, token_id)?
            .ok_or_else(|| anyhow!("No Nft"))?;
        Ok(nft.into())
    }
}

/// creates the signed transaction
/// for `MsgIssueDenom` from the Chainmain nft module
pub fn get_nft_issue_denom_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    id: String,
    name: String,
    schema: String,
) -> Result<Vec<u8>> {
    let ret = transaction::nft::get_nft_issue_denom_signed_tx(
        tx_info.into(),
        private_key.key.clone(),
        id,
        name,
        schema,
    )?;
    Ok(ret)
}

/// creates the signed transaction
/// for `MsgMintNft` from the Chainmain nft module
#[allow(clippy::too_many_arguments)]
pub fn get_nft_mint_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    id: String,
    denom_id: String,
    name: String,
    uri: String,
    data: String,
    recipient: String,
) -> Result<Vec<u8>> {
    let ret = transaction::nft::get_nft_mint_signed_tx(
        tx_info.into(),
        private_key.key.clone(),
        id,
        denom_id,
        name,
        uri,
        data,
        recipient,
    )?;
    Ok(ret)
}

/// creates the signed transaction
/// for `MsgEditNft` from the Chainmain nft module
pub fn get_nft_edit_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    id: String,
    denom_id: String,
    name: String,
    uri: String,
    data: String,
) -> Result<Vec<u8>> {
    let ret = transaction::nft::get_nft_edit_signed_tx(
        tx_info.into(),
        private_key.key.clone(),
        id,
        denom_id,
        name,
        uri,
        data,
    )?;

    Ok(ret)
}

/// creates the signed transaction
/// for `MsgTransferNft` from the Chainmain nft module
pub fn get_nft_transfer_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    id: String,
    denom_id: String,
    recipient: String,
) -> Result<Vec<u8>> {
    let ret = transaction::nft::get_nft_transfer_signed_tx(
        tx_info.into(),
        private_key.key.clone(),
        id,
        denom_id,
        recipient,
    )?;

    Ok(ret)
}

/// creates the signed transaction
/// for `MsgBurnNft` from the Chainmain nft module
pub fn get_nft_burn_signed_tx(
    tx_info: ffi::CosmosSDKTxInfoRaw,
    private_key: &PrivateKey,
    id: String,
    denom_id: String,
) -> Result<Vec<u8>> {
    let ret = transaction::nft::get_nft_burn_signed_tx(
        tx_info.into(),
        private_key.key.clone(),
        id,
        denom_id,
    )?;

    Ok(ret)
}
