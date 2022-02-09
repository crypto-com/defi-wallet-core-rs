//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]
use std::assert_eq;

use wasm_bindgen_test::*;

use defi_wallet_core_common::{Network, RawRpcAccountResponse, RawRpcAccountStatus, RawRpcBalance};
use defi_wallet_core_wasm::{
    broadcast_tx, get_nft_issue_denom_signed_tx, get_single_bank_send_signed_tx,
    get_staking_delegate_signed_tx, get_staking_unbond_signed_tx, query_account_balance,
    query_account_details, query_denoms, CoinType, CosmosSDKTxInfoRaw, Wallet,
};

use tendermint_rpc::endpoint::broadcast::tx_sync::Response;

use core::time::Duration;
use ethers::types::U256;
use wasm_timer::Delay;

const API_URL: &str = "http://127.0.0.1:26804";
const SIGNER1: &str = "cro1u08u5dvtnpmlpdq333uj9tcj75yceggszxpnsy";
const SIGNER2: &str = "cro1apdh4yc2lnpephevc6lmpvkyv6s5cjh652n6e4";
const COMMUNITY: &str = "cro1qj4u2y23hx7plrztswrel2hgf8mh2m22k80fet";
const DELEGATOR: &str = "cro1ykec6vralvrh5vcvpf7w7u02gj728u4wp738kz";
const VALIDATOR: &str = "crocncl1pk9eajj4zuzpptnadwz6tzfgcpchqvpkvql0a9";
const DENOM: &str = "basecro";
const TENDERMINT_RPC_URL: &str = "http://127.0.0.1:26807";
const CHAIN_ID: &str = "chainmain-1";
const DELEGATOR_MNEMONIC: &str = "yard night airport critic main upper measure metal unhappy cliff pistol square upon access math owner enemy unfold scan small injury blind aunt million";
const SIGNER1_MNEMONIC: &str = "shed crumble dismiss loyal latin million oblige gesture shrug still oxygen custom remove ribbon disorder palace addict again blanket sad flock consider obey popular";
const SIGNER2_MNEMONIC: &str = "night renew tonight dinner shaft scheme domain oppose echo summer broccoli agent face guitar surface belt veteran siren poem alcohol menu custom crunch index";

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_query_account_details() {
    // Query account details from devnet
    let account = query_account_details(API_URL.to_owned(), COMMUNITY.to_owned())
        .await
        .unwrap();

    assert_eq!(account.is_object(), true);
    assert_eq!(
        account.into_serde::<RawRpcAccountResponse>().unwrap(),
        RawRpcAccountResponse::OkResponse {
            account: RawRpcAccountStatus {
                account_type: "/cosmos.auth.v1beta1.BaseAccount".to_owned(),
                address: COMMUNITY.to_owned(),
                pub_key: None,
                account_number: 2,
                sequence: 0,
            }
        }
    );
}

#[wasm_bindgen_test]
async fn test_query_account_balance() {
    // Query account balance from devnet
    let balance = query_account_balance(
        API_URL.to_owned(),
        COMMUNITY.to_owned(),
        DENOM.to_owned(),
        1,
    )
    .await
    .unwrap();

    assert_eq!(balance.is_object(), true);

    assert_eq!(
        balance.into_serde::<RawRpcBalance>().unwrap(),
        RawRpcBalance {
            denom: DENOM.to_owned(),
            amount: "1000000000000000000000".to_owned()
        }
    );
}

