use crate::models::line_item::LineItem;

pub fn create_order(
    order_id: &i32,
    items: Vec<LineItem>,
    connection: oracle::Connection,
) -> Result<(), Box<dyn std::error::Error>> {

    println!("{}", order_id);

    // Get Store ID
    let store_id = get_correct_store_id(&items, &connection);
    if store_id.is_err() {
        return Err("Not enough quantity in any store".into());
    }
    let store_id = store_id.unwrap();

    // CREATE MASTER RECORD
    create_master_record(&connection, &store_id, &order_id)?;
    println!("Master Record Created");

    // GET TRANS NUMBER
    let open_trans_no: i32 = get_last_master_record_id(&connection, &order_id)?;
    println!("Open Trans Number: {}", open_trans_no);
    // PERPARE DETAIL INSERT

    create_detail_records(&connection, &items, store_id, open_trans_no)?;

    Ok(())
}

pub fn create_master_record(
    connection: &oracle::Connection,
    store_id: &i32,
    order_id: &i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let sql = r#"DECLARE
    v_open_trans_no number;
    v_year_trans_no number;
    v_period_trans_no number;
    BEGIN
        inv_pkg.get_max_trans_no(2, 13, 2, extract(year from sysdate), extract(month from sysdate), v_open_trans_no, v_year_trans_no, v_period_trans_no);
        v_year_trans_no := v_year_trans_no + (100000 * 1) + (1000000 * :v_store_id);
        v_period_trans_no := v_period_trans_no + (100000 * 1) + (1000000 * :v_store_id);
        -- Insert Master Record

        INSERT INTO jhc.INV_STORE_TRANS_MAS(
        COMPANY_ID,
        TRANS_ID,
        OPEN_TRANS_NO,
        STORE_ID,
        YEAR_CODE,
        PERIOD_CODE,
        YEAR_TRANS_NO,
        PERIOD_TRANS_NO,
        TRANS_STATUS,
        TRANS_DATE,
        ACC_ARAP,
        PERSON_ID,
        ACC_ID,
        CURRENCY_ID,
        EXCHANGE_BASE_FACTOR,
        REFERENCE_NO,
        REFERENCE_DATE,
        REFERENCE_ID,
        IS_AUTO,
        FROM_SYSTEM,
        PERSON_NAME,
        DEFAULT_SALE_ID,
        VOUCHER_ID,
        OPEN_VOUCHER_NO,
        USER_CREATE,
        PC_NAME_CREATE,
        DATE_CREATED,
        CASH_DEBIT,
        DISCOUNT_TYPE,
        DISCOUNT_PERCENT,
        IS_PRINTED,
        HAS_SALE_TAX,
        PRICE_INCLUDES_TAX,
        LIST_ID,
        ADDED_VALUE_CALCED,
        IS_COSTED,
        TERM_ID,
        HAS_REFUND_TRANS
        )
        VALUES (
        2,
        13,
        v_open_trans_no,
        :v_store_id,
        extract(year from sysdate),
        extract(month from sysdate),
        v_year_trans_no,
        v_period_trans_no,
        1,
        TO_CHAR(SYSDATE, 'DD-Mon-YY'),
        2,
        1100,
        18,
        1,
        1,
        :v_order_id,
        TO_CHAR(SYSDATE, 'DD-Mon-YY'),
        14,
        0,
        6,
        'مبيعات اونلاين',
        1100,
        null,
        null,
        10,
        'Automatic',
        TO_CHAR(SYSDATE, 'DD-Mon-YY'),
        1,
        1,
        13.793103,
        0,
        1,
        0,
        1,
        0,
        0,
        62,
        0
        );
    END;
    "#;
    let mut stmt = connection.statement(sql).build()?;
    stmt.execute_named(&[("v_store_id", store_id), ("v_order_id", order_id)])?;

    connection.commit()?;
    Ok(())
}

pub fn get_last_master_record_id(
    connection: &oracle::Connection,
    order_id: &i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let order_id_string = order_id.to_string();
    let sql = r#"SELECT OPEN_TRANS_NO from jhc.INV_STORE_TRANS_MAS where REFERENCE_NO = :1"#;
    let mut stmt = connection.statement(sql).build()?;
    let rows = stmt.query(&[&order_id_string])?;
    let mut result: i32 = 0;
    for row in rows {
        let row = row?;
        result = row.get(0)?;
    }
    Ok(result)
}

