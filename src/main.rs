pub mod models;
pub mod oracle_client;
pub mod order_manager;
pub mod webhook_server;
use std::error::Error;

use dotenv::dotenv;
use models::line_item::LineItem;
use order_manager::create_order;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let pool = oracle_client::OraclePool::new();
    // let _ = webhook_server::WebServer::start(pool).await;
    let r = create_order(Vec::<LineItem>::new(), pool.get_conn());
    match r {
        Ok(_) => println!("Order Created"),
        Err(e) => println!("Error: {:?}", e),
    }
    Ok(())
}
