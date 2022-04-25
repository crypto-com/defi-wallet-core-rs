//! Test suite for nft messages.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use defi_wallet_core_wasm::{
    broadcast_tx, get_nft_burn_signed_tx, get_nft_edit_signed_tx, get_nft_issue_denom_signed_tx,
    get_nft_mint_signed_tx, get_nft_transfer_signed_tx, CoinType, GrpcWebClient, PrivateKey,
    Wallet,
};
use std::{assert, assert_eq};
use test_helper::*;
use wasm_bindgen_test::*;
use wasm_timer::Delay;

use tendermint_rpc::endpoint::broadcast::tx_sync::Response;

use defi_wallet_core_proto as proto;
use proto::chainmain::nft::v1::{BaseNft, Collection, Denom, Owner};

wasm_bindgen_test_configure!(run_in_browser);

pub struct NftConfig {
    wallet: Option<Wallet>,
    tendermint_rpc: Option<String>,
    grpc_web_client: Option<GrpcWebClient>,
}

pub struct NftWallet {
    address: String,
    key: PrivateKey,
    tendermint_rpc: String,
    grpc_web_client: GrpcWebClient,
}

/// Helper for configuring the wallet
impl NftConfig {
    fn new() -> NftConfig {
        NftConfig {
            wallet: None,
            tendermint_rpc: None,
            grpc_web_client: None,
        }
    }
    fn mnemonic(mut self, mnemonic: &str) -> Self {
        self.wallet = Some(Wallet::recover_wallet(mnemonic.to_owned(), None).unwrap());
        self
    }
    fn tendermint_rpc(mut self, url: &str) -> Self {
        self.tendermint_rpc = Some(url.to_owned());
        self
    }
    fn grpc_web_url(mut self, url: &str) -> Self {
        self.grpc_web_client = Some(GrpcWebClient::new(url.to_owned()));
        self
    }

    fn finalize(self) -> NftWallet {
        let wallet = self.wallet.unwrap_or(Wallet::new(None, None).unwrap());
        let address = wallet
            .get_default_address(CoinType::CryptoOrgMainnet)
            .unwrap();
        let key = wallet.get_key("m/44'/394'/0'/0/0".to_owned()).unwrap();
        let tendermint_rpc = self.tendermint_rpc.unwrap_or("".to_owned());
        let grpc_web_client = self
            .grpc_web_client
            .unwrap_or(GrpcWebClient::new("".to_owned()));
        NftWallet {
            address,
            key,
            tendermint_rpc,
            grpc_web_client,
        }
    }
}

/// Helper for exposing the nft features
impl NftWallet {
    async fn broadcast(&self, tx: Vec<u8>) -> Response {
        broadcast_tx(self.tendermint_rpc.to_string(), tx)
            .await
            .unwrap()
            .into_serde::<Response>()
            .unwrap()
    }

    async fn issue(&self, denom: Denom) -> Response {
        wait_for_timeout().await;
        self.broadcast(
            get_nft_issue_denom_signed_tx(
                chainmain_tx_info(&self.address).await,
                self.key.clone(),
                denom.id,
                denom.name,
                denom.schema,
            )
            .unwrap(),
        )
        .await
    }

    async fn mint(
        &self,
        id: &str,
        denom_id: &str,
        name: &str,
        uri: &str,
        data: &str,
        recipient: &str,
    ) -> Response {
        wait_for_timeout().await;
        self.broadcast(
            get_nft_mint_signed_tx(
                chainmain_tx_info(&self.address).await,
                self.key.clone(),
                id.to_owned(),
                denom_id.to_owned(),
                name.to_owned(),
                uri.to_owned(),
                data.to_owned(),
                recipient.to_owned(),
            )
            .unwrap(),
        )
        .await
    }

    async fn edit(&self, id: &str, denom_id: &str, name: &str, uri: &str, data: &str) -> Response {
        wait_for_timeout().await;
        self.broadcast(
            get_nft_edit_signed_tx(
                chainmain_tx_info(&self.address).await,
                self.key.clone(),
                id.to_owned(),
                denom_id.to_owned(),
                name.to_owned(),
                uri.to_owned(),
                data.to_owned(),
            )
            .unwrap(),
        )
        .await
    }

    async fn transfer(&self, id: &str, denom_id: &str, recipient: &str) -> Response {
        wait_for_timeout().await;
        self.broadcast(
            get_nft_transfer_signed_tx(
                chainmain_tx_info(&self.address).await,
                self.key.clone(),
                id.to_owned(),
                denom_id.to_owned(),
                recipient.to_owned(),
            )
            .unwrap(),
        )
        .await
    }

