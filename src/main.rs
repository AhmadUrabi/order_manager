pub mod models;
pub mod oracle_client;
pub mod order_manager;
pub mod webhook_server;
use std::error::Error;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let pool = oracle_client::OraclePool::new();
    let _ = webhook_server::WebServer::start(pool).await;
    Ok(())
}
