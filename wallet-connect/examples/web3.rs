use defi_wallet_connect::{Client, Metadata, WCMiddleware};
use ethers::prelude::Middleware;

#[tokio::main]
async fn main() {
    let mut client = Client::new(Metadata {
        description: "Defi WalletConnect example.".into(),
        url: "http://localhost:8080/".parse().expect("url"),
        icons: vec![],
        name: "Defi WalletConnect Web3 Example".into(),
    })
    .await
    .expect("client");
    let (address, chain_id) = client.ensure_session().await.expect("ensure session");
    println!("address: {:?}", address);
    println!("chain_id: {}", chain_id);

    let middleware = WCMiddleware::new(client);

    let sig = middleware
        .sign("hello".as_bytes().to_vec(), &address[0])
        .await
        .expect("sign");
    println!("sig: {:?}", sig);
}
