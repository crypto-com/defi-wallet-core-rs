use super::{ffi, PrivateKey};
use anyhow::{anyhow, Result};
use defi_wallet_core_common::{transaction, Client};
use defi_wallet_core_proto as proto;
use proto::chainmain::nft::v1::{BaseNft, Collection, Denom, IdCollection, Owner};

/// Wrapper of proto::chainmain::nft::v1::Denom
///
/// For now, types used as extern Rust types are required to be defined by the same crate that
/// contains the bridge using them. This restriction may be lifted in the future.
/// Check https://cxx.rs/extern-rust.html
pub struct DenomRaw {
    pub id: String,
    pub name: String,
    pub schema: String,
    pub creator: String,
}

impl From<Denom> for DenomRaw {
    fn from(d: Denom) -> DenomRaw {
        DenomRaw {
            id: d.id,
            name: d.name,
            schema: d.schema,
            creator: d.creator,
        }
    }
}

/// Wrapper of proto::chainmain::nft::v1::BaseNft
///
/// For now, types used as extern Rust types are required to be defined by the same crate that
/// contains the bridge using them. This restriction may be lifted in the future.
/// Check https://cxx.rs/extern-rust.html
pub struct BaseNftRaw {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub data: String,
    pub owner: String,
}

impl From<BaseNft> for BaseNftRaw {
    fn from(d: BaseNft) -> BaseNftRaw {
        BaseNftRaw {
            id: d.id,
            name: d.name,
            uri: d.uri,
            data: d.data,
            owner: d.owner,
        }
    }
}

/// Wrapper of proto::chainmain::nft::v1::Owner
///
/// For now, types used as extern Rust types are required to be defined by the same crate that
/// contains the bridge using them. This restriction may be lifted in the future.
/// Check https://cxx.rs/extern-rust.html
pub struct OwnerRaw {
    pub address: String,
    pub id_collections: Vec<IdCollection>,
}

impl From<Owner> for OwnerRaw {
    fn from(d: Owner) -> OwnerRaw {
        OwnerRaw {
            address: d.address,
            id_collections: d.id_collections,
        }
    }
}

/// Wrapper of proto::chainmain::nft::v1::Collection
///
/// For now, types used as extern Rust types are required to be defined by the same crate that
/// contains the bridge using them. This restriction may be lifted in the future.
/// Check https://cxx.rs/extern-rust.html
pub struct CollectionRaw {
    pub denom: Option<Denom>,
    pub nfts: Vec<BaseNft>,
}

impl From<Collection> for CollectionRaw {
    fn from(d: Collection) -> CollectionRaw {
        CollectionRaw {
            denom: d.denom,
            nfts: d.nfts,
        }
    }
}
/// Wrapper of `Client`
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
    pub fn owner(&self, denom_id: String, owner: String) -> Result<Box<OwnerRaw>> {
        let owner = self
            .0
            .owner_blocking(denom_id, owner)?
            .ok_or_else(|| anyhow!("No Owner"))?;
        Ok(Box::new(owner.into()))
    }

    /// Collection queries the NFTs of the specified denom
    pub fn collection(&self, denom_id: String) -> Result<Box<CollectionRaw>> {
        let collection = self
            .0
            .collection_blocking(denom_id)?
            .ok_or_else(|| anyhow!("No Collection"))?;
        Ok(Box::new(collection.into()))
    }

    /// Denom queries the definition of a given denom
    pub fn denom(&self, denom_id: String) -> Result<Box<DenomRaw>> {
        let denom = self
            .0
            .denom_blocking(denom_id)?
            .ok_or_else(|| anyhow!("No denom"))?;
        Ok(Box::new(denom.into()))
    }

    /// DenomByName queries the definition of a given denom by name
    pub fn denom_by_name(&self, denom_name: String) -> Result<Box<DenomRaw>> {
        let denom = self
            .0
            .denom_by_name_blocking(denom_name)?
            .ok_or_else(|| anyhow!("No denom"))?;
        Ok(Box::new(denom.into()))
    }

    /// Denoms queries all the denoms
    pub fn denoms(&self) -> Result<Vec<DenomRaw>> {
        let denoms = self.0.denoms_blocking()?;
        Ok(denoms.into_iter().map(|v| v.into()).collect())
    }

    /// NFT queries the NFT for the given denom and token ID
    pub fn nft(&self, denom_id: String, token_id: String) -> Result<Box<BaseNftRaw>> {
        let nft = self
            .0
            .nft_blocking(denom_id, token_id)?
            .ok_or_else(|| anyhow!("No Nft"))?;
        Ok(Box::new(nft.into()))
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
