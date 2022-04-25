//! Test suite for ethereum transactions.

#![cfg(target_arch = "wasm32")]

mod test_helper;
use defi_wallet_core_wasm::{
    broadcast_eth_signed_raw_tx, broadcast_transfer_eth, build_signed_eth_tx,
    get_eth_transaction_count, query_account_eth_balance, CoinType, EthTxAmount, EthTxInfo, Wallet,
};
use test_helper::*;
use wasm_bindgen_test::*;
use wasm_timer::Instant;

use js_sys::BigInt;

wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen_test]
async fn test_ethereum() {
    let from_wallet = Wallet::recover_wallet(SIGNER1_MNEMONIC.to_owned(), None).unwrap();
    let from_address = from_wallet.get_default_address(CoinType::Cronos).unwrap();
    let private_key = from_wallet.get_key_from_index(CoinType::Cronos, 0).unwrap();
    let to_wallet = Wallet::recover_wallet(SIGNER2_MNEMONIC.to_owned(), None).unwrap();
    let to_address = to_wallet.get_default_address(CoinType::Cronos).unwrap();
    let now = Instant::now();
    let initial_balance: BigInt =
        query_account_eth_balance(CRONOS_API_URL.to_owned(), to_address.clone())
            .await
            .unwrap();
    broadcast_transfer_eth(
        CRONOS_API_URL.to_owned(),
        to_address.clone(),
        EthTxAmount::new("100".to_owned(), "wei".to_owned()),
        777,
        4000,
        true,
        private_key.clone(),
    )
    .await
    .unwrap();
    let second_balance: BigInt =
        query_account_eth_balance(CRONOS_API_URL.to_owned(), to_address.clone())
            .await
            .unwrap();
    assert_eq!(second_balance, initial_balance + BigInt::from(100));
    assert!(now.elapsed().as_millis() > 4000); // test the interval works

    let nonce = get_eth_transaction_count(from_address, CRONOS_API_URL.to_string())
        .await
        .unwrap();
    let eth_tx_info = EthTxInfo::new(
        to_address.clone().into(),
        EthTxAmount::new("1".to_owned(), "gwei".to_owned()),
        nonce,
        BigInt::from(21000),
        EthTxAmount::new("7".to_owned(), "wei".to_owned()),
        None,
        true,
    );
    let raw_tx = build_signed_eth_tx(eth_tx_info, 777, private_key).unwrap();
    broadcast_eth_signed_raw_tx(raw_tx, CRONOS_API_URL.to_string(), 4000)
        .await
        .unwrap();

    let final_balance: BigInt =
        query_account_eth_balance(CRONOS_API_URL.to_owned(), to_address.clone())
            .await
            .unwrap();
    assert_eq!(final_balance, second_balance + BigInt::from(1000000000));
}
