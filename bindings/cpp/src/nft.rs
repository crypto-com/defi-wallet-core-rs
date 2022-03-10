use super::{ffi, PrivateKey};
use anyhow::{anyhow, Result};
use defi_wallet_core_common::{transaction, Client};
use defi_wallet_core_proto as proto;
use std::fmt;

/// Wrapper of proto::chainmain::nft::v1::Denom
/// It is a rust opaque type, internals can not be seen in C++
pub struct Denom(proto::chainmain::nft::v1::Denom);

impl fmt::Display for Denom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            serde_json::to_string(&self.0).map_err(|_| fmt::Error)?
        )
    }
}

impl Denom {
    pub fn id(&self) -> String {
        self.0.id.clone()
    }
    pub fn name(&self) -> String {
        self.0.name.clone()
    }
    pub fn schema(&self) -> String {
        self.0.schema.clone()
    }
    pub fn creator(&self) -> String {
        self.0.creator.clone()
    }
}

/// Wrapper of proto::chainmain::nft::v1::BaseNft
/// It is a rust opaque type, internals can not be seen in C++
pub struct BaseNft(proto::chainmain::nft::v1::BaseNft);

impl BaseNft {
    pub fn id(&self) -> String {
        self.0.id.clone()
    }
    pub fn name(&self) -> String {
        self.0.name.clone()
    }
    pub fn uri(&self) -> String {
        self.0.uri.clone()
    }
    pub fn data(&self) -> String {
        self.0.data.clone()
    }
    pub fn owner(&self) -> String {
        self.0.owner.clone()
    }
}

impl fmt::Display for BaseNft {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            serde_json::to_string(&self.0).map_err(|_| fmt::Error)?
        )
    }
}

/// Wrapper of proto::chainmain::nft::v1::Owner
/// It is a rust opaque type, internals can not be seen in C++
pub struct Owner(proto::chainmain::nft::v1::Owner);
impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            serde_json::to_string(&self.0).map_err(|_| fmt::Error)?
        )
    }
}

pub struct IdCollection(proto::chainmain::nft::v1::IdCollection);
impl Owner {
    pub fn address(&self) -> String {
        self.0.address.clone()
    }
    pub fn id_collections(&self) -> Vec<IdCollection> {
        self.0
            .id_collections
            .clone()
            .into_iter()
            .map(|v| IdCollection(v))
            .collect()
    }
}
impl IdCollection {
    pub fn denom_id(&self) -> String {
        self.0.denom_id.clone()
    }
    pub fn token_ids(&self) -> Vec<String> {
        self.0.token_ids.clone()
    }
}

/// Wrapper of proto::chainmain::nft::v1::Collection
/// It is a rust opaque type, internals can not be seen in C++
pub struct Collection(proto::chainmain::nft::v1::Collection);
impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            serde_json::to_string(&self.0).map_err(|_| fmt::Error)?
        )
    }
}
impl Collection {
    pub fn denom(&self) -> Result<Box<Denom>> {
        match self.0.denom.clone() {
            Some(d) => Ok(Box::new(Denom(d))),
            None => Err(anyhow!("No Denom")),
        }
    }
    pub fn nfts(&self) -> Vec<BaseNft> {
        self.0
            .nfts
            .clone()
            .into_iter()
            .map(|v| BaseNft(v))
            .collect()
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
    pub fn owner(&self, denom_id: String, owner: String) -> Result<Box<Owner>> {
        let owner = self
            .0
            .owner_blocking(denom_id, owner)?
            .ok_or(anyhow!("No Owner"))?;
        Ok(Box::new(Owner(owner)))
    }

    /// Collection queries the NFTs of the specified denom
    pub fn collection(&self, denom_id: String) -> Result<Box<Collection>> {
        let collection = self
            .0
            .collection_blocking(denom_id)?
            .ok_or(anyhow!("No Collection"))?;
        Ok(Box::new(Collection(collection)))
    }

    /// Denom queries the definition of a given denom
    pub fn denom(&self, denom_id: String) -> Result<Box<Denom>> {
        let denom = self
            .0
            .denom_blocking(denom_id)?
            .ok_or(anyhow!("No denom"))?;
        Ok(Box::new(Denom(denom)))
    }

    /// DenomByName queries the definition of a given denom by name
    pub fn denom_by_name(&self, denom_name: String) -> Result<Box<Denom>> {
        let denom = self
            .0
            .denom_by_name_blocking(denom_name)?
            .ok_or(anyhow!("No denom"))?;
        Ok(Box::new(Denom(denom)))
    }

    /// Denoms queries all the denoms
    pub fn denoms(&self) -> Result<Vec<Denom>> {
        let denoms = self.0.denoms_blocking()?;
        Ok(denoms.into_iter().map(|v| Denom(v)).collect())
    }

    /// NFT queries the NFT for the given denom and token ID
    pub fn nft(&self, denom_id: String, token_id: String) -> Result<Box<BaseNft>> {
        let nft = self
            .0
            .nft_blocking(denom_id, token_id)?
            .ok_or(anyhow!("No Nft"))?;
        Ok(Box::new(BaseNft(nft)))
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