pub fn get_correct_store_id(
    items: &Vec<LineItem>,
    conn: &oracle::Connection,
) -> Result<i32, Box<dyn std::error::Error>> {
    println!("Getting Store ID");
    let sql = r#"SELECT QTY_STORE_02, QTY_STORE_08 FROM ODBC_JHC.JHC_INVDATA WHERE FOREIGN_ITEM_CODE = :1"#;
    let mut stores: Vec<(i32, i32, i32)> = Vec::new();
    for item in items {
        let mut stmt = conn.statement(sql).build()?;
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
        println!("Store 2");
        Ok(2)
    } else if use_store_8 {
        println!("Store 8");
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
    let sql = r#"
    DECLARE
    v_id number;
    v_item_barcode number;
    BEGIN
        SELECT ITEM_ID INTO v_id FROM ODBC_JHC.JHC_INVDATA ji WHERE FOREIGN_ITEM_CODE = :v_item_sku;
        SELECT ITEM_MAIN_BARCODE INTO v_item_barcode FROM ODBC_JHC.JHC_INVDATA ji WHERE FOREIGN_ITEM_CODE = :v_item_sku;
        INSERT INTO jhc.INV_STORE_TRANS_DTL(
        COMPANY_ID,
        TRANS_ID,
        OPEN_TRANS_NO,
        SERIAL_ID,
        STORE_ID,
        ITEM_ID,
        TRANS_UNIT,
        TRANS_QUANT,
        BASE_UNIT,
        TRANS_BASE_QUANT,
        TRANS_AMOUNT_CURR,
        TRANS_AMOUNT_BASE,
        EXCHANGE_BASE_FACTOR,
        CURRENCY_ID,
        IS_POSTED,
        USER_CREATE,
        PC_NAME_CREATE,
        DATE_CREATED,
        UNIT_EXCHANGE_FACTOR,
        ITEM_CODE,
        DTL_TRANS_STATUS,
        DTL_TRANS_DATE,
        TAX_ID,
        TAX_PERCENT_AMOUNT,
        TAX_VALUE,
        TAX_AMOUNT,
        PRICE_NOTAX,
        ORG_PRICE,
        PRICE_WTAX,
        COST_PRICE,
        TAX_AMOUNT_BASE,
        PRICE_NOTAX_BASE,
        PRICE_WTAX_BASE,
        BASE_ITEM_BARCODE,
        FOREIGN_ITEM_CODE,
        FIRST_DISC_PER,
        SECOND_DISC_PER,
        SECOND_DISC_AMT,
        REFUND_BASE_QUANT,
        TRANS_PACK_COUNT,
        INT_NO_PACKS,
        REMAIN_NO_PACKS,
        DISCOUNT_AMOUNT_BASE,
        MAS_DISCOUNT_AMOUNT,
        MAS_DISCOUNT_AMOUNT_BASE,
        FIRST_DISC_AMT_BASE,
        SECOND_DISC_AMT_BASE,
        BASE_UNIT_PRICE_NOTAX,
        REFUND_UNIT_EXCHANGE,
        ROW_ORDER)
        VALUES (2,
        13,
        :v_open_trans_no,
        :v_serial_id,
        :v_store_id,
        v_id,
        1,
        :v_trans_quantity,
        1,
        1,
        :v_trans_amount_curr,
        :v_trans_amount_base,
        1,
        1,
        0,
        22,
        'Automatic',
        TO_CHAR(SYSDATE,
        'DD-Mon-YY'),
        1,
        v_id,
        3,
        TO_CHAR(SYSDATE,
        'DD-Mon-YY'),
        16,
        1,
        16,
        :v_tax_amount,
        :v_price_notax,
        0,
        :v_price_wtax,
        0,
        :v_tax_amount_base,
        :v_price_notax_base,
        :v_price_wtax_base,
        v_item_barcode,
        :v_item_sku,
        :v_disc_per,
        0,
        0,
        0,
        24,
        0,
        1,
        0,
        :v_mas_discount_amount,
        :v_mas_discount_amount_base,
        :v_first_disc_amt_base,
        :v_second_disc_amt_base,
        :v_base_unit_price_notax,
        1,
        :v_row_order
        );
    END;
    "#;
    let mut row_order = 1;
    let mut total_discount: f64 = 0.0;
    for product in products {
        total_discount += product.price * 0.16 * product.quantity as f64;
        let mut stmt = connection.statement(sql).build()?;
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
            ("v_tax_amount_base", &(product.price * 0.16 * product.quantity as f64)),
            ("v_price_notax_base", &(product.price / 1.16 * product.quantity as f64)),
            ("v_price_wtax_base", &(product.price * product.quantity as f64)),
            // ("v_item_barcode", &product.barcode),
            ("v_item_sku", &product.sku),
            ("v_disc_per", &0),
            (
                "v_mas_discount_amount",
                &(product.price * 0.16 * product.quantity as f64),
            ),
            ("v_mas_discount_amount_base", &(product.price * 0.16 * product.quantity as f64)),
            ("v_first_disc_amt_base", &0),
            ("v_second_disc_amt_base", &0),
            ("v_base_unit_price_notax", &(product.price)),
            ("v_row_order", &row_order),
        ])?;
        row_order += 1;
    }

    let sql_2 = r#"
    UPDATE jhc.INV_STORE_TRANS_MAS SET DISCOUNT_AMOUNT = :v_total_discount WHERE OPEN_TRANS_NO = :v_open_trans_no AND TRANS_ID = 13"#;
    let mut stmt_2 = connection.statement(sql_2).build()?;
    stmt_2.execute_named(&[
        ("v_total_discount", &total_discount),
        ("v_open_trans_no", &open_trans_no),
    ])?;
    connection.commit()?;
    Ok(())
}
