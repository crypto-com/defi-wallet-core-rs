//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]
use std::assert_eq;

use wasm_bindgen_test::*;
use wasm_bindgen::JsValue;

use defi_wallet_core_wasm::{
    get_single_bank_send_signed_tx, query_account_balance, query_account_details, CoinType,
    CosmosSDKTxInfoRaw, PrivateKey, Wallet
};
use defi_wallet_core_common::RawRpcBalance;


wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_wasm() {
    let wallet = Wallet::new(Some("".to_owned()));

    // assert_eq!(
    //     wallet.get_default_address(CoinType::CosmosHub).unwrap(),
    //     "".to_owned()
    // );

    let tx_info = CosmosSDKTxInfoRaw::new(
        1,
        1,
        100000,
        1000000,
        "uatom".to_owned(),
        9001,
        Some("example memo".to_owned()),
        "cosmoshub-4".to_owned(),
        "cosmos".to_owned(),
        118,
    );

    let key = PrivateKey::new();
    let signed_tx = get_single_bank_send_signed_tx(
        tx_info,
        key,
        "cosmos19dyl0uyzes4k23lscla02n06fc22h4uqsdwq6z".to_owned(),
        1000000,
        "uatom".to_owned(),
    )
    .unwrap();
    // assert_eq!(signed_tx, vec![1]);

    let account = query_account_details(
        "https://testnet-croeseid-4.crypto.org:1317".to_owned(),
        "tcro1y6493k3smakl2wf09u7ds4amztx8ks7leyrtmy".to_owned(),
    )
    .await
    .unwrap();

    // assert_eq!(account, "hello");

    let balance = query_account_balance(
        "https://testnet-croeseid-4.crypto.org:1317".to_owned(),
        "tcro1y6493k3smakl2wf09u7ds4amztx8ks7leyrtmy".to_owned(),
        "basetcro".to_owned(),
        0,
    )
    .await
    .unwrap();

    // assert_eq!(
    //     balance,
    //     JsValue::from_serde(&RawRpcBalance {
    //         denom: "basetcro".to_owned(),
    //         amount: "19449960400".to_owned()
    //     }).unwrap()
    // );
}
