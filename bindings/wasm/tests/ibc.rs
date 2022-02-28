//! Test suite for ibc messages.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use core::time::Duration;
use defi_wallet_core_common::Network;
use defi_wallet_core_wasm::{
    broadcast_tx, get_ibc_transfer_signed_tx, CosmosSDKTxInfoRaw, PrivateKey, Wallet,
};
use test_helper::*;
use wasm_bindgen_test::*;
use wasm_timer::Delay;

wasm_bindgen_test_configure!(run_in_browser);

// This test case only tests if message `MsgTransfer` could be processed for now.
// Need to wait ibc configuration for full test.
#[wasm_bindgen_test]
async fn test_transfer() {
    let private_key = get_private_key(DELEGATOR1_MNEMONIC);
    let _beginning_balance = query_balance(DELEGATOR1).await;

    send_transfer_msg(&private_key, DELEGATOR1, DELEGATOR2).await;

    Delay::new(Duration::from_millis(3000)).await.unwrap();

    // TODO: Need to validate the balance after ibc transfer when ibc is supported in integration-test.
    let _after_transfer_balance = query_balance(DELEGATOR1).await;
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

async fn send_transfer_msg(private_key: &PrivateKey, sender: &str, receiver: &str) {
    let tx_info = build_tx_info(sender).await;

    let signed_tx = get_ibc_transfer_signed_tx(
        tx_info,
        private_key.clone(),
        receiver.to_owned(),
        "transfer".to_owned(),
        "channel-3".to_owned(),
        DENOM.to_owned(),
        100000000000,
        0,
        0,
        0,
    )
    .unwrap();

    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap();
}
