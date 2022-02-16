use std::error::Error;

use defi_wallet_connect::{Client, Metadata, WCMiddleware};
use ethers::prelude::Middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::new(Metadata {
        description: "Defi WalletConnect example.".into(),
        url: "http://localhost:8080/".parse().expect("url"),
        icons: vec![],
        name: "Defi WalletConnect Web3 Example".into(),
    })
    .await?;
    let (address, chain_id) = client.ensure_session().await?;
    println!("address: {:?}", address);
    println!("chain_id: {}", chain_id);
    let sig1 = client.personal_sign("hello", &address[0]).await?;
    println!("sig1: {:?}", sig1);
    let middleware = WCMiddleware::new(client);
    // Note that `sign` on ethers middleware translate to `eth_sign` JSON-RPC method
    // which in Metamask docs is marked as "(insecure and unadvised to use)"
    // and some wallets may reject it
    let sig2 = middleware
        .sign("world".as_bytes().to_vec(), &address[0])
        .await?;
    println!("sig2: {:?}", sig2);
    Ok(())
}
