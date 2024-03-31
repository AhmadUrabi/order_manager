use actix_web::{get, post, HttpResponse, Responder};

use crate::models::line_item::LineItem;

#[post("/order")]
pub async fn create_order(
    req_body: Option<String>,
    pool: actix_web::web::Data<super::OracleClientState>,
) -> impl Responder {
    if req_body.is_some() {
        let body = req_body.unwrap();
        // println!("{}", body);        
        let order: serde_json::Value = serde_json::from_str(&body).unwrap();
        let line_items = order["line_items"].as_array().unwrap();
        let mut line_items_vec: Vec<LineItem> = Vec::new();
        let order_id = order["order_number"].as_i64().unwrap() as i32;
        for item in line_items {
            let line_item = LineItem {
                id: item["id"].as_i64().unwrap() as i32,
                name: item["name"].as_str().unwrap().to_string(),
                sku: item["sku"].as_str().unwrap().to_string(),
                // barcode: item["barcode"].as_str().unwrap().to_string(),
                quantity: item["quantity"].as_i64().unwrap() as i32,
                price: item["price"].as_str().unwrap().parse::<f64>().unwrap(),
                discount: item["total_discount"].as_str().unwrap().parse::<f64>().unwrap(),
            };
            line_items_vec.push(line_item);
        }
        let pool = pool.pool.lock().await;
        let connection = pool.get_conn();
        match crate::order_manager::create_order(&order_id, line_items_vec, connection) {
            Ok(trans) => println!("Order Created, open_trans_no: {}", trans),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    HttpResponse::Ok().body("create_order")
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Order Creation Server Running")
}