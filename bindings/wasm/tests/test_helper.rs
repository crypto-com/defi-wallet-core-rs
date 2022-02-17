//! Test helper constants and functions.

// Disable unused code warnings in this file. Since the helper constants and
// functions are not all used for each test files.
#![allow(dead_code)]

use defi_wallet_core_common::{RawRpcAccountResponse, RawRpcAccountStatus, RawRpcBalance};
use defi_wallet_core_wasm::{query_account_balance, query_account_details};

const API_URL: &str = "http://127.0.0.1:26804";

pub(crate) const CHAIN_ID: &str = "chainmain-1";
pub(crate) const DENOM: &str = "basecro";
pub(crate) const TENDERMINT_RPC_URL: &str = "http://127.0.0.1:26807";

pub(crate) const COMMUNITY: &str = "cro1qj4u2y23hx7plrztswrel2hgf8mh2m22k80fet";
pub(crate) const DELEGATOR1: &str = "cro1ykec6vralvrh5vcvpf7w7u02gj728u4wp738kz";
pub(crate) const DELEGATOR2: &str = "cro1tmfhgwp62uhz5y5hqcyl8jkjq22l2cles2lum8";
pub(crate) const SIGNER1: &str = "cro1u08u5dvtnpmlpdq333uj9tcj75yceggszxpnsy";
pub(crate) const SIGNER2: &str = "cro1apdh4yc2lnpephevc6lmpvkyv6s5cjh652n6e4";
pub(crate) const VALIDATOR1: &str = "crocncl1pk9eajj4zuzpptnadwz6tzfgcpchqvpkvql0a9";
pub(crate) const VALIDATOR2: &str = "crocncl1dkwjtmkueye3fqwzyv2jrdn7fspd2jkm37nunc";

pub(crate) const DELEGATOR1_MNEMONIC: &str = "yard night airport critic main upper measure metal unhappy cliff pistol square upon access math owner enemy unfold scan small injury blind aunt million";
pub(crate) const DELEGATOR2_MNEMONIC: &str = "strong pyramid worth tennis option wet broccoli smoke midnight maze hint soft hen ignore shuffle multiply room recycle hurt degree crouch drill economy surge";
pub(crate) const SIGNER1_MNEMONIC: &str = "shed crumble dismiss loyal latin million oblige gesture shrug still oxygen custom remove ribbon disorder palace addict again blanket sad flock consider obey popular";

// Helper functions

pub(crate) async fn query_account(address: &str) -> RawRpcAccountStatus {
    let account_details = query_account_details(API_URL.to_owned(), address.to_owned())
        .await
        .unwrap()
        .into_serde::<RawRpcAccountResponse>()
        .unwrap();

    match account_details {
        RawRpcAccountResponse::OkResponse { account } => account,
        _ => panic!("Failed to query account details"),
    }
}

pub(crate) async fn query_balance(address: &str) -> RawRpcBalance {
    query_account_balance(API_URL.to_owned(), address.to_owned(), DENOM.to_owned(), 1)
        .await
        .unwrap()
        .into_serde::<RawRpcBalance>()
        .unwrap()
}
