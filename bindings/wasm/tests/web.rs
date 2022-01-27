//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]
use std::assert_eq;

use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

use defi_wallet_core_common::{
    HDWallet, Network, RawRpcAccountResponse, RawRpcAccountStatus, RawRpcBalance, RawRpcPubKey,
};
use defi_wallet_core_wasm::{
    broadcast_tx, get_single_bank_send_signed_tx, query_account_balance, query_account_details,
    CoinType, CosmosSDKTxInfoRaw, PrivateKey, Wallet,
};

use core::time::Duration;
use ethers::types::U256;
use wasm_timer::Delay;

const API_URL: &str = "http://127.0.0.1:26804";
const SIGNER1: &str = "cro1u08u5dvtnpmlpdq333uj9tcj75yceggszxpnsy";
const SIGNER2: &str = "cro1apdh4yc2lnpephevc6lmpvkyv6s5cjh652n6e4";
const COMMUNITY: &str = "cro1qj4u2y23hx7plrztswrel2hgf8mh2m22k80fet";
const VALIDATOR: &str = "cro1dkwjtmkueye3fqwzyv2jrdn7fspd2jkmjns43y";
const DENOM: &str = "basecro";
const TENDERMINT_RPC_URL: &str = "http://127.0.0.1:26807";
const CHAIN_ID: &str = "chainmain-1";

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
    let wallet = Wallet::recover_wallet("shed crumble dismiss loyal latin million oblige gesture shrug still oxygen custom remove ribbon disorder palace addict again blanket sad flock consider obey popular".to_owned(), None).unwrap();
    let address = wallet.get_default_address(CoinType::CryptoOrgMainnet);
    assert_eq!(address.unwrap(), SIGNER1.to_owned());
    let key = wallet.get_key("m/44'/394'/0'/0/0".to_owned()).unwrap();
    let account_details = query_account_details(API_URL.to_owned(), SIGNER1.to_owned())
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

    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap();

    // Delay to wait the tx is included in the block, could be improved by waiting block
    Delay::new(Duration::from_millis(3000)).await;

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
