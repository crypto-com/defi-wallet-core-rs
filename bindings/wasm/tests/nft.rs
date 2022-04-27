//! Test suite for nft messages.

#![cfg(target_arch = "wasm32")]

mod test_helper;

use defi_wallet_core_proto::chainmain::nft::v1::{BaseNft, Collection, Denom, Owner};
use defi_wallet_core_wasm::{CosmosClient, CosmosMsg, CosmosTx, PrivateKey};
use tendermint_rpc::endpoint::broadcast::tx_sync::Response;
use test_helper::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const TEST_DENOM_SCHEMA: &str = r#"
    {
        "title": "Asset Metadata",
        "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "description": "testidentity"
            },
            "description": {
                "type": "string",
                "description": "testdescription"
            },
            "image": {
                "type": "string",
                "description": "testdescription"
            }
        }
    }"#;

const TEST_TOKEN_ID: &str = "testtokenid";

#[wasm_bindgen_test]
async fn test_nft() {
    let denom1 = test_denom1();
    let denom2 = test_denom2();
    let mut client1 = NftClient::new(SIGNER1, SIGNER1_MNEMONIC);
    let mut client2 = NftClient::new(SIGNER2, SIGNER2_MNEMONIC);
    let mut grpc_client = grpc_web_client();

    // Issue denom1 and denom2 by signer1.
    client1.add_issue_msg(&denom1);
    client1.add_issue_msg(&denom2);
    client1.broadcast_tx().await;
    validate_issue().await;

    // Mint denom1 by signer2.
    client1.add_mint_msg(TEST_TOKEN_ID, &denom1.id, "", "testuri", "", SIGNER2);
    client1.broadcast_tx().await;
    validate_mint().await;

    // Transfer denom1 from signer2 to signer1.
    validate_before_transfer().await;
    client2.add_transfer_msg(TEST_TOKEN_ID, &denom1.id, SIGNER1);
    client2.broadcast_tx().await;
    validate_after_transfer().await;

    // Edit denom1 by signer1.
    client1.add_edit_msg(TEST_TOKEN_ID, &denom1.id, "newname", "newuri", "newdata");
    client1.broadcast_tx().await;
    validate_edit().await;

    // Burn denom1 by signer1.
    validate_before_burn().await;
    client1.add_burn_msg(TEST_TOKEN_ID, &denom1.id);
    client1.broadcast_tx().await;
    wait_for_timeout(None).await;
    validate_after_burn().await;
}

struct NftClient {
    address: String,
    private_key: PrivateKey,
    client: CosmosClient,
    tx: CosmosTx,
}

impl NftClient {
    pub fn new(address: &str, mnemonic: &str) -> Self {
        Self {
            address: address.to_owned(),
            private_key: get_private_key(mnemonic),
            client: chainmain_client(),
            tx: CosmosTx::new(),
        }
    }

    fn add_burn_msg(&mut self, id: &str, denom_id: &str) {
        self.tx.add_msg(CosmosMsg::build_nft_burn_msg(
            id.to_owned(),
            denom_id.to_owned(),
        ));
    }

    fn add_edit_msg(&mut self, id: &str, denom_id: &str, name: &str, uri: &str, data: &str) {
        self.tx.add_msg(CosmosMsg::build_nft_edit_msg(
            id.to_owned(),
            denom_id.to_owned(),
            name.to_owned(),
            uri.to_owned(),
            data.to_owned(),
        ));
    }

    fn add_issue_msg(&mut self, denom: &Denom) {
        self.tx.add_msg(CosmosMsg::build_nft_issue_denom_msg(
            denom.id.to_owned(),
            denom.name.to_owned(),
            denom.schema.to_owned(),
        ));
    }

    fn add_mint_msg(
        &mut self,
        id: &str,
        denom_id: &str,
        name: &str,
        uri: &str,
        data: &str,
        recipient: &str,
    ) {
        self.tx.add_msg(CosmosMsg::build_nft_mint_msg(
            id.to_owned(),
            denom_id.to_owned(),
            name.to_owned(),
            uri.to_owned(),
            data.to_owned(),
            recipient.to_owned(),
        ));
    }

    fn add_transfer_msg(&mut self, id: &str, denom_id: &str, recipient: &str) {
        self.tx.add_msg(CosmosMsg::build_nft_transfer_msg(
            id.to_owned(),
            denom_id.to_owned(),
            recipient.to_owned(),
        ));
    }

    async fn broadcast_tx(&mut self) {
        let signed_data = self
            .tx
            .sign_into(
                self.private_key.clone(),
                chainmain_tx_info(&self.address).await,
            )
            .unwrap();

        let res = JsFuture::from(self.client.broadcast_tx(signed_data))
            .await
            .unwrap()
            .into_serde::<Response>()
            .unwrap();

        assert_eq!(res.code, tendermint::abci::Code::Ok);
        wait_for_timeout(None).await;
    }
}

fn test_denom1() -> Denom {
    Denom {
        id: "testdenomid".to_owned(),
        name: "testdenomname".to_owned(),
        schema: TEST_DENOM_SCHEMA.to_owned(),
        creator: SIGNER1.to_owned(),
    }
}

fn test_denom2() -> Denom {
    Denom {
        id: "testdenomid2".to_owned(),
        name: "testdenomname2".to_owned(),
        schema: TEST_DENOM_SCHEMA.to_owned(),
        creator: SIGNER1.to_owned(),
    }
}

