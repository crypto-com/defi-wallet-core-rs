//! Test suite for account queries.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use defi_wallet_core_common::{RawRpcAccountStatus, RawRpcBalance};
use std::assert_eq;
use test_helper::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_query_account_details() {
    // Query account details from devnet
    let account = query_account(COMMUNITY).await;

    assert_eq!(
        account,
        RawRpcAccountStatus {
            account_type: "/cosmos.auth.v1beta1.BaseAccount".to_owned(),
            address: COMMUNITY.to_owned(),
            pub_key: None,
            account_number: 2,
            sequence: 0,
        }
    );
}

#[wasm_bindgen_test]
async fn test_query_account_balance() {
    // Query account balance from devnet
    let balance = query_balance(COMMUNITY).await;

    assert_eq!(
        balance,
        RawRpcBalance {
            denom: DENOM.to_owned(),
            amount: "1000000000000000000000".to_owned()
        }
    );
}
