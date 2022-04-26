//! Test suite for bank messages.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use defi_wallet_core_common::RawRpcBalance;
use defi_wallet_core_wasm::{CosmosMsg, CosmosTx};
use ethers::types::U256;
use std::assert_eq;
use test_helper::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const BANK_SEND_AMOUNT: u64 = 100;

#[wasm_bindgen_test]
async fn test_get_single_bank_send_signed_tx() {
    let mut tx = CosmosTx::new();
    tx.add_msg(CosmosMsg::build_bank_send_msg(
        SIGNER2.to_owned(),
        BANK_SEND_AMOUNT,
        CHAINMAIN_DENOM.to_owned(),
    ));
    let signed_data = tx
        .sign_into(
            get_private_key(SIGNER1_MNEMONIC),
            chainmain_tx_info(SIGNER1).await,
        )
        .unwrap();

    let balance1 = query_chainmain_balance(SIGNER2).await;
    JsFuture::from(chainmain_client().broadcast_tx(signed_data))
        .await
        .unwrap();
    wait_for_timeout().await;
    let balance2 = query_chainmain_balance(SIGNER2).await;

    assert_eq!(
        balance2,
        RawRpcBalance {
            denom: CHAINMAIN_DENOM.to_owned(),
            amount: (U256::from_dec_str(&balance1.amount).unwrap() + BANK_SEND_AMOUNT).to_string()
        }
    );
}