async fn validate_issue() {
    let denom1 = test_denom1();
    let denom2 = test_denom2();
    let mut grpc_client = grpc_web_client();

    let res = grpc_client
        .supply(denom1.id.clone(), SIGNER1.to_owned())
        .await;
    assert!(res.is_ok());
    let supply = res.unwrap().into_serde::<u64>().unwrap();
    assert_eq!(supply, 0);

    let res = grpc_client
        .owner(denom1.id.clone(), SIGNER1.to_owned())
        .await;
    assert!(res.is_ok());
    let owner = res.unwrap().into_serde::<Owner>().unwrap();
    assert_eq!(owner.address, SIGNER1.to_owned());
    assert_eq!(owner.id_collections, vec![]);

    let res = grpc_client.collection(denom2.id.clone()).await;
    assert!(res.is_ok());
    let collection = res.unwrap().into_serde::<Collection>().unwrap();
    assert_eq!(collection.denom, Some(denom2.clone()));
    assert_eq!(collection.nfts.len(), 0);

    let res = grpc_client.denom(denom2.id.clone()).await;
    assert!(res.is_ok());
    let denom = res.unwrap().into_serde::<Denom>().unwrap();
    assert_eq!(denom, denom2);

    let res = grpc_client.denom_by_name(denom1.name.clone()).await;
    assert!(res.is_ok());
    let denom = res.unwrap().into_serde::<Denom>().unwrap();
    assert_eq!(denom, denom1);

    let res = grpc_client.denoms().await;
    assert!(res.is_ok());
    let denoms = res.unwrap().into_serde::<Vec<Denom>>().unwrap();
    assert_eq!(denoms.len(), 2);
    assert_eq!(denoms[0], denom1);
    assert_eq!(denoms[1], denom2);
}

async fn validate_mint() {
    let denom1 = test_denom1();
    let mut grpc_client = grpc_web_client();

    // Check nft after minting.
    let res = grpc_client
        .nft(denom1.id.clone(), TEST_TOKEN_ID.to_owned())
        .await;
    assert!(res.is_ok());
    let nft = res.unwrap().into_serde::<BaseNft>().unwrap();
    assert_eq!(nft.owner, SIGNER2.to_owned());

    // Check collection after minting.
    let res = grpc_client.collection(denom1.id.clone()).await;
    assert!(res.is_ok());
    let collection = res.unwrap().into_serde::<Collection>().unwrap();
    assert_eq!(collection.denom, Some(denom1));
    assert_eq!(collection.nfts.len(), 1);
}

async fn validate_before_transfer() {
    let denom1 = test_denom1();
    let mut grpc_client = grpc_web_client();

    // Check owner.
    let res = grpc_client
        .owner(denom1.clone().id, SIGNER2.to_owned())
        .await;
    assert!(res.is_ok());
    let owner = res.unwrap().into_serde::<Owner>().unwrap();
    assert_eq!(owner.address, SIGNER2.to_owned());
    assert_eq!(owner.id_collections.len(), 1);
    assert_eq!(owner.id_collections[0].denom_id, denom1.clone().id);
    assert_eq!(owner.id_collections[0].token_ids.len(), 1);
    assert_eq!(
        owner.id_collections[0].token_ids[0],
        TEST_TOKEN_ID.to_owned()
    );
}

async fn validate_after_transfer() {
    let denom1 = test_denom1();
    let mut grpc_client = grpc_web_client();

    let res = grpc_client
        .nft(denom1.clone().id, TEST_TOKEN_ID.to_owned())
        .await;
    assert!(res.is_ok());
    let nft = res.unwrap().into_serde::<BaseNft>().unwrap();
    assert_eq!(nft.owner, SIGNER1.to_owned());

    // Check owner.
    let res = grpc_client
        .owner(denom1.clone().id, SIGNER2.to_owned())
        .await;
    assert!(res.is_ok());
    let owner = res.unwrap().into_serde::<Owner>().unwrap();
    assert_eq!(owner.address, SIGNER2.to_owned());
    assert_eq!(owner.id_collections, vec![]);
}

async fn validate_edit() {
    let denom1 = test_denom1();
    let mut grpc_client = grpc_web_client();

    let res = grpc_client
        .nft(denom1.id.clone(), TEST_TOKEN_ID.to_owned())
        .await;
    assert!(res.is_ok());
    let nft = res.unwrap().into_serde::<BaseNft>().unwrap();
    assert_eq!(nft.name, "newname".to_owned());
    assert_eq!(nft.uri, "newuri".to_owned());
    assert_eq!(nft.data, "newdata".to_owned());
}

async fn validate_before_burn() {
    let denom1 = test_denom1();
    let mut grpc_client = grpc_web_client();

    // Check supply.
    let res = grpc_client
        .supply(denom1.id.clone(), SIGNER1.to_owned())
        .await;
    assert!(res.is_ok());
    let supply = res.unwrap().into_serde::<u64>().unwrap();
    assert_eq!(supply, 1);
}

async fn validate_after_burn() {
    let denom1 = test_denom1();
    let mut grpc_client = grpc_web_client();

    let res = grpc_client
        .nft(denom1.clone().id, TEST_TOKEN_ID.to_owned())
        .await;
    assert!(res.is_err());

    // Check supply.
    let res = grpc_client
        .supply(denom1.clone().id, SIGNER1.to_owned())
        .await;
    assert!(res.is_ok());
    let supply = res.unwrap().into_serde::<u64>().unwrap();
    assert_eq!(supply, 0);

    // Check collection.
    let res = grpc_client.collection(denom1.clone().id).await;
    assert!(res.is_ok());
    let collection = res.unwrap().into_serde::<Collection>().unwrap();
    assert_eq!(collection.denom, Some(denom1));
    assert_eq!(collection.nfts.len(), 0);
}
