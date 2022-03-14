//! Test suite for bank messages.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use defi_wallet_core_common::{Network, RawRpcBalance};
use defi_wallet_core_wasm::{
    broadcast_tx, get_single_bank_send_signed_tx, CoinType, CosmosSDKTxInfoRaw, Wallet,
};
use ethers::types::U256;
use std::assert_eq;
use test_helper::*;
use wasm_bindgen_test::*;
use wasm_timer::Delay;

wasm_bindgen_test_configure!(run_in_browser);

const BANK_SEND_AMOUNT: u64 = 100;

#[wasm_bindgen_test]
async fn test_get_single_bank_send_signed_tx() {
    let wallet = Wallet::recover_wallet(SIGNER1_MNEMONIC.to_owned(), None).unwrap();
    let address = wallet.get_default_address(CoinType::CryptoOrgMainnet);
    assert_eq!(address.unwrap(), SIGNER1.to_owned());
    let key = wallet.get_key("m/44'/394'/0'/0/0".to_owned()).unwrap();

    let account = query_account(SIGNER1).await;

    let tx_info = CosmosSDKTxInfoRaw::new(
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
    );

    // Query account balance from devnet
    let beginning_balance = query_chainmain_balance(SIGNER2).await;

    let signed_tx = get_single_bank_send_signed_tx(
        tx_info,
        key,
        SIGNER2.to_owned(),
        BANK_SEND_AMOUNT,
        CHAINMAIN_DENOM.to_owned(),
    )
    .unwrap();

    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap();

    // Delay to wait the tx is included in the block, could be improved by waiting block
    Delay::new(DEFAULT_WAITING_DURATION).await.unwrap();

    let balance = query_chainmain_balance(SIGNER2).await;

    assert_eq!(
        balance,
        RawRpcBalance {
            denom: CHAINMAIN_DENOM.to_owned(),
            amount: (U256::from_dec_str(&beginning_balance.amount).unwrap() + BANK_SEND_AMOUNT)
                .to_string()
        }
    );
}
