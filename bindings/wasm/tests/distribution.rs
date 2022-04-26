//! Test suite for distribution messages.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use defi_wallet_core_common::{Network, RawRpcBalance};
use defi_wallet_core_wasm::{CosmosMsg, CosmosTx};
use ethers::types::U256;
use std::assert_eq;
use test_helper::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const STAKING_DELEGATE_AMOUNT: u64 = 100_000_000_000;

#[wasm_bindgen_test]
async fn test_reward_withdrawed_to_default_address() {
    let mut tx = CosmosTx::new();
    add_delegate_msg(&mut tx);
    let signed_data = sign_tx(&mut tx).await;

    let balance1 = query_chainmain_balance(DELEGATOR1).await;
    JsFuture::from(chainmain_client().broadcast_tx(signed_data))
        .await
        .unwrap();
    wait_for_timeout().await;
    let balance2 = query_chainmain_balance(DELEGATOR1).await;

    assert_eq!(
        balance2,
        RawRpcBalance {
            denom: CHAINMAIN_DENOM.to_owned(),
            amount: (U256::from_dec_str(&balance1.amount).unwrap()
                - STAKING_DELEGATE_AMOUNT
                - DEFAULT_FEE_AMOUNT)
                .to_string()
        }
    );

    add_withdraw_reward_msg(&mut tx);
    let signed_data = sign_tx(&mut tx).await;
    JsFuture::from(chainmain_client().broadcast_tx(signed_data))
        .await
        .unwrap();
    wait_for_timeout().await;
    let balance3 = query_chainmain_balance(DELEGATOR1).await;

    assert_eq!(balance3.denom, CHAINMAIN_DENOM.to_owned());

    // Balance should be equal to or greater than the balance after delegating since reward
    // withdrawal.
    assert!(
        U256::from_dec_str(&balance3.amount).unwrap()
            >= U256::from_dec_str(&balance3.amount).unwrap() - DEFAULT_FEE_AMOUNT
    );
}

#[wasm_bindgen_test]
async fn test_reward_withdrawed_to_set_address() {
    let mut tx = CosmosTx::new();
    add_delegate_msg(&mut tx);
    add_set_withdraw_address_msg(&mut tx);
    let signed_data = sign_tx(&mut tx).await;

    JsFuture::from(chainmain_client().broadcast_tx(signed_data))
        .await
        .unwrap();
    wait_for_timeout().await;

    add_withdraw_reward_msg(&mut tx);
    let signed_data = sign_tx(&mut tx).await;

    let delegator_balance1 = query_chainmain_balance(DELEGATOR1).await;
    let withdrawer_balance1 = query_chainmain_balance(DELEGATOR2).await;
    JsFuture::from(chainmain_client().broadcast_tx(signed_data))
        .await
        .unwrap();
    wait_for_timeout().await;
    let delegator_balance2 = query_chainmain_balance(DELEGATOR1).await;
    let withdrawer_balance2 = query_chainmain_balance(DELEGATOR2).await;

    // Delegator should only pay the fee and receive no reward.
    assert_eq!(
        delegator_balance2,
        RawRpcBalance {
            denom: CHAINMAIN_DENOM.to_owned(),
            amount: (U256::from_dec_str(&delegator_balance1.amount).unwrap() - DEFAULT_FEE_AMOUNT)
                .to_string()
        }
    );

    // Withdrawer should get the reward.
    assert!(
        U256::from_dec_str(&withdrawer_balance2.amount).unwrap()
            > U256::from_dec_str(&withdrawer_balance1.amount).unwrap()
    );
}

fn add_delegate_msg(tx: &mut CosmosTx) {
    tx.add_msg(CosmosMsg::build_staking_delegate_msg(
        VALIDATOR1.to_owned(),
        STAKING_DELEGATE_AMOUNT,
        CHAINMAIN_DENOM.to_owned(),
    ));
}

fn add_set_withdraw_address_msg(tx: &mut CosmosTx) {
    tx.add_msg(CosmosMsg::build_distribution_set_withdraw_address_msg(
        DELEGATOR2.to_owned(),
    ));
}

fn add_withdraw_reward_msg(tx: &mut CosmosTx) {
    tx.add_msg(CosmosMsg::build_distribution_withdraw_delegator_reward_msg(
        VALIDATOR1.to_owned(),
    ));
}

async fn sign_tx(tx: &mut CosmosTx) -> Vec<u8> {
    tx.sign_into(
        get_private_key(DELEGATOR1_MNEMONIC),
        chainmain_tx_info(DELEGATOR1).await,
    )
    .unwrap()
}
