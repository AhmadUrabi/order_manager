use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use tokio::sync::Mutex;

use crate::oracle_client::OraclePool;

// use crate::models::order::Order;

#[post("/order")]
async fn create_order(
    req_body: Option<String>,
    pool: actix_web::web::Data<OracleClientState>,
) -> impl Responder {
    if req_body.is_some() {
        // Deserialize JSON with no type to extract line_items array and print name field
        let order: serde_json::Value = serde_json::from_str(&req_body.unwrap()).unwrap();
        let line_items = order["line_items"].as_array().unwrap();
        for item in line_items {
            println!("{}", item["sku"].as_str().unwrap());
        }
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
