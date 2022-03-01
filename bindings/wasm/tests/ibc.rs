//! Test suite for ibc messages.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use core::time::Duration;
use defi_wallet_core_common::{Network, RawRpcBalance};
use defi_wallet_core_wasm::{
    broadcast_tx, get_ibc_transfer_signed_tx, CosmosSDKTxInfoRaw, PrivateKey, Wallet,
};
use ethers::types::U256;
use test_helper::*;
use wasm_bindgen_test::*;
use wasm_timer::{Delay, SystemTime, UNIX_EPOCH};

wasm_bindgen_test_configure!(run_in_browser);

// This test case only tests if message `MsgTransfer` could be processed for now.
// Need to wait ibc configuration for full test.
#[wasm_bindgen_test]
async fn test_transfer() {
    let private_key = get_private_key(SIGNER1_MNEMONIC);
    let src_beginning_balance = query_chainmain_balance(SIGNER1).await;
    let dst_beginning_balance = query_cronos_balance(CRONOS_DELEGATOR1).await;

    send_transfer_msg(&private_key, SIGNER1, CRONOS_DELEGATOR1).await;
    Delay::new(Duration::from_millis(6000)).await.unwrap();

    let src_after_transfer_balance = query_chainmain_balance(SIGNER1).await;

    assert_eq!(
        src_after_transfer_balance,
        RawRpcBalance {
            denom: CHAINMAIN_DENOM.to_owned(),
            amount: (U256::from_dec_str(&src_beginning_balance.amount).unwrap()
                - 25000000000u64
                - 5u64)
                .to_string(),
        }
    );

    let dst_after_transfer_balance = query_cronos_balance(CRONOS_DELEGATOR1).await;

    assert_eq!(
        dst_after_transfer_balance,
        RawRpcBalance {
            denom: CRONOS_DENOM.to_owned(),
            amount: (U256::from_dec_str(&dst_beginning_balance.amount).unwrap()
                + 5u64 * 10u64.pow(10))
            .to_string(),
        }
    );
}

async fn build_tx_info(address: &str) -> CosmosSDKTxInfoRaw {
    let account = query_account(address).await;

    CosmosSDKTxInfoRaw::new(
        account.account_number,
        account.sequence,
        50000000,
        25000000000,
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

async fn send_transfer_msg(private_key: &PrivateKey, sender: &str, receiver: &str) {
    let tx_info = build_tx_info(sender).await;

    let time_now = SystemTime::now();
    let timeout = time_now.duration_since(UNIX_EPOCH).unwrap() + Duration::new(120, 0);

    let signed_tx = get_ibc_transfer_signed_tx(
        tx_info,
        private_key.clone(),
        receiver.to_owned(),
        "transfer".to_owned(),
        "channel-0".to_owned(),
        CHAINMAIN_DENOM.to_owned(),
        5,
        0,
        0,
        timeout.as_nanos().try_into().unwrap(),
    )
    .unwrap();

    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap();
}
