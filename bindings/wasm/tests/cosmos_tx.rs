//! Test suite for Cosmos message and transaction wrappers.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use defi_wallet_core_common::Network;
use defi_wallet_core_wasm::{
    broadcast_tx, CosmosMsg, CosmosSDKTxInfoRaw, CosmosTx, PrivateKey, Wallet,
};
use ethers::types::U256;
use std::assert_eq;
use test_helper::*;
use wasm_bindgen_test::*;
use wasm_timer::Delay;

wasm_bindgen_test_configure!(run_in_browser);

const BANK_SEND_AMOUNT: u64 = 50_000_000_000;
const STAKING_DELEGATE_AMOUNT: u64 = 100_000_000_000;

#[wasm_bindgen_test]
async fn test_tx_build_with_multiple_msgs() {
    // Create a transaction.
    let mut tx = CosmosTx::new();

    // Add a staking delegate message.
    tx.add_msg(CosmosMsg::build_staking_delegate_msg(
        VALIDATOR1.to_owned(),
        STAKING_DELEGATE_AMOUNT,
        CHAINMAIN_DENOM.to_owned(),
    ));

    // Add a bank send message.
    tx.add_msg(CosmosMsg::build_bank_send_msg(
        DELEGATOR2.to_owned(),
        BANK_SEND_AMOUNT,
        CHAINMAIN_DENOM.to_owned(),
    ));

    // Sign the transaction and move out all pending messages.
    assert_eq!(tx.get_msg_count(), 2);
    let private_key = get_private_key(DELEGATOR1_MNEMONIC);
    let tx_info = build_tx_info(DELEGATOR1).await;
    let tx_data = tx.sign_into(private_key, tx_info).unwrap();
    assert_eq!(tx.get_msg_count(), 0);

    // Query balance before sending transaction.
    let beginning_balance = query_chainmain_balance(DELEGATOR1).await;

    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), tx_data)
        .await
        .unwrap();
    Delay::new(DEFAULT_WAITING_DURATION).await.unwrap();

    // Query and compare balance after sending transaction.
    let after_transaction_balance = query_chainmain_balance(DELEGATOR1).await;
    assert_eq!(after_transaction_balance.denom, CHAINMAIN_DENOM.to_owned());

    // Balance should be equal to or greater than the previous balance since reward withdrawal.
    assert!(
        U256::from_dec_str(&after_transaction_balance.amount).unwrap()
            >= U256::from_dec_str(&beginning_balance.amount).unwrap()
                - STAKING_DELEGATE_AMOUNT
                - BANK_SEND_AMOUNT
                - DEFAULT_FEE_AMOUNT
    );
}

async fn build_tx_info(address: &str) -> CosmosSDKTxInfoRaw {
    let account = query_account(address).await;

    CosmosSDKTxInfoRaw::new(
        account.account_number,
        account.sequence,
        DEFAULT_GAS_LIMIT,
        DEFAULT_FEE_AMOUNT,
        CHAINMAIN_DENOM.to_owned(),
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
