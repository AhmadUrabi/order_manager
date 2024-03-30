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
    let _ = webhook_server::WebServer::start(pool).await;

    let products: Vec<LineItem> = vec![
        LineItem {id:1, name: "test1".to_string(), sku: "2116116".to_string(), quantity: 5, price: 13.95, discount: 0.0 },
        LineItem {id:1, name: "test1".to_string(), sku: "2116114".to_string(), quantity: 1, price: 13.95, discount: 0.0 },
    ];

    // let r = create_order(products, pool.get_conn());
    // match r {
    //     Ok(_) => println!("Order Created"),
    //     Err(e) => println!("Error: {:?}", e),
    // }
    Ok(())
}
