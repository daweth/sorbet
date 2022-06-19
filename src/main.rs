use axum::{
    extract::Json,
    routing::{get, post},
    Extension, Router,
};
use escrow::{
    evm::{generate_local_client, send_transaction, EvmContext},
    TransactionReceipt, TransactionRequest,
};
use std::sync::Arc;

/*`/execute` handler */
async fn execute_tx(
    Extension(state): Extension<Arc<EvmContext>>,
    Json(payload): Json<TransactionRequest>,
) -> Json<TransactionReceipt> {
    let receipt = send_transaction(state.client.clone(), payload).await;
    Json(receipt.unwrap())
}

#[tokio::main]
async fn main() {
    let shared_avax_state = Arc::new(EvmContext {
        client: generate_local_client().await.unwrap(),
    });

    let app = Router::new()
        .route("/", get(|| async { "hello world! " }))
        .route("/execute", post(execute_tx))
        .layer(Extension(shared_avax_state));

    axum::Server::bind(&"0.0.0.0:6969".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
