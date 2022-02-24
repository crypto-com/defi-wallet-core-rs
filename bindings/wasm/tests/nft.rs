//! Test suite for nft messages.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use core::time::Duration;
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

    async fn issue(&self, id: &str, name: &str, schema: &str) -> Response {
        self.broadcast(
            get_nft_issue_denom_signed_tx(
                get_tx_info(self.address.to_string()).await,
                self.key.clone(),
                id.to_owned(),
                name.to_owned(),
                schema.to_owned(),
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
        self.broadcast(
            get_nft_mint_signed_tx(
                get_tx_info(self.address.to_string()).await,
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
        self.broadcast(
            get_nft_edit_signed_tx(
                get_tx_info(self.address.to_string()).await,
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
        self.broadcast(
            get_nft_transfer_signed_tx(
                get_tx_info(self.address.to_string()).await,
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
        self.broadcast(
            get_nft_burn_signed_tx(
                get_tx_info(self.address.to_string()).await,
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
    let res = wallet
        .issue(
            "testdenomid",
            "testdenomname",
            r#"
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
                    }"#,
        )
        .await;
    assert_eq!(res.code, tendermint::abci::Code::Ok);

    //
    // issue, SIGNER1
    //
    let res = wallet
        .issue(
            "testdenomid2",
            "testdenomname2",
            r#"
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
                    }"#,
        )
        .await;
    assert_eq!(res.code, tendermint::abci::Code::Ok);
    let _ = Delay::new(Duration::from_millis(3000)).await;

    let res = wallet
        .grpc_web_client
        .supply("testdenomid".to_owned(), SIGNER1.to_owned())
        .await;
    assert!(res.is_ok());
    // console_log!("client.supply: {:?}", res);

    let res = wallet
        .grpc_web_client
        .owner("testdenomid".to_owned(), SIGNER1.to_owned())
        .await;
    assert!(res.is_ok());
    // console_log!("client.owner: {:?}", res);

    let res = wallet
        .grpc_web_client
        .collection("testdenomid2".to_owned())
        .await;
    assert!(res.is_ok());
    // console_log!("client.collection: {:?}", res);

    let res = wallet
        .grpc_web_client
        .denom("testdenomid2".to_owned())
        .await;
    assert!(res.is_ok());
    // console_log!("client.denom: {:?}", res);

    let res = wallet
        .grpc_web_client
        .denom_by_name("testdenomname".to_owned())
        .await;
    assert!(res.is_ok());
    // console_log!("client.denom_by_name: {:?}", res);

    let res = wallet.grpc_web_client.denoms().await;
    assert!(res.is_ok());
    // console_log!("client.denoms: {:?}", res);

    //
    // mint, SIGNER1
    //
    let res = wallet
        .mint("testtokenid", "testdenomid", "", "testuri", "", SIGNER2)
        .await;
    assert_eq!(res.code, tendermint::abci::Code::Ok);
    let _ = Delay::new(Duration::from_millis(3000)).await;

    let res = wallet
        .grpc_web_client
        .nft("testdenomid".to_owned(), "testtokenid".to_owned())
        .await;
    assert!(res.is_ok());
    console_log!("minted nft: {:?}", res);

    //
    // transfer, SIGNER2 -> SIGNER1
    //
    let mut wallet = NftConfig::new()
        .mnemonic(SIGNER2_MNEMONIC)
        .tendermint_rpc(TENDERMINT_RPC_URL)
        .grpc_web_url(GRPC_WEB_URL)
        .finalize();
    let res = wallet.transfer("testtokenid", "testdenomid", SIGNER1).await;
    assert_eq!(res.code, tendermint::abci::Code::Ok);
    let _ = Delay::new(Duration::from_millis(3000)).await;
    let res = wallet
        .grpc_web_client
        .nft("testdenomid".to_owned(), "testtokenid".to_owned())
        .await;
    assert!(res.is_ok());
    console_log!("transferred nft: {:?}", res);

    //
    // edit, SIGNER1
    //
    let mut wallet = NftConfig::new()
        .mnemonic(SIGNER1_MNEMONIC)
        .tendermint_rpc(TENDERMINT_RPC_URL)
        .grpc_web_url(GRPC_WEB_URL)
        .finalize();
    let res = wallet
        .edit("testtokenid", "testdenomid", "newname", "newuri", "")
        .await;
    assert_eq!(res.code, tendermint::abci::Code::Ok);
    let _ = Delay::new(Duration::from_millis(3000)).await;
    let res = wallet
        .grpc_web_client
        .nft("testdenomid".to_owned(), "testtokenid".to_owned())
        .await;
    assert!(res.is_ok());
    console_log!("edited nft: {:?}", res);

    //
    // burn, SIGNER 1
    //
    let res = wallet.burn("testtokenid", "testdenomid").await;
    assert_eq!(res.code, tendermint::abci::Code::Ok);
    let _ = Delay::new(Duration::from_millis(3000)).await;
    let res = wallet
        .grpc_web_client
        .nft("testdenomid".to_owned(), "testtokenid".to_owned())
        .await;
    assert!(res.is_err());
    console_log!("burned nft: {:?}", res);
}
