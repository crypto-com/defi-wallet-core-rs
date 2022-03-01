//! Test helper constants and functions.

// Disable unused code warnings in this file. Since the helper constants and
// functions are not all used for each test files.
#![allow(dead_code)]

use core::time::Duration;
use defi_wallet_core_common::{Network, RawRpcAccountResponse, RawRpcAccountStatus, RawRpcBalance};
use defi_wallet_core_wasm::{query_account_balance, query_account_details, CosmosSDKTxInfoRaw};
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

pub(crate) const CRONOS_DELEGATOR1: &str = "crc1zgxux2e3m8aexy9husuglyez2dcdmyw6nlkv79";

pub(crate) const COMMUNITY_MNEMONIC: &str = "notable error gospel wave pair ugly measure elite toddler cost various fly make eye ketchup despair slab throw tribe swarm word fruit into inmate";
pub(crate) const DELEGATOR1_MNEMONIC: &str = "yard night airport critic main upper measure metal unhappy cliff pistol square upon access math owner enemy unfold scan small injury blind aunt million";
pub(crate) const DELEGATOR2_MNEMONIC: &str = "strong pyramid worth tennis option wet broccoli smoke midnight maze hint soft hen ignore shuffle multiply room recycle hurt degree crouch drill economy surge";
pub(crate) const SIGNER1_MNEMONIC: &str = "shed crumble dismiss loyal latin million oblige gesture shrug still oxygen custom remove ribbon disorder palace addict again blanket sad flock consider obey popular";
pub(crate) const SIGNER2_MNEMONIC: &str = "night renew tonight dinner shaft scheme domain oppose echo summer broccoli agent face guitar surface belt veteran siren poem alcohol menu custom crunch index";

// Helper functions

pub(crate) async fn query_account(address: &str) -> RawRpcAccountStatus {
    let account_details = query_account_details(CHAINMAIN_API_URL.to_owned(), address.to_owned())
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
    query_account_balance(
        CHAINMAIN_API_URL.to_owned(),
        address.to_owned(),
        CHAINMAIN_DENOM.to_owned(),
        1,
    )
    .await
    .unwrap()
    .into_serde::<RawRpcBalance>()
    .unwrap()
}

pub(crate) async fn query_cronos_balance(address: &str) -> RawRpcBalance {
    query_account_balance(
        CRONOS_API_URL.to_owned(),
        address.to_owned(),
        CRONOS_DENOM.to_owned(),
        0,
    )
    .await
    .unwrap()
    .into_serde::<RawRpcBalance>()
    .unwrap()
}

pub(crate) async fn get_tx_info(address: String) -> CosmosSDKTxInfoRaw {
    // Delay to wait the tx is included in the block, could be improved by waiting block
    let _ = Delay::new(Duration::from_millis(3000)).await;
    let account_details = query_account_details(CHAINMAIN_API_URL.to_owned(), address)
        .await
        .unwrap()
        .into_serde::<RawRpcAccountResponse>()
        .unwrap();

    let account = match account_details {
        RawRpcAccountResponse::OkResponse { account } => account,
        _ => panic!("Failed to query account details"),
    };

    CosmosSDKTxInfoRaw::new(
        account.account_number,
        account.sequence, // the sequence returned by server is what we need for next tx
        50000000,
        25000000000,
        CHAINMAIN_DENOM.to_owned(),
        0,
        Some("".to_owned()),
        CHAIN_ID.to_owned(),
        Network::CryptoOrgMainnet.get_bech32_hrp().to_owned(),
        Network::CryptoOrgMainnet.get_coin_type(),
    )
}
