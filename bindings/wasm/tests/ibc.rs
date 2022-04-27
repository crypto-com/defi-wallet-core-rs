//! Test suite for ibc messages.

#![cfg(target_arch = "wasm32")]
#![cfg(feature = "ibc-test")]

mod test_helper;

use core::time::Duration;
use defi_wallet_core_common::RawRpcBalance;
use defi_wallet_core_wasm::{CosmosMsg, CosmosTx};
use ethers::types::U256;
use test_helper::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use wasm_timer::{SystemTime, UNIX_EPOCH};

// basecro is a 8 decimals token and basetcro is a 18 decimals token
const DECIMAL_RATIO: u64 = 10_u64.pow(10); // basetcro to basecro

const TRANSFER_AMOUNT: u64 = 5; // basecro

wasm_bindgen_test_configure!(run_in_browser);

// This test case only tests if message `MsgTransfer` could be processed for now.
// Need to wait ibc configuration for full test.
#[wasm_bindgen_test]
async fn test_transfer() {
    let balance1 = query_cronos_balance(CRONOS_DELEGATOR1).await;
    send_transfer_msg().await;
    let balance2 = query_cronos_balance(CRONOS_DELEGATOR1).await;

    assert_eq!(
        balance2,
        RawRpcBalance {
            denom: CRONOS_DENOM.to_owned(),
            amount: (U256::from_dec_str(&balance1.amount).unwrap()
                + TRANSFER_AMOUNT * DECIMAL_RATIO)
                .to_string(),
        }
    );
}

async fn send_transfer_msg() {
    let time_now = SystemTime::now();
    let timeout = time_now.duration_since(UNIX_EPOCH).unwrap() + Duration::new(120, 0);

    let mut tx = CosmosTx::new();
    tx.add_msg(CosmosMsg::build_ibc_transfer_msg(
        CRONOS_DELEGATOR1.to_owned(),
        "transfer".to_owned(),
        "channel-0".to_owned(),
        CHAINMAIN_DENOM.to_owned(),
        5,
        0,
        0,
        timeout.as_nanos().try_into().unwrap(),
    ));
    let signed_data = tx
        .sign_into(
            get_private_key(SIGNER1_MNEMONIC),
            chainmain_tx_info(SIGNER1).await,
        )
        .unwrap();

    JsFuture::from(chainmain_client().broadcast_tx(signed_data))
        .await
        .unwrap();
    wait_for_timeout(None).await;
}
