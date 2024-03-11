use alloy_core::primitives::bytes;
use alloy_node_bindings::Anvil;
use alloy_providers::tmp::{Provider, TempProvider};
use eyre::Result;
#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().spawn();
    let provider = Provider::try_from(anvil.endpoint())?; // Uses anvil

    // Transfer 1ETH from 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266 to Address::ZERO
    let tx = provider.send_raw_transaction(bytes!("f865808477359400825208940000000000000000000000000000000000000000018082f4f5a00505e227c1c636c76fac55795db1a40a4d24840d81b40d2fe0cc85767f6bd202a01e91b437099a8a90234ac5af3cb7ca4fb1432e133f75f9a91678eaf5f487c74b")).await?;

    println!("Transaction hash: {}", tx);

    let transaction = provider.get_transaction_by_hash(tx).await?;

    println!("Transaction: {:?}", transaction);

    Ok(())
}
