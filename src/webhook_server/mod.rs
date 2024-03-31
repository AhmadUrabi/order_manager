use std::env;

use actix_web::{App, HttpServer };
use hmac::Mac;
use tokio::sync::Mutex;

use crate::oracle_client::OraclePool;
use base64::prelude::*;

pub mod routes;

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
                .service(routes::create_order)
                .service(routes::index)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }
}


pub fn verify_webhook(header: &str, body: &str) -> Result<bool, Box<dyn std::error::Error>>{
    let secret = env::var("WEBHOOK_SECRET").unwrap();
    let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(secret.as_bytes())?;
    mac.update(body.as_bytes());
    let calculated_hmac = BASE64_STANDARD.encode(mac.finalize().into_bytes().as_slice());
    Ok(calculated_hmac == header)
}