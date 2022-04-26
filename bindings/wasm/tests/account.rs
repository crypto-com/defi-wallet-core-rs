//! Test suite for account queries.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use std::assert_eq;
use test_helper::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_query_chainmain_account() {
    let account = query_chainmain_account(COMMUNITY).await;

    assert_eq!(account.account_type, "/cosmos.auth.v1beta1.BaseAccount");
    assert_eq!(account.address, COMMUNITY);
}

#[wasm_bindgen_test]
async fn test_query_chainmain_balance() {
    let balance = query_chainmain_balance(COMMUNITY).await;

    assert_eq!(balance.denom, CHAINMAIN_DENOM);
    assert!(balance.amount.parse::<u128>().unwrap() > 0);
}

#[wasm_bindgen_test]
async fn test_query_cronos_balance() {
    let balance = query_cronos_balance(CRONOS_COMMUNITY).await;

    assert_eq!(balance.denom, CRONOS_DENOM);
    assert!(balance.amount.parse::<u128>().unwrap() > 0);
}
