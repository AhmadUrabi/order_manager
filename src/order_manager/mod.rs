use std::result;

use oracle::sql_type::ToSql;

use crate::models::line_item::LineItem;
pub fn create_order(items: Vec<LineItem>, connection: oracle::Connection) -> Result<(), Box<dyn std::error::Error>>{
    let sql = r#"DECLARE
    v_open_trans_no number;
    v_year_trans_no number;
    v_period_trans_no number;
BEGIN
    inv_pkg.get_max_trans_no(2, 13, 2, extract(year from sysdate), extract(month from sysdate), v_open_trans_no, v_year_trans_no, v_period_trans_no);

    -- Insert Master Record

    INSERT INTO INV_STORE_TRANS_MAS(COMPANY_ID,TRANS_ID,OPEN_TRANS_NO,STORE_ID,YEAR_CODE,PERIOD_CODE,YEAR_TRANS_NO,PERIOD_TRANS_NO,TRANS_STATUS,TRANS_DATE,ACC_ARAP,PERSON_ID,ACC_ID,CURRENCY_ID,EXCHANGE_BASE_FACTOR,REFERENCE_NO,REFERENCE_DATE,REFERENCE_ID,IS_AUTO,FROM_SYSTEM,PERSON_NAME,DEFAULT_SALE_ID,USER_CREATE,PC_NAME_CREATE, CASH_DEBIT, DISCOUNT_TYPE, DISCOUNT_PERCENT, IS_PRINTED,HAS_SALE_TAX,PRICE_INCLUDES_TAX, LIST_ID, ADDED_VALUE_CALCED, IS_COSTED, TERM_ID, HAS_REFUND_TRANS)
    VALUES (2, 13, v_open_trans_no, 2, extract(year from sysdate), extract(month from sysdate), v_year_trans_no, v_period_trans_no, 1, sysdate,2,1100,18,1,1,3071,sysdate,14,0,6,'مبيعات اونلاين',1100,10,'Automatic',1,1,13.793103,0,1,0,1,0,0,62,0);
END;
    "#;
    let mut stmt = connection.statement(sql).build()?;
    let res = stmt.execute(&[])?;
    
    



    // INV Check

    // GET TRANS NUMBER

    // PREPARE MASTER INSERT

    // PERPARE DETAIL INSERT

    Ok(())
}

