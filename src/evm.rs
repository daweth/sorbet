use dotenv;
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{
    transaction::eip2718::TypedTransaction, NameOrAddress, TransactionReceipt, TransactionRequest,
};

/* Axum handler shared state */
pub struct EvmContext {
    pub client: SignerMiddleware<Provider<Http>, LocalWallet>,
}

pub async fn generate_local_signer() -> Option<LocalWallet> {
    //TODO: create a KMS wallet here for prod
    let wallet = match dotenv::var("PRIVATE_KEY").unwrap().parse::<LocalWallet>() {
        Ok(w) => w,
        Err(_) => return None,
    };
    // set chain ID to Avalanche Fuji
    let wallet = wallet.with_chain_id(43113u64);
    dbg!(&wallet);
    Some(wallet)
}

pub async fn generate_local_provider() -> Option<Provider<Http>> {
    // connect to the network
    let provider = match Provider::<Http>::try_from(dotenv::var("RPC_URL").unwrap()) {
        Ok(p) => p,
        Err(_) => return None,
    };
    Some(provider)
}

pub async fn generate_local_client(
) -> Result<SignerMiddleware<Provider<Http>, LocalWallet>, Box<dyn std::error::Error>> {
    let wallet = dotenv::var("PRIVATE_KEY")
        .unwrap()
        .parse::<LocalWallet>()?
        .with_chain_id(43113u64);
    let provider = Provider::<Http>::try_from(dotenv::var("RPC_URL").unwrap())?;
    let client = ethers::middleware::SignerMiddleware::new(provider, wallet);
    Ok(client)
}

// function has issues
pub async fn send_transaction<'a>(
    client: SignerMiddleware<Provider<Http>, LocalWallet>,
    mut tx: TransactionRequest,
) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
    let gas = client
        .estimate_gas(&TypedTransaction::Legacy(tx.clone()))
        .await?;

    let nonce = client
        .get_transaction_count(NameOrAddress::Address(client.address()), None)
        .await?;

    // Adjust gas accordingly to on-chain values.
    dbg!(&tx);
    tx.gas = Some(gas);
    tx.nonce = Some(nonce);
    dbg!(&tx);

    // send tx
    let pending_tx = client.send_transaction(tx, None).await?;
    dbg!(&pending_tx);

    // get the mined tx
    let receipt = pending_tx
        .await?
        .ok_or_else(|| eyre::format_err!("tx dropped from mempol!"))?;
    let _txn = client.get_transaction(receipt.transaction_hash).await?;

    println!("Tx receipt: {}", serde_json::to_string(&receipt).unwrap());
    Ok(receipt)
}
