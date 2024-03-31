use std::{fs, path};

use crate::models::line_item::LineItem;

pub fn create_order(
    order_id: &i32,
    items: Vec<LineItem>,
    connection: oracle::Connection,
) -> Result<i32, Box<dyn std::error::Error>> {
    // Get Store ID
    let store_id = get_correct_store_id(&items, &connection)?;

    // CREATE MASTER RECORD
    create_master_record(&connection, &store_id, &order_id)?;
    println!("Master Record Created");

    // GET TRANS NUMBER
    let open_trans_no: i32 = get_last_master_record_id(&connection, &order_id)?;
    println!("Open Trans Number: {}", open_trans_no);

    // PERPARE DETAIL INSERT
    create_detail_records(&connection, &items, store_id, open_trans_no)?;
    println!("Detail Records Created");
    Ok(open_trans_no)
}

pub fn create_master_record(
    connection: &oracle::Connection,
    store_id: &i32,
    order_id: &i32,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating Master Record");
    let sql = fs::read_to_string(path::Path::new("src/sql/create_master_record.sql"))?;
    let mut stmt = connection.statement(sql.as_str()).build()?;
    stmt.execute_named(&[("v_store_id", store_id), ("v_order_id", order_id)])?;
    connection.commit()?;
    Ok(())
}

pub fn get_last_master_record_id(
    connection: &oracle::Connection,
    order_id: &i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let order_id_string = order_id.to_string();
    let sql = fs::read_to_string(path::Path::new("src/sql/get_last_master_record_id.sql"))?;
    let mut stmt = connection.statement(sql.as_str()).build()?;
    let rows = stmt.query(&[&order_id_string])?;
    let mut result: i32 = 0;

    // Get first row only
    for row in rows {
        let r = row?;
        result = r.get(0)?;
        break;
    }

    Ok(result)
}

pub fn get_correct_store_id(
    items: &Vec<LineItem>,
    conn: &oracle::Connection,
) -> Result<i32, Box<dyn std::error::Error>> {
    println!("Getting Store ID");
    let sql = fs::read_to_string(path::Path::new("src/sql/get_store_quantity.sql"))?;
    let mut stores: Vec<(i32, i32, i32)> = Vec::new();
    for item in items {
        let mut stmt = conn.statement(sql.as_str()).build()?;
        let res = stmt.query(&[&item.sku])?;
        for i in res {
            if i.is_ok() {
                let row = i.unwrap();
                stores.push((item.quantity, row.get(0)?, row.get(1)?));
            }
        }
    }
    let mut use_store_2: bool = true;
    let mut use_store_8: bool = true;
    for item in stores {
        if item.1 >= item.0 && use_store_2 {
            use_store_2 = true;
        } else {
            use_store_2 = false;
        }
        if item.2 >= item.0 && use_store_8 {
            use_store_8 = true;
        } else {
            use_store_8 = false;
        }
    }
    if use_store_2 {
        println!("Using Store 2");
        Ok(2)
    } else if use_store_8 {
        println!("Using Store 8");
        Ok(8)
    } else {
        Err("Not enough quantity in any store".into())
    }
}

pub fn create_detail_records(
    connection: &oracle::Connection,
    products: &Vec<LineItem>,
    store_id: i32,
    open_trans_no: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating Detail Records");
    let sql = fs::read_to_string(path::Path::new("src/sql/create_detail_record.sql"))?;
    let mut row_order = 1;
    let mut total_discount: f64 = 0.0;
    for product in products {
        total_discount += product.price * 0.16 * product.quantity as f64;
        let mut stmt = connection.statement(sql.as_str()).build()?;
        stmt.execute_named(&[
            ("v_open_trans_no", &open_trans_no),
            ("v_serial_id", &row_order),
            ("v_store_id", &store_id),
            ("v_trans_quantity", &product.quantity),
            ("v_trans_amount_curr", &(product.price)),
            ("v_trans_amount_base", &product.price),
            (
                "v_tax_amount",
                &(product.price * 0.16 * product.quantity as f64),
            ),
            (
                "v_price_notax",
                &(product.price / 1.16 * product.quantity as f64),
            ),
            ("v_price_wtax", &(product.price * product.quantity as f64)),
            (
                "v_tax_amount_base",
                &(product.price * 0.16 * product.quantity as f64),
            ),
            (
                "v_price_notax_base",
                &(product.price / 1.16 * product.quantity as f64),
            ),
            (
                "v_price_wtax_base",
                &(product.price * product.quantity as f64),
            ),
            // ("v_item_barcode", &product.barcode),
            ("v_item_sku", &product.sku),
            ("v_disc_per", &0),
            (
                "v_mas_discount_amount",
                &(product.price * 0.16 * product.quantity as f64),
            ),
            (
                "v_mas_discount_amount_base",
                &(product.price * 0.16 * product.quantity as f64),
            ),
            ("v_first_disc_amt_base", &0),
            ("v_second_disc_amt_base", &0),
            ("v_base_unit_price_notax", &(product.price)),
            ("v_row_order", &row_order),
        ])?;
        row_order += 1;
    }

    // Update Total Discount
    println!("Updating Total Discount");
    let sql = fs::read_to_string(path::Path::new("src/sql/update_total_discount.sql"))?;
    let mut stmt_2 = connection.statement(sql.as_str()).build()?;
    stmt_2.execute_named(&[
        ("v_total_discount", &total_discount),
        ("v_open_trans_no", &open_trans_no),
    ])?;
    connection.commit()?;
    Ok(())
}
