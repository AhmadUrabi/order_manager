use chrono::DateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CurrentSubtotalPriceSet {
    shop_money: Money,
    presentment_money: Money,
}

#[derive(Debug, Deserialize)]
struct Money {
    amount: String,
    currency_code: String,
}

#[derive(Debug, Deserialize)]
struct CurrentTotalDiscountsSet {
    shop_money: Money,
    presentment_money: Money,
}

#[derive(Debug, Deserialize)]
struct CurrentTotalPriceSet {
    shop_money: Money,
    presentment_money: Money,
}

#[derive(Debug, Deserialize)]
struct CurrentTotalTaxSet {
    shop_money: Money,
    presentment_money: Money,
}

#[derive(Debug, Deserialize)]
struct DefaultAddress {
    id: i64,
    customer_id: i64,
    first_name: Option<String>,
    last_name: Option<String>,
    company: Option<String>,
    address1: String,
    address2: Option<String>,
    city: String,
    province: String,
    country: String,
    zip: String,
    phone: String,
    name: String,
    province_code: String,
    country_code: String,
    country_name: String,
    default: bool,
}

#[derive(Debug, Deserialize)]
struct EmailMarketingConsent {
    state: String,
    opt_in_level: Option<String>,
    consent_updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Customer {
    id: i64,
    email: String,
    created_at: Option<String>,
    updated_at: Option<String>,
    first_name: String,
    last_name: String,
    state: String,
    note: Option<String>,
    verified_email: bool,
    multipass_identifier: Option<String>,
    tax_exempt: bool,
    phone: Option<String>,
    email_marketing_consent: EmailMarketingConsent,
    sms_marketing_consent: Option<String>,
    tags: String,
    currency: String,
    tax_exemptions: Vec<String>,
    admin_graphql_api_id: String,
    default_address: DefaultAddress,
}

#[derive(Debug, Deserialize)]
struct LineItem {
    id: i64,
    admin_graphql_api_id: String,
    attributed_staffs: Vec<AttributedStaff>,
    current_quantity: i32,
    fulfillable_quantity: i32,
    fulfillment_service: String,
    fulfillment_status: Option<String>,
    gift_card: bool,
    grams: i32,
    name: String,
    price: String,
    price_set: Money,
    product_exists: bool,
    product_id: i64,
    properties: Vec<String>,
    quantity: i32,
    requires_shipping: bool,
    sku: String,
    taxable: bool,
    title: String,
    total_discount: String,
    total_discount_set: Money,
    variant_id: i64,
    variant_inventory_management: String,
    variant_title: Option<String>,
    vendor: Option<String>,
    tax_lines: Vec<String>,
    duties: Vec<String>,
    discount_allocations: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AttributedStaff {
    id: String,
    quantity: i32,
}

#[derive(Debug, Deserialize)]
struct ShippingAddress {
    first_name: String,
    address1: String,
    phone: String,
    city: String,
    zip: String,
    province: String,
    country: String,
    last_name: String,
    address2: Option<String>,
    company: String,
    latitude: Option<String>,
    longitude: Option<String>,
    name: String,
    country_code: String,
    province_code: String,
}

#[derive(Debug, Deserialize)]
struct ShippingLine {
    pub id: i64,
    pub carrier_identifier: Option<String>,
    pub code: Option<String>,
    pub discounted_price: String,
    pub discounted_price_set: Money,
    pub phone: Option<String>,
    pub price: String,
    pub price_set: Money,
    pub requested_fulfillment_service_id: Option<String>,
    pub source: String,
    pub title: String,
    pub tax_lines: Vec<String>,
    pub discount_allocations: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Order {
    pub id: Option<i64>,
    pub admin_graphql_api_id: Option<String>,
    pub app_id: Option<String>,
    pub browser_ip: Option<String>,
    pub buyer_accepts_marketing: Option<bool>,
    pub cancel_reason: Option<String>,
    pub cancelled_at: Option<String>,
    pub cart_token: Option<String>,
    pub checkout_id: Option<String>,
    pub checkout_token: Option<String>,
    pub client_details: Option<String>,
    pub closed_at: Option<String>,
    pub confirmation_number: Option<String>,
    pub confirmed: Option<bool>,
    pub contact_email: Option<String>,
    pub created_at: Option<String>,
    pub currency: Option<String>,
    pub current_subtotal_price: Option<String>,
    pub current_subtotal_price_set: Option<CurrentSubtotalPriceSet>,
    pub current_total_additional_fees_set: Option<String>,
    pub current_total_discounts: Option<String>,
    pub current_total_discounts_set: Option<CurrentTotalDiscountsSet>,
    pub current_total_duties_set: Option<String>,
    pub current_total_price: Option<String>,
    pub current_total_price_set: Option<CurrentTotalPriceSet>,
    pub current_total_tax: Option<String>,
    pub current_total_tax_set: Option<CurrentTotalTaxSet>,
    pub customer_locale: Option<String>,
    pub device_id: Option<String>,
    pub discount_codes: Option<Vec<String>>,
    pub email: String,
    pub estimated_taxes: Option<bool>,
    pub financial_status: Option<String>,
    pub fulfillment_status: Option<String>,
    pub landing_site: Option<String>,
    pub landing_site_ref: Option<String>,
    pub location_id: Option<String>,
    pub merchant_of_record_app_id: Option<String>,
    pub name: String,
    pub note: Option<String>,
    pub note_attributes: Option<Vec<String>>,
    pub number: i32,
    pub order_number: Option<i32>,
    pub order_status_url: Option<String>,
    pub original_total_additional_fees_set: Option<String>,
    pub original_total_duties_set: Option<String>,
    pub payment_gateway_names: Option<Vec<String>>,
    pub phone: Option<String>,
    pub po_number: Option<String>,
    pub presentment_currency: Option<String>,
    pub processed_at: Option<String>,
    pub reference: Option<String>,
    pub referring_site: Option<String>,
    pub source_identifier: Option<String>,
    pub source_name: Option<String>,
    pub source_url: Option<String>,
    pub subtotal_price: Option<String>,
    pub subtotal_price_set: Option<CurrentSubtotalPriceSet>,
    pub tags: String,
    pub tax_exempt: Option<bool>,
    pub tax_lines: Option<Vec<String>>,
    pub taxes_included: Option<bool>,
    pub test: Option<bool>,
    pub token: String,
    pub total_discounts: Option<String>,
    pub total_discounts_set: Option<CurrentTotalDiscountsSet>,
    pub total_line_items_price: Option<String>,
    pub total_line_items_price_set: Option<CurrentSubtotalPriceSet>,
    pub total_outstanding: Option<String>,
    pub total_price: Option<String>,
    pub total_price_set: Option<CurrentTotalPriceSet>,
    pub total_shipping_price_set: Option<Money>,
    pub total_tax: Option<String>,
    pub total_tax_set: Option<CurrentTotalTaxSet>,
    pub total_tip_received: Option<String>,
    pub total_weight: Option<i32>,
    pub updated_at: Option<String>,
    pub user_id: Option<String>,
    pub billing_address: Option<ShippingAddress>,
    pub customer: Option<Customer>,
    pub discount_applications: Option<Vec<String>>,
    pub fulfillments: Option<Vec<String>>,
    pub line_items: Option<Vec<LineItem>>,
    pub payment_terms: Option<String>,
    pub refunds: Option<Vec<String>>,
    pub shipping_address: Option<ShippingAddress>,
    pub shipping_lines: Option<Vec<ShippingLine>>,
}
