//! Test helper constants and functions.

// Disable unused code warnings in this file. Since the helper constants and
// functions are not all used for each test files.
#![allow(dead_code)]

use core::time::Duration;
use defi_wallet_core_common::{Network, RawRpcAccountResponse, RawRpcAccountStatus, RawRpcBalance};
use defi_wallet_core_wasm::{
    CosmosClient, CosmosClientConfig, CosmosSDKTxInfoRaw, PrivateKey, Wallet,
};
use wasm_bindgen_futures::JsFuture;
use wasm_timer::Delay;

pub(crate) const CHAIN_ID: &str = "chainmain-1";
pub(crate) const CHAINMAIN_DENOM: &str = "basecro";
pub(crate) const CRONOS_DENOM: &str = "basetcro";
pub(crate) const CHAINMAIN_API_URL: &str = "http://127.0.0.1:26804";
pub(crate) const CRONOS_API_URL: &str = "http://127.0.0.1:26654";
pub(crate) const TENDERMINT_RPC_URL: &str = "http://127.0.0.1:26807";
pub(crate) const GRPC_WEB_URL: &str = "http://127.0.0.1:26808";

pub(crate) const COMMUNITY: &str = "cro1qj4u2y23hx7plrztswrel2hgf8mh2m22k80fet";
pub(crate) const DELEGATOR1: &str = "cro1ykec6vralvrh5vcvpf7w7u02gj728u4wp738kz";
pub(crate) const DELEGATOR2: &str = "cro1tmfhgwp62uhz5y5hqcyl8jkjq22l2cles2lum8";
pub(crate) const SIGNER1: &str = "cro1u08u5dvtnpmlpdq333uj9tcj75yceggszxpnsy";
pub(crate) const SIGNER2: &str = "cro1apdh4yc2lnpephevc6lmpvkyv6s5cjh652n6e4";
pub(crate) const VALIDATOR1: &str = "crocncl1pk9eajj4zuzpptnadwz6tzfgcpchqvpkvql0a9";
pub(crate) const VALIDATOR2: &str = "crocncl1dkwjtmkueye3fqwzyv2jrdn7fspd2jkm37nunc";

pub(crate) const CRONOS_COMMUNITY: &str = "crc1x7x9pkfxf33l87ftspk5aetwnkr0lvlv3346cd";
pub(crate) const CRONOS_DELEGATOR1: &str = "crc1zgxux2e3m8aexy9husuglyez2dcdmyw6nlkv79";
pub(crate) const CRONOS_DELEGATOR2: &str = "crc1nmptsrsac2hejmtc6kkz70zlc745ls5mm3pl7q";
pub(crate) const CRONOS_SIGNER1: &str = "crc16z0herz998946wr659lr84c8c556da55dc34hh";
pub(crate) const CRONOS_SIGNER2: &str = "crc1q04jewhxw4xxu3vlg3rc85240h9q7ns6hglz0g";
pub(crate) const CRONOS_VALIDATOR1: &str = "crc12luku6uxehhak02py4rcz65zu0swh7wjsrw0pp";
pub(crate) const CRONOS_VALIDATOR2: &str = "crc18z6q38mhvtsvyr5mak8fj8s8g4gw7kjjtsgrn7";

pub(crate) const COMMUNITY_MNEMONIC: &str = "notable error gospel wave pair ugly measure elite toddler cost various fly make eye ketchup despair slab throw tribe swarm word fruit into inmate";
pub(crate) const DELEGATOR1_MNEMONIC: &str = "yard night airport critic main upper measure metal unhappy cliff pistol square upon access math owner enemy unfold scan small injury blind aunt million";
pub(crate) const DELEGATOR2_MNEMONIC: &str = "strong pyramid worth tennis option wet broccoli smoke midnight maze hint soft hen ignore shuffle multiply room recycle hurt degree crouch drill economy surge";
pub(crate) const SIGNER1_MNEMONIC: &str = "shed crumble dismiss loyal latin million oblige gesture shrug still oxygen custom remove ribbon disorder palace addict again blanket sad flock consider obey popular";
pub(crate) const SIGNER2_MNEMONIC: &str = "night renew tonight dinner shaft scheme domain oppose echo summer broccoli agent face guitar surface belt veteran siren poem alcohol menu custom crunch index";

pub(crate) const DEFAULT_GAS_LIMIT: u64 = 50_000_000;
pub(crate) const DEFAULT_FEE_AMOUNT: u64 = 25_000_000_000;

pub(crate) const DEFAULT_WAITING_SECS: u64 = 3;

// Helper functions

pub(crate) fn chainmain_client() -> CosmosClient {
    let config =
        CosmosClientConfig::new(CHAINMAIN_API_URL.to_owned(), TENDERMINT_RPC_URL.to_owned());
    CosmosClient::new(config)
}

pub(crate) async fn chainmain_tx_info(address: &str) -> CosmosSDKTxInfoRaw {
    let account = query_chainmain_account(address).await;
    CosmosSDKTxInfoRaw::new(
        account.account_number,
        account.sequence, // the sequence returned by server is what we need for next tx
        DEFAULT_GAS_LIMIT,
        DEFAULT_FEE_AMOUNT,
        CHAINMAIN_DENOM.to_owned(),
        0,
        Some("".to_owned()),
        CHAIN_ID.to_owned(),
        Network::CryptoOrgMainnet.get_bech32_hrp().to_owned(),
        Network::CryptoOrgMainnet.get_coin_type(),
    )
}

pub(crate) fn cronos_client() -> CosmosClient {
    let config = CosmosClientConfig::new(CRONOS_API_URL.to_owned(), TENDERMINT_RPC_URL.to_owned());
    CosmosClient::new(config)
}

pub(crate) fn get_private_key(mnemonic: &str) -> PrivateKey {
    let wallet = Wallet::recover_wallet(mnemonic.to_owned(), None).unwrap();
    wallet.get_key("m/44'/394'/0'/0/0".to_owned()).unwrap()
}

pub(crate) async fn query_chainmain_account(address: &str) -> RawRpcAccountStatus {
    let account_details =
        JsFuture::from(chainmain_client().query_account_details(address.to_owned()))
            .await
            .unwrap()
            .into_serde::<RawRpcAccountResponse>()
            .unwrap();

    match account_details {
        RawRpcAccountResponse::OkResponse { account } => account,
        _ => panic!("Failed to query account details"),
    }
}

pub(crate) async fn query_chainmain_balance(address: &str) -> RawRpcBalance {
    JsFuture::from(chainmain_client().query_account_balance(
        address.to_owned(),
        CHAINMAIN_DENOM.to_owned(),
        1,
    ))
    .await
    .unwrap()
    .into_serde::<RawRpcBalance>()
    .unwrap()
}

pub(crate) async fn query_cronos_balance(address: &str) -> RawRpcBalance {
    JsFuture::from(cronos_client().query_account_balance(
        address.to_owned(),
        CRONOS_DENOM.to_owned(),
        0,
    ))
    .await
    .unwrap()
    .into_serde::<RawRpcBalance>()
    .unwrap()
}

pub(crate) async fn wait_for_timeout(secs: Option<u64>) {
    Delay::new(Duration::from_secs(secs.unwrap_or(DEFAULT_WAITING_SECS)))
        .await
        .unwrap();
}
