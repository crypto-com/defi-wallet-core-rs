//! Test suite for staking messages.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use core::time::Duration;
use defi_wallet_core_common::Network;
use defi_wallet_core_wasm::{
    broadcast_tx, get_staking_delegate_signed_tx, get_staking_redelegate_signed_tx,
    get_staking_unbond_signed_tx, CoinType, CosmosSDKTxInfoRaw, Wallet,
};
use ethers::types::U256;
use std::assert_eq;
use test_helper::*;
use wasm_bindgen_test::*;
use wasm_timer::Delay;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_delegate_and_unbound() {
    // Get private key.
    let wallet = Wallet::recover_wallet(DELEGATOR1_MNEMONIC.to_owned(), None).unwrap();
    let address = wallet.get_default_address(CoinType::CryptoOrgMainnet);
    assert_eq!(address.unwrap(), DELEGATOR1.to_owned());
    let private_key = wallet.get_key("m/44'/394'/0'/0/0".to_owned()).unwrap();

    // Query account for delegating.
    let account = query_account(DELEGATOR1).await;

    // Build tx info for delegating.
    let tx_info = CosmosSDKTxInfoRaw::new(
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
    );

    // Query balance before delegating.
    let beginning_balance = query_chainmain_balance(DELEGATOR1).await;

    // Send Delegate message.
    let signed_tx = get_staking_delegate_signed_tx(
        tx_info,
        private_key.clone(),
        VALIDATOR1.to_owned(),
        100000000000,
        CHAINMAIN_DENOM.to_owned(),
        true,
    )
    .unwrap();

    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap();

    // Delay to wait the tx is included in the block, could be improved by waiting block
    Delay::new(Duration::from_millis(3000)).await.unwrap();

    // Query and compare balance after delegating.
    let after_delegating_balance = query_chainmain_balance(DELEGATOR1).await;
    assert_eq!(after_delegating_balance.denom, CHAINMAIN_DENOM.to_owned());

    // Balance should be equal to or greater than the previous balance since reward withdrawal.
    assert!(
        U256::from_dec_str(&after_delegating_balance.amount).unwrap()
            >= U256::from_dec_str(&beginning_balance.amount).unwrap()
                - 100000000000u64
                - 25000000000u64
    );

    // Query account for unbonding. Since `account.sequence` is changed.
    let account = query_account(DELEGATOR1).await;

    // Build tx info for unbonding.
    let tx_info = CosmosSDKTxInfoRaw::new(
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
    );

    // Send Undelegate message.
    let signed_tx = get_staking_unbond_signed_tx(
        tx_info,
        private_key,
        VALIDATOR1.to_owned(),
        50000000000,
        CHAINMAIN_DENOM.to_owned(),
        true,
    )
    .unwrap();

    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap();

    // Delay to wait the tx is included in the block, could be improved by waiting block
    Delay::new(Duration::from_millis(4000)).await.unwrap();

    // Query and compare balance after unbonding.
    let after_unbonding_balance = query_chainmain_balance(DELEGATOR1).await;
    assert_eq!(after_unbonding_balance.denom, CHAINMAIN_DENOM.to_owned());

    // TODO:
    // The value of unbonding_time is set to 30s in `scripts/chainmain-devnet.yaml`
    // (because of `trusting_period` of `scripts/hermes.toml` cannot be set to such low and
    // must be less than unbonding_time).
    // We cannot wait for 30s here, and may only test with `chainmain` and another configuration
    // with a lower unbonding_time.

    // Balance should be equal to or greater than the previous balance since reward withdrawal.
    // assert!(
    //     U256::from_dec_str(&after_unbonding_balance.amount).unwrap()
    //         >= U256::from_dec_str(&after_delegating_balance.amount).unwrap() + 50000000000u64
    //             - 25000000000u64
    // );
}

#[wasm_bindgen_test]
async fn test_redelegate() {
    // Get private key.
    let wallet = Wallet::recover_wallet(DELEGATOR2_MNEMONIC.to_owned(), None).unwrap();
    let address = wallet.get_default_address(CoinType::CryptoOrgMainnet);
    assert_eq!(address.unwrap(), DELEGATOR2.to_owned());
    let private_key = wallet.get_key("m/44'/394'/0'/0/0".to_owned()).unwrap();

    let account = query_account(DELEGATOR2).await;

    // Build tx info for delegating.
    let tx_info = CosmosSDKTxInfoRaw::new(
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
    );

    // Query balance before delegating.
    let beginning_balance = query_chainmain_balance(DELEGATOR2).await;

    // Send Delegate message.
    let signed_tx = get_staking_delegate_signed_tx(
        tx_info,
        private_key.clone(),
        VALIDATOR1.to_owned(),
        100000000000,
        CHAINMAIN_DENOM.to_owned(),
        true,
    )
    .unwrap();
    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap();

    // Delay to wait the tx is included in the block, could be improved by waiting block
    Delay::new(Duration::from_millis(3000)).await.unwrap();

    // Query and compare balance after delegating.
    let after_delegating_balance = query_chainmain_balance(DELEGATOR2).await;
    assert_eq!(after_delegating_balance.denom, CHAINMAIN_DENOM.to_owned());

    // Balance should be equal to or greater than the previous balance since reward withdrawal.
    assert!(
        U256::from_dec_str(&after_delegating_balance.amount).unwrap()
            >= U256::from_dec_str(&beginning_balance.amount).unwrap()
                - 100000000000u64
                - 25000000000u64
    );

    // Query account for redelegating. Since `account.sequence` is changed.
    let account = query_account(DELEGATOR2).await;

    // Build tx info for redelegating.
    let tx_info = CosmosSDKTxInfoRaw::new(
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
    );

    // Send BeginRedelegate message.
    let signed_tx = get_staking_redelegate_signed_tx(
        tx_info,
        private_key,
        VALIDATOR1.to_owned(),
        VALIDATOR2.to_owned(),
        50000000000,
        CHAINMAIN_DENOM.to_owned(),
        true,
    )
    .unwrap();
    broadcast_tx(TENDERMINT_RPC_URL.to_owned(), signed_tx)
        .await
        .unwrap();

    // Delay to wait the tx is included in the block, could be improved by waiting block
    Delay::new(Duration::from_millis(4000)).await.unwrap();

    // Query and compare balance after redelegating.
    let after_redelegating_balance = query_chainmain_balance(DELEGATOR2).await;

    assert_eq!(after_redelegating_balance.denom, CHAINMAIN_DENOM.to_owned());

    // Balance should be equal to or greater than the balance after delegating.
    // Since rewards are withdrawn from source validator.
    assert!(
        U256::from_dec_str(&after_redelegating_balance.amount).unwrap()
            >= U256::from_dec_str(&after_delegating_balance.amount).unwrap() - 25000000000u64
    );
}
