DECLARE
    V_OPEN_TRANS_NO   NUMBER;
    V_YEAR_TRANS_NO   NUMBER;
    V_PERIOD_TRANS_NO NUMBER;
BEGIN
    INV_PKG.GET_MAX_TRANS_NO(2, 13, 2, EXTRACT(YEAR FROM SYSDATE), EXTRACT(MONTH FROM SYSDATE), V_OPEN_TRANS_NO, V_YEAR_TRANS_NO, V_PERIOD_TRANS_NO);
    V_YEAR_TRANS_NO := V_YEAR_TRANS_NO + (100000 * 1) + (1000000 * :V_STORE_ID);
    V_PERIOD_TRANS_NO := V_PERIOD_TRANS_NO + (100000 * 1) + (1000000 * :V_STORE_ID);
 -- Insert Master Record
    INSERT INTO JHC.INV_STORE_TRANS_MAS(
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
    ) VALUES (
        2,
        13,
        V_OPEN_TRANS_NO,
        :V_STORE_ID,
        EXTRACT(YEAR FROM SYSDATE),
        EXTRACT(MONTH FROM SYSDATE),
        V_YEAR_TRANS_NO,
        V_PERIOD_TRANS_NO,
        1,
        TO_CHAR(SYSDATE, 'DD-Mon-YY'),
        2,
        1100,
        18,
        1,
        1,
        :V_ORDER_ID,
        TO_CHAR(SYSDATE, 'DD-Mon-YY'),
        14,
        0,
        6,
        'مبيعات اونلاين',
        1100,
        NULL,
        NULL,
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