#[wasm_bindgen_test]
async fn test_get_single_bank_send_signed_tx() {
    let wallet = Wallet::recover_wallet(SIGNER1_MNEMONIC.to_owned(), None).unwrap();
    let address = wallet.get_default_address(CoinType::CryptoOrgMainnet);
    assert_eq!(address.unwrap(), SIGNER1.to_owned());
    let key = wallet.get_key("m/44'/394'/0'/0/0".to_owned()).unwrap();
    let account_details = query_account_details(API_URL.to_owned(), SIGNER1.to_owned())
        .await
        .unwrap()
        .into_serde::<RawRpcAccountResponse>()
        .unwrap();

    let account = match account_details {
        RawRpcAccountResponse::OkResponse { account } => account,
        _ => panic!("Failed to query account details"),
    };

    let tx_info = CosmosSDKTxInfoRaw::new(
        account.account_number,
        account.sequence,
        50000000,
        25000000000,
        DENOM.to_owned(),
        0,
        Some("".to_owned()),
        CHAIN_ID.to_owned(),
        Network::CryptoOrgMainnet.get_bech32_hrp().to_owned(),
        Network::CryptoOrgMainnet.get_coin_type(),
    );

    // Query account balance from devnet
    let beginning_balance =
        query_account_balance(API_URL.to_owned(), SIGNER2.to_owned(), DENOM.to_owned(), 1)
            .await
            .unwrap()
            .into_serde::<RawRpcBalance>()
            .unwrap();

    let signed_tx =
        get_single_bank_send_signed_tx(tx_info, key, SIGNER2.to_owned(), 100, DENOM.to_owned())
            .unwrap();

    let res = broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap()
        .into_serde::<Response>()
        .unwrap();

    assert_eq!(res.code, tendermint::abci::Code::Ok);

    // Delay to wait the tx is included in the block, could be improved by waiting block
    Delay::new(Duration::from_millis(3000)).await.unwrap();

    let balance = query_account_balance(
        API_URL.to_owned(),
        SIGNER2.to_owned(),
        DENOM.to_owned(),
        100,
    )
    .await
    .unwrap();

    assert_eq!(balance.is_object(), true);

    assert_eq!(
        balance.into_serde::<RawRpcBalance>().unwrap(),
        RawRpcBalance {
            denom: DENOM.to_owned(),
            amount: (U256::from_dec_str(&beginning_balance.amount).unwrap() + 100).to_string()
        }
    );
}

