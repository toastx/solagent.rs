use std::sync::Arc;

use solagent::{NftMetadata, SolAgent};
use solana_sdk::pubkey::Pubkey;

#[tokio::main]
async fn main() {
    let name = "Solagent Collection";
    let uri = "https://arweave.net/metadata.json";
    let royalty_basis_points = Some(500);
    let creators = vec![(
        Pubkey::from_str_const("HHV3DX4UT4u3vBek2XCaZeAyox88zuhWfcLRJbFx1oYt"),
        100,
    )];
    let options = NftMetadata::new(name, uri, royalty_basis_points, Some(creators));

    let agent = Arc::new(SolAgent::new(
        "private_key",
        "https://api.devnet.solana.com",
        "openai_api_key",
    ));
    let tx = agent.deploy_collection(options).await.unwrap();
    println!("Mint: {:?}", tx.0);
}
