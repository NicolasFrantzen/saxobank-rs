//! Request and response definition for
//! <https://www.developer.saxo/openapi/referencedocs/port/v1/clients>

use crate::{saxo_request, saxo_response};

saxo_request!("port/v1/clients/");
saxo_response! {
    account_value_protection_limit: f32,
    allowed_netting_profiles: Vec<String>,
    allowed_trading_sessions: String,
    client_id: String,
    client_key: String,
    currency_decimals: u8,
    default_account_id: String,
    default_account_key: String,
    default_currency: String,
    force_open_default_value: bool,
    is_margin_trading_allowed: bool,
    is_variation_margin_eligible: bool,
    legal_asset_types: Vec<String>,
    legal_asset_types_are_indicative: bool,
    margin_calculation_method: String,
    margin_monitoring_mode: String,
    mutual_funds_cash_amount_order_currency: String,
    name: String,
    partner_platform_id: String,
    position_netting_method: String,
    position_netting_mode: String,
    position_netting_profile: String,
    reduce_exposure_only: bool,
    supports_account_value_protection_limit: bool
}
