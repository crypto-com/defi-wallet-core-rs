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

    let middleware = WCMiddleware::new(client);

    let sig = middleware
        .sign("hello".as_bytes().to_vec(), &address[0])
        .await?;
    println!("sig: {:?}", sig);
    Ok(())
}
