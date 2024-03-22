pub struct OrderManager {
    pool: OraclePool,
}

impl OrderManager {
    pub fn new(pool: OraclePool) -> Self {
        OrderManager { pool }
    }
    pub fn create_order(&self, req_body: Option<String>) {
        if req_body.is_some() {
            // Deserialize JSON with no type to extract line_items array and print name field
            let order: serde_json::Value = serde_json::from_str(&req_body.unwrap()).unwrap();
            let line_items = order["line_items"].as_array().unwrap();
            for item in line_items {
                println!("{}", item["sku"].as_str().unwrap());
            }
        }
    }
}