    async fn burn(&self, id: &str, denom_id: &str) -> Response {
        wait_for_timeout().await;
        self.broadcast(
            get_nft_burn_signed_tx(
                chainmain_tx_info(&self.address).await,
                self.key.clone(),
                id.to_owned(),
                denom_id.to_owned(),
            )
            .unwrap(),
        )
        .await
    }
}

#[wasm_bindgen_test]
async fn test_nft() {
    let mut wallet = NftConfig::new()
        .mnemonic(SIGNER1_MNEMONIC)
        .tendermint_rpc(TENDERMINT_RPC_URL)
        .grpc_web_url(GRPC_WEB_URL)
        .finalize();

    //
    // issue, SIGNER1
    //
    let denom1 = Denom {
        id: "testdenomid".to_owned(),
        name: "testdenomname".to_owned(),
        schema: r#"
                    {
                        "title":"Asset Metadata",
                        "type":"object",
                        "properties":{
                            "name":{
                                "type":"string",
                                "description":"testidentity"
                            },
                            "description":{
                                "type":"string",
                                "description":"testdescription"
                            },
                            "image":{
                                "type":"string",
                                "description":"testdescription"
                            }
                        }
                    }"#
        .to_owned(),
        creator: SIGNER1.to_owned(), // not used
    };
    let res = wallet.issue(denom1.clone()).await;
    assert_eq!(res.code, tendermint::abci::Code::Ok);

    //
    // issue, SIGNER1
    //
    let denom2 = Denom {
        id: "testdenomid2".to_owned(),
        name: "testdenomname2".to_owned(),
        schema: r#"
                    {
                        "title":"Asset Metadata",
                        "type":"object",
                        "properties":{
                            "name":{
                                "type":"string",
                                "description":"testidentity"
                            },
                            "description":{
                                "type":"string",
                                "description":"testdescription"
                            },
                            "image":{
                                "type":"string",
                                "description":"testdescription"
                            }
                        }
                    }"#
        .to_owned(),
        creator: SIGNER1.to_owned(), // not used
    };
    let res = wallet.issue(denom2.clone()).await;
    assert_eq!(res.code, tendermint::abci::Code::Ok);
    Delay::new(DEFAULT_WAITING_DURATION).await.unwrap();

    let res = wallet
        .grpc_web_client
        .supply(denom1.clone().id, SIGNER1.to_owned())
        .await;
    assert!(res.is_ok());
    let supply = res.unwrap().into_serde::<u64>().unwrap();
    assert_eq!(supply, 0);

    let res = wallet
        .grpc_web_client
        .owner(denom1.clone().id, SIGNER1.to_owned())
        .await;
    assert!(res.is_ok());
    let owner = res.unwrap().into_serde::<Owner>().unwrap();
    assert_eq!(owner.address, SIGNER1.to_owned());
    assert_eq!(owner.id_collections, vec![]);

    let res = wallet.grpc_web_client.collection(denom2.clone().id).await;
    assert!(res.is_ok());
    let collection = res.unwrap().into_serde::<Collection>().unwrap();
    assert_eq!(collection.denom, Some(denom2.clone()));
    assert_eq!(collection.nfts.len(), 0);

    let res = wallet.grpc_web_client.denom(denom2.clone().id).await;
    assert!(res.is_ok());
    let denom = res.unwrap().into_serde::<Denom>().unwrap();
    assert_eq!(denom, denom2.clone());

    let res = wallet
        .grpc_web_client
        .denom_by_name(denom1.clone().name)
        .await;
    assert!(res.is_ok());
    let denom = res.unwrap().into_serde::<Denom>().unwrap();
    assert_eq!(denom, denom1.clone());

    let res = wallet.grpc_web_client.denoms().await;
    assert!(res.is_ok());
    let denoms = res.unwrap().into_serde::<Vec<Denom>>().unwrap();
    assert_eq!(denoms.len(), 2);
    assert_eq!(denoms[0], denom1.clone());
    assert_eq!(denoms[1], denom2);

    //
    // mint, SIGNER1
    //
    let tokenid = "testtokenid";
    let res = wallet
        .mint(tokenid, &denom1.clone().id, "", "testuri", "", SIGNER2)
        .await;
    assert_eq!(res.code, tendermint::abci::Code::Ok);
    Delay::new(DEFAULT_WAITING_DURATION).await.unwrap();

    // Check nft after minting
    let res = wallet
        .grpc_web_client
        .nft(denom1.clone().id, tokenid.to_owned())
        .await;
    assert!(res.is_ok());
    let nft = res.unwrap().into_serde::<BaseNft>().unwrap();
    assert_eq!(nft.owner, SIGNER2.to_owned());

    // Check collection after minting
    let res = wallet.grpc_web_client.collection(denom1.clone().id).await;
    assert!(res.is_ok());
    let collection = res.unwrap().into_serde::<Collection>().unwrap();
    assert_eq!(collection.denom, Some(denom1.clone()));
    assert_eq!(collection.nfts.len(), 1);

    // check owner before transferring
    let res = wallet
        .grpc_web_client
        .owner(denom1.clone().id, SIGNER2.to_owned())
        .await;
    assert!(res.is_ok());
    let owner = res.unwrap().into_serde::<Owner>().unwrap();
    assert_eq!(owner.address, SIGNER2.to_owned());
    assert_eq!(owner.id_collections.len(), 1);
    assert_eq!(owner.id_collections[0].denom_id, denom1.clone().id);
    assert_eq!(owner.id_collections[0].token_ids.len(), 1);
    assert_eq!(owner.id_collections[0].token_ids[0], tokenid.to_owned());

    //
    // transfer, SIGNER2 -> SIGNER1
    //
    let mut wallet = NftConfig::new()
        .mnemonic(SIGNER2_MNEMONIC)
        .tendermint_rpc(TENDERMINT_RPC_URL)
        .grpc_web_url(GRPC_WEB_URL)
        .finalize();
    let res = wallet.transfer(tokenid, &denom1.clone().id, SIGNER1).await;
    assert_eq!(res.code, tendermint::abci::Code::Ok);
    Delay::new(DEFAULT_WAITING_DURATION).await.unwrap();
    let res = wallet
        .grpc_web_client
        .nft(denom1.clone().id, tokenid.to_owned())
        .await;
    assert!(res.is_ok());
    let nft = res.unwrap().into_serde::<BaseNft>().unwrap();
    assert_eq!(nft.owner, SIGNER1.to_owned());
    assert_eq!(owner.id_collections.len(), 1);
    assert_eq!(owner.id_collections[0].denom_id, denom1.clone().id);
    assert_eq!(owner.id_collections[0].token_ids.len(), 1);
    assert_eq!(owner.id_collections[0].token_ids[0], tokenid.to_owned());

    // check owner after transferring
    let res = wallet
        .grpc_web_client
        .owner(denom1.clone().id, SIGNER2.to_owned())
        .await;
    assert!(res.is_ok());
    let owner = res.unwrap().into_serde::<Owner>().unwrap();
    assert_eq!(owner.address, SIGNER2.to_owned());
    assert_eq!(owner.id_collections, vec![]);

    //
    // edit, SIGNER1
    //
    let mut wallet = NftConfig::new()
        .mnemonic(SIGNER1_MNEMONIC)
        .tendermint_rpc(TENDERMINT_RPC_URL)
        .grpc_web_url(GRPC_WEB_URL)
        .finalize();
    let res = wallet
        .edit(tokenid, &denom1.clone().id, "newname", "newuri", "newdata")
        .await;
    assert_eq!(res.code, tendermint::abci::Code::Ok);
    Delay::new(DEFAULT_WAITING_DURATION).await.unwrap();
    let res = wallet
        .grpc_web_client
        .nft(denom1.clone().id, tokenid.to_owned())
        .await;
    assert!(res.is_ok());
    let nft = res.unwrap().into_serde::<BaseNft>().unwrap();
    assert_eq!(nft.name, "newname".to_owned());
    assert_eq!(nft.uri, "newuri".to_owned());
    assert_eq!(nft.data, "newdata".to_owned());

    // check supply before burning
    let res = wallet
        .grpc_web_client
        .supply(denom1.clone().id, SIGNER1.to_owned())
        .await;
    assert!(res.is_ok());
    let supply = res.unwrap().into_serde::<u64>().unwrap();
    assert_eq!(supply, 1);

    //
    // burn, SIGNER 1
    //
    let res = wallet.burn(tokenid, &denom1.clone().id).await;
    assert_eq!(res.code, tendermint::abci::Code::Ok);
    Delay::new(DEFAULT_WAITING_DURATION).await.unwrap();
    let res = wallet
        .grpc_web_client
        .nft(denom1.clone().id, tokenid.to_owned())
        .await;
    assert!(res.is_err());

    // check supply after burning
    let res = wallet
        .grpc_web_client
        .supply(denom1.clone().id, SIGNER1.to_owned())
        .await;
    assert!(res.is_ok());
    let supply = res.unwrap().into_serde::<u64>().unwrap();
    assert_eq!(supply, 0);

    // Check collection after burning
    let res = wallet.grpc_web_client.collection(denom1.clone().id).await;
    assert!(res.is_ok());
    let collection = res.unwrap().into_serde::<Collection>().unwrap();
    assert_eq!(collection.denom, Some(denom1));
    assert_eq!(collection.nfts.len(), 0);
}
