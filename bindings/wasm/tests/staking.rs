//! Test suite for staking messages.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use defi_wallet_core_wasm::{CosmosMsg, CosmosTx};
use ethers::types::U256;
use std::assert_eq;
use test_helper::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const DELEGATE_AMOUNT: u64 = 100_000_000_000;
const REDELEGATE_AMOUNT: u64 = 50_000_000_000;
const UNBOND_AMOUNT: u64 = 50_000_000_000;

#[wasm_bindgen_test]
async fn test_delegate_and_unbound() {
    let balance1 = query_chainmain_balance(DELEGATOR1).await;
    send_delegate_msg(DELEGATOR1, DELEGATOR1_MNEMONIC, DELEGATE_AMOUNT).await;
    let balance2 = query_chainmain_balance(DELEGATOR1).await;
    assert_eq!(balance2.denom, CHAINMAIN_DENOM);

    // Balance should be equal to or greater than the previous balance since reward withdrawal.
    assert!(
        U256::from_dec_str(&balance2.amount).unwrap()
            >= U256::from_dec_str(&balance1.amount).unwrap() - DELEGATE_AMOUNT - DEFAULT_FEE_AMOUNT
    );

    send_unbond_msg(DELEGATOR1, DELEGATOR1_MNEMONIC, UNBOND_AMOUNT).await;
    let balance3 = query_chainmain_balance(DELEGATOR1).await;
    assert_eq!(balance3.denom, CHAINMAIN_DENOM);

    // Balance should be equal to or greater than the previous balance since reward withdrawal.
    assert!(
        U256::from_dec_str(&balance3.amount).unwrap()
            >= U256::from_dec_str(&balance2.amount).unwrap() + UNBOND_AMOUNT - DEFAULT_FEE_AMOUNT
    );
}

#[wasm_bindgen_test]
async fn test_redelegate() {
    let balance1 = query_chainmain_balance(DELEGATOR2).await;
    send_delegate_msg(DELEGATOR2, DELEGATOR2_MNEMONIC, DELEGATE_AMOUNT).await;
    let balance2 = query_chainmain_balance(DELEGATOR2).await;
    assert_eq!(balance2.denom, CHAINMAIN_DENOM);

    // Balance should be equal to or greater than the previous balance since reward withdrawal.
    assert!(
        U256::from_dec_str(&balance2.amount).unwrap()
            >= U256::from_dec_str(&balance1.amount).unwrap() - DELEGATE_AMOUNT - DEFAULT_FEE_AMOUNT
    );

    send_redelegate_msg(DELEGATOR2, DELEGATOR2_MNEMONIC, REDELEGATE_AMOUNT).await;
    let balance3 = query_chainmain_balance(DELEGATOR2).await;
    assert_eq!(balance3.denom, CHAINMAIN_DENOM);

    // Balance should be equal to or greater than the balance after delegating. Since rewards are
    // withdrawn from source validator.
    assert!(
        U256::from_dec_str(&balance3.amount).unwrap()
            >= U256::from_dec_str(&balance2.amount).unwrap() - DEFAULT_FEE_AMOUNT
    );
}

async fn send_delegate_msg(address: &str, mnemonic: &str, amount: u64) {
    let mut tx = CosmosTx::new();
    tx.add_msg(CosmosMsg::build_staking_delegate_msg(
        VALIDATOR1.to_owned(),
        amount,
        CHAINMAIN_DENOM.to_owned(),
    ));
    let signed_data = tx
        .sign_into(get_private_key(mnemonic), chainmain_tx_info(address).await)
        .unwrap();

    JsFuture::from(chainmain_client().broadcast_tx(signed_data))
        .await
        .unwrap();
    wait_for_timeout(None).await;
}

async fn send_redelegate_msg(address: &str, mnemonic: &str, amount: u64) {
    let mut tx = CosmosTx::new();
    tx.add_msg(CosmosMsg::build_staking_begin_redelegate_msg(
        VALIDATOR1.to_owned(),
        VALIDATOR2.to_owned(),
        amount,
        CHAINMAIN_DENOM.to_owned(),
    ));
    let signed_data = tx
        .sign_into(get_private_key(mnemonic), chainmain_tx_info(address).await)
        .unwrap();

    JsFuture::from(chainmain_client().broadcast_tx(signed_data))
        .await
        .unwrap();
    wait_for_timeout(None).await;
}

async fn send_unbond_msg(address: &str, mnemonic: &str, amount: u64) {
    let mut tx = CosmosTx::new();
    tx.add_msg(CosmosMsg::build_staking_undelegate_msg(
        VALIDATOR1.to_owned(),
        amount,
        CHAINMAIN_DENOM.to_owned(),
    ));
    let signed_data = tx
        .sign_into(get_private_key(mnemonic), chainmain_tx_info(address).await)
        .unwrap();

    JsFuture::from(chainmain_client().broadcast_tx(signed_data))
        .await
        .unwrap();
    wait_for_timeout(None).await;
}