#[wasm_bindgen_test]
async fn test_staking() {
    // Get private key.
    let wallet = Wallet::recover_wallet(DELEGATOR_MNEMONIC.to_owned(), None).unwrap();
    let address = wallet.get_default_address(CoinType::CryptoOrgMainnet);
    assert_eq!(address.unwrap(), DELEGATOR.to_owned());
    let private_key = wallet.get_key("m/44'/394'/0'/0/0".to_owned()).unwrap();

    // Query account for delegating.
    let account_details = query_account_details(API_URL.to_owned(), DELEGATOR.to_owned())
        .await
        .unwrap()
        .into_serde::<RawRpcAccountResponse>()
        .unwrap();

    let account = match account_details {
        RawRpcAccountResponse::OkResponse { account } => account,
        _ => panic!("Failed to query account details"),
    };

    // Build tx info for delegating.
    let tx_info = CosmosSDKTxInfoRaw::new(
        account.account_number,
        account.sequence,
        50000000,
        25000000000,
        DENOM.to_owned(),
        0,
        Some("".to_owned()),
        CHAIN_ID.to_owned(),
        Network::CryptoOrgMainnet.get_bech32_hrp().to_owned(),
        Network::CryptoOrgMainnet.get_coin_type(),
    );

    // Query balance before delegating.
    let beginning_balance = query_account_balance(
        API_URL.to_owned(),
        DELEGATOR.to_owned(),
        DENOM.to_owned(),
        1,
    )
    .await
    .unwrap()
    .into_serde::<RawRpcBalance>()
    .unwrap();

    // Send Delegate message.
    let signed_tx = get_staking_delegate_signed_tx(
        tx_info,
        private_key.clone(),
        VALIDATOR.to_owned(),
        100000000000,
        DENOM.to_owned(),
    )
    .unwrap();
    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap();

    // Delay to wait the tx is included in the block, could be improved by waiting block
    Delay::new(Duration::from_millis(3000)).await.unwrap();

    // Query and compare balance after delegating.
    let after_delegating_balance = query_account_balance(
        API_URL.to_owned(),
        DELEGATOR.to_owned(),
        DENOM.to_owned(),
        1,
    )
    .await
    .unwrap()
    .into_serde::<RawRpcBalance>()
    .unwrap();

    assert_eq!(
        after_delegating_balance,
        RawRpcBalance {
            denom: DENOM.to_owned(),
            amount: (U256::from_dec_str(&beginning_balance.amount).unwrap()
                - 100000000000u64
                - 25000000000u64)
                .to_string()
        }
    );

    // Query account for unbonding. Since `account.sequence` is changed.
    let account_details = query_account_details(API_URL.to_owned(), DELEGATOR.to_owned())
        .await
        .unwrap()
        .into_serde::<RawRpcAccountResponse>()
        .unwrap();

    let account = match account_details {
        RawRpcAccountResponse::OkResponse { account } => account,
        _ => panic!("Failed to query account details"),
    };

    // Build tx info for unbonding.
    let tx_info = CosmosSDKTxInfoRaw::new(
        account.account_number,
        account.sequence,
        50000000,
        25000000000,
        DENOM.to_owned(),
        0,
        Some("".to_owned()),
        CHAIN_ID.to_owned(),
        Network::CryptoOrgMainnet.get_bech32_hrp().to_owned(),
        Network::CryptoOrgMainnet.get_coin_type(),
    );

    // Send Undelegate message.
    let signed_tx = get_staking_unbond_signed_tx(
        tx_info,
        private_key,
        VALIDATOR.to_owned(),
        50000000000,
        DENOM.to_owned(),
    )
    .unwrap();
    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap();

    // Delay to wait the tx is included in the block, could be improved by waiting block
    Delay::new(Duration::from_millis(4000)).await.unwrap();

    // Query and compare balance after unbonding.
    let after_unbonding_balance = query_account_balance(
        API_URL.to_owned(),
        DELEGATOR.to_owned(),
        DENOM.to_owned(),
        1,
    )
    .await
    .unwrap()
    .into_serde::<RawRpcBalance>()
    .unwrap();

    assert_eq!(after_unbonding_balance.denom, DENOM.to_owned());

    // Balance should be equal to or greater than the previous balance since reward.
    assert!(
        U256::from_dec_str(&after_unbonding_balance.amount).unwrap()
            >= U256::from_dec_str(&after_delegating_balance.amount).unwrap() + 50000000000u64
                - 25000000000u64
    );
}

#[wasm_bindgen_test]
async fn test_get_nft_issue_denom_signed_tx() {
    let wallet = Wallet::recover_wallet(SIGNER2_MNEMONIC.to_owned(), None).unwrap();
    let address = wallet.get_default_address(CoinType::CryptoOrgMainnet);
    assert_eq!(address.unwrap(), SIGNER2.to_owned());
    let key = wallet.get_key("m/44'/394'/0'/0/0".to_owned()).unwrap();
    let account_details = query_account_details(API_URL.to_owned(), SIGNER2.to_owned())
        .await
        .unwrap()
        .into_serde::<RawRpcAccountResponse>()
        .unwrap();

    let mut account_number = 0;
    let mut sequence = 0;
    if let RawRpcAccountResponse::OkResponse { account } = account_details {
        account_number = account.account_number;
        sequence = account.sequence;
    } else {
        panic!("Query account details error.");
    }

    let tx_info = CosmosSDKTxInfoRaw::new(
        account_number,
        sequence,
        50000000,
        25000000000,
        DENOM.to_owned(),
        0,
        Some("".to_owned()),
        CHAIN_ID.to_owned(),
        Network::CryptoOrgMainnet.get_bech32_hrp().to_owned(),
        Network::CryptoOrgMainnet.get_coin_type(),
    );

    let signed_tx = get_nft_issue_denom_signed_tx(
        tx_info,
        key,
        "testdenomid".to_owned(),
        "testdenomname".to_owned(),
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
                    }"#
        .to_string(),
    )
    .unwrap();

    let res = broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap()
        .into_serde::<Response>()
        .unwrap();

    assert_eq!(res.code, tendermint::abci::Code::Ok);

    query_denoms(API_URL.to_owned()).await;
}
