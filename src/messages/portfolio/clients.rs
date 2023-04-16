use crate::{SaxoRequest, SaxoResponse};

use serde::Deserialize;
use std::borrow::Cow;
use std::fmt;

pub struct Request(pub &'static str);

impl SaxoRequest for Request {
    type ResponseType = Response;

    fn id(&self) -> &str {
        self.0
    }

    fn path() -> &'static str {
        "port/v1/clients/"
    }
}

#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    account_value_protection_limit: Option<f32>,
    allowed_netting_profiles: Option<Vec<String>>,
    allowed_trading_sessions: Option<String>,
    client_id: Option<String>,
    client_key: Option<String>,
    currency_decimals: Option<u8>,
    default_account_id: Option<String>,
    default_account_key: Option<String>,
    default_currency: Option<String>,
    force_open_default_value: Option<bool>,
    is_margin_trading_allowed: Option<bool>,
    is_variation_margin_eligible: Option<bool>,
    legal_asset_types: Option<Vec<String>>,
    legal_asset_types_are_indicative: Option<bool>,
    margin_calculation_method: Option<String>,
    margin_monitoring_mode: Option<String>,
    mutual_funds_cash_amount_order_currency: Option<String>,
    name: Option<String>,
    partner_platform_id: Option<String>,
    position_netting_method: Option<String>,
    position_netting_mode: Option<String>,
    position_netting_profile: Option<String>,
    reduce_exposure_only: Option<bool>,
    supports_account_value_protection_limit: Option<bool>,
}

impl<'a> fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpenAPIRequest: Get clients info")
    }
}

impl<'a> SaxoResponse for Response { }
