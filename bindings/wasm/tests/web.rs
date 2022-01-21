//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]
use std::assert_eq;

use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

use defi_wallet_core_common::{
    HDWallet, RawRpcAccountResponse, RawRpcAccountStatus, RawRpcBalance, RawRpcPubKey,
};
use defi_wallet_core_wasm::{
    broadcast_tx, get_single_bank_send_signed_tx, query_account_balance, query_account_details,
    CoinType, CosmosSDKTxInfoRaw, PrivateKey, Wallet,
};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_query_account_details() {
    // Query account details from testnet
    let account = query_account_details(
        "https://testnet-croeseid-4.crypto.org:1317".to_owned(),
        "tcro1y6493k3smakl2wf09u7ds4amztx8ks7leyrtmy".to_owned(),
    )
    .await
    .unwrap();
    assert_eq!(account.is_object(), true);
    assert_eq!(
        account.into_serde::<RawRpcAccountResponse>().unwrap(),
        RawRpcAccountResponse::OkResponse {
            account: RawRpcAccountStatus {
                account_type: "/cosmos.auth.v1beta1.BaseAccount".to_owned(),
                address: "tcro1y6493k3smakl2wf09u7ds4amztx8ks7leyrtmy".to_owned(),
                pub_key: Some(RawRpcPubKey {
                    pub_key_type: "/cosmos.crypto.secp256k1.PubKey".to_owned(),
                    key: "A5bZFcAZ1jxFloOgHDmDKq2oHyIJQyuO3dGQL2buz1Ye".to_owned(),
                }),
                account_number: 1191,
                sequence: 11,
            }
        }
    );

    // Query account details from devnet
    let account = query_account_details(
        "http://127.0.0.1:26804".to_owned(),
        "cro1u08u5dvtnpmlpdq333uj9tcj75yceggszxpnsy".to_owned(),
    )
    .await
    .unwrap();

    assert_eq!(account.is_object(), true);
    assert_eq!(
        account.into_serde::<RawRpcAccountResponse>().unwrap(),
        RawRpcAccountResponse::OkResponse {
            account: RawRpcAccountStatus {
                account_type: "/cosmos.auth.v1beta1.BaseAccount".to_owned(),
                address: "cro1u08u5dvtnpmlpdq333uj9tcj75yceggszxpnsy".to_owned(),
                pub_key: None,
                account_number: 3,
                sequence: 0,
            }
        }
    );
}

#[wasm_bindgen_test]
async fn test_query_account_balance() {
    // Query account balance from testnet
    let balance = query_account_balance(
        "https://testnet-croeseid-4.crypto.org:1317".to_owned(),
        "tcro1y6493k3smakl2wf09u7ds4amztx8ks7leyrtmy".to_owned(),
        "basetcro".to_owned(),
        0,
    )
    .await
    .unwrap();

    assert_eq!(balance.is_object(), true);

    assert_eq!(
        balance.into_serde::<RawRpcBalance>().unwrap(),
        RawRpcBalance {
            denom: "basetcro".to_owned(),
            amount: "19449960400".to_owned()
        }
    );

    // Query account balance from devnet
    let balance = query_account_balance(
        "http://127.0.0.1:26804".to_owned(),
        "cro1u08u5dvtnpmlpdq333uj9tcj75yceggszxpnsy".to_owned(),
        "basecro".to_owned(),
        0,
    )
    .await
    .unwrap();

    assert_eq!(balance.is_object(), true);

    assert_eq!(
        balance.into_serde::<RawRpcBalance>().unwrap(),
        RawRpcBalance {
            denom: "basecro".to_owned(),
            amount: "1000000000000000000000".to_owned()
        }
    );
}

#[wasm_bindgen_test]
async fn test_get_single_bank_send_signed_tx() {
    let wallet = Wallet::recover_wallet("shed crumble dismiss loyal latin million oblige gesture shrug still oxygen custom remove ribbon disorder palace addict again blanket sad flock consider obey popular".to_owned(), None).unwrap();
    let address = wallet.get_default_address(CoinType::CryptoOrgMainnet);
    let key = wallet.get_key("m/44'/394'/0'/0/0".to_owned());
    let account_details = query_account_details(
        "http://127.0.0.1:26804".to_owned(),
        "cro1u08u5dvtnpmlpdq333uj9tcj75yceggszxpnsy".to_owned(),
    )
    .await
    .unwrap()
    .into_serde::<RawRpcAccountResponse>()
    .unwrap();

    match account_details {
        RawRpcAccountResponse::OkResponse { account } => {
            let tx_info = CosmosSDKTxInfoRaw::new(
                account.account_number,
                account.sequence,
                100000,
                1000000,
                "basecro".to_owned(),
                0,
                Some("example memo".to_owned()),
                "chainmain-1".to_owned(),
                "cro".to_owned(),
                394,
            );

            let key = PrivateKey::new();
            let signed_tx = get_single_bank_send_signed_tx(
                tx_info,
                key,
                "cro1apdh4yc2lnpephevc6lmpvkyv6s5cjh652n6e4".to_owned(),
                1000000,
                "basecro".to_owned(),
            )
            .unwrap();

            broadcast_tx("http://127.0.0.1:26807".to_owned(), signed_tx)
                .await
                .unwrap();

            let balance = query_account_balance(
                "http://127.0.0.1:26804".to_owned(),
                "cro1apdh4yc2lnpephevc6lmpvkyv6s5cjh652n6e4".to_owned(),
                "basecro".to_owned(),
                0,
            )
            .await
            .unwrap();

            assert_eq!(balance.is_object(), true);

            // WIP
            // assert_eq!(
            //     balance.into_serde::<RawRpcBalance>().unwrap(),
            //     RawRpcBalance {
            //         denom: "basecro".to_owned(),
            //         amount: "1000000000000001000000".to_owned()
            //     }
            // );
        }

        _ => {
            panic!("Query account details error.");
        }
    }
}
