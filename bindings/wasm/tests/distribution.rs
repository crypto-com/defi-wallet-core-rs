//! Test suite for distribution messages.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use core::time::Duration;
use defi_wallet_core_common::{Network, RawRpcBalance};
use defi_wallet_core_wasm::{
    broadcast_tx, get_distribution_set_withdraw_address_signed_tx,
    get_distribution_withdraw_reward_signed_tx, get_staking_delegate_signed_tx, CosmosSDKTxInfoRaw,
    PrivateKey, Wallet,
};
use ethers::types::U256;
use std::assert_eq;
use test_helper::*;
use wasm_bindgen_test::*;
use wasm_timer::Delay;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_reward_withdrawed_to_default_address() {
    let private_key = get_private_key(DELEGATOR1_MNEMONIC);
    let beginning_balance = query_balance(DELEGATOR1).await;

    send_delegate_msg(DELEGATOR1, VALIDATOR1, &private_key).await;
    Delay::new(Duration::from_millis(3000)).await.unwrap();

    let after_delegating_balance = query_balance(DELEGATOR1).await;

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

    send_withdraw_reward_msg(DELEGATOR1, VALIDATOR1, &private_key).await;
    Delay::new(Duration::from_millis(3000)).await.unwrap();

    let after_withdrawal_balance = query_balance(DELEGATOR1).await;
    assert_eq!(after_withdrawal_balance.denom, DENOM.to_owned());

    // Balance should be equal to or greater than the balance after delegating since reward withdrawal.
    assert!(
        U256::from_dec_str(&after_withdrawal_balance.amount).unwrap()
            >= U256::from_dec_str(&after_delegating_balance.amount).unwrap() - 25000000000u64
    );
}

#[wasm_bindgen_test]
async fn test_reward_withdrawed_to_set_address() {
    let private_key = get_private_key(DELEGATOR1_MNEMONIC);

    send_delegate_msg(DELEGATOR1, VALIDATOR1, &private_key).await;
    Delay::new(Duration::from_millis(3000)).await.unwrap();

    send_set_withdraw_address_msg(DELEGATOR1, DELEGATOR2, &private_key).await;
    Delay::new(Duration::from_millis(3000)).await.unwrap();

    let delegator_beginning_balance = query_balance(DELEGATOR1).await;
    let withdrawer_beginning_balance = query_balance(DELEGATOR2).await;

    send_withdraw_reward_msg(DELEGATOR1, VALIDATOR1, &private_key).await;
    Delay::new(Duration::from_millis(3000)).await.unwrap();

    let delegator_complete_balance = query_balance(DELEGATOR1).await;
    let withdrawer_complete_balance = query_balance(DELEGATOR2).await;

    // Delegator should only pay the fee and receive no reward.
    assert_eq!(
        delegator_complete_balance,
        RawRpcBalance {
            denom: DENOM.to_owned(),
            amount: (U256::from_dec_str(&delegator_beginning_balance.amount).unwrap()
                - 25000000000u64)
                .to_string()
        }
    );

    // Withdrawer should get the reward.
    assert!(
        U256::from_dec_str(&withdrawer_complete_balance.amount).unwrap()
            > U256::from_dec_str(&withdrawer_beginning_balance.amount).unwrap()
    );
}

async fn build_tx_info(address: &str) -> CosmosSDKTxInfoRaw {
    let account = query_account(address).await;

    CosmosSDKTxInfoRaw::new(
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
    )
}

fn get_private_key(mnemonic: &str) -> PrivateKey {
    let wallet = Wallet::recover_wallet(mnemonic.to_owned(), None).unwrap();
    wallet.get_key("m/44'/394'/0'/0/0".to_owned()).unwrap()
}

async fn send_delegate_msg(
    delegator_address: &str,
    validator_address: &str,
    private_key: &PrivateKey,
) {
    let tx_info = build_tx_info(delegator_address).await;

    let signed_tx = get_staking_delegate_signed_tx(
        tx_info,
        private_key.clone(),
        validator_address.to_owned(),
        100000000000,
        DENOM.to_owned(),
        true,
    )
    .unwrap();

    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap();
}

async fn send_set_withdraw_address_msg(
    delegator_address: &str,
    withdraw_address: &str,
    private_key: &PrivateKey,
) {
    let tx_info = build_tx_info(delegator_address).await;

    let signed_tx = get_distribution_set_withdraw_address_signed_tx(
        tx_info,
        private_key.clone(),
        withdraw_address.to_owned(),
    )
    .unwrap();

    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap();
}

async fn send_withdraw_reward_msg(
    delegator_address: &str,
    validator_address: &str,
    private_key: &PrivateKey,
) {
    let tx_info = build_tx_info(delegator_address).await;

    let signed_tx = get_distribution_withdraw_reward_signed_tx(
        tx_info,
        private_key.clone(),
        validator_address.to_owned(),
    )
    .unwrap();

    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap();
}
