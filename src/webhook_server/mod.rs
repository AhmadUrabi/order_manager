use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use tokio::sync::Mutex;

use crate::{models::line_item::LineItem, oracle_client::OraclePool};

// use crate::models::order::Order;

#[post("/order")]
async fn create_order(
    req_body: Option<String>,
    pool: actix_web::web::Data<OracleClientState>,
) -> impl Responder {
    if req_body.is_some() {
        let body = req_body.unwrap();
        println!("{}", body);        
        let order: serde_json::Value = serde_json::from_str(&body).unwrap();
        let line_items = order["line_items"].as_array().unwrap();
        let mut line_items_vec: Vec<LineItem> = Vec::new();
        for item in line_items {
            let line_item = LineItem {
                id: item["id"].as_i64().unwrap() as i32,
                name: item["name"].as_i64().unwrap() as i32,
                sku: item["sku"].as_str().unwrap().to_string(),
                barcode: item["barcode"].as_str().unwrap().to_string(),
                quantity: item["quantity"].as_i64().unwrap() as i32,
                price: item["price"].as_f64().unwrap(),
                discount: item["discount"].as_f64().unwrap(),
            };
            line_items_vec.push(line_item);
        }
        let pool = pool.pool.lock().await;
        let connection = pool.get_conn();
        crate::order_manager::create_order(line_items_vec, connection).unwrap();
    }
    HttpResponse::Ok().body("create_order")
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Order Creation Server Running")
}

pub struct WebServer;

struct OracleClientState {
    pool: Mutex<OraclePool>,
}

impl WebServer {
    pub async fn start(pool: OraclePool) -> std::io::Result<()> {
        let pool_state = actix_web::web::Data::new(OracleClientState {
            pool: Mutex::new(pool),
        });
        HttpServer::new(move || {
            App::new()
                .app_data(pool_state.clone())
                .service(create_order)
                .service(index)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }
}
