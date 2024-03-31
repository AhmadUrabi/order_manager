use actix_web::{App, HttpServer };
use tokio::sync::Mutex;

use crate::oracle_client::OraclePool;

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
