use crate::{OpenAPIRequest, OpenAPIResponse};

use serde::Deserialize;
use std::borrow::Cow;
use std::fmt;

pub struct Request(pub &'static str);

impl OpenAPIRequest for Request {
    type ResponseType<'b> = Response<'b>;

    fn id(&self) -> &str {
        self.0
    }

    fn path() -> &'static str {
        "port/v1/clients/"
    }
}

#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Response<'a> {
    account_value_protection_limit: Option<f32>,
    allowed_netting_profiles: Option<Vec<Cow<'a, str>>>,
    allowed_trading_sessions: Option<Cow<'a, str>>,
    client_id: Option<Cow<'a, str>>,
    client_key: Option<Cow<'a, str>>,
    currency_decimals: Option<u8>,
    default_account_id: Option<Cow<'a, str>>,
    default_account_key: Option<Cow<'a, str>>,
    default_currency: Option<Cow<'a, str>>,
    force_open_default_value: Option<bool>,
    is_margin_trading_allowed: Option<bool>,
    is_variation_margin_eligible: Option<bool>,
    legal_asset_types: Option<Vec<Cow<'a, str>>>,
    legal_asset_types_are_indicative: Option<bool>,
    margin_calculation_method: Option<Cow<'a, str>>,
    margin_monitoring_mode: Option<Cow<'a, str>>,
    mutual_funds_cash_amount_order_currency: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    partner_platform_id: Option<Cow<'a, str>>,
    position_netting_method: Option<Cow<'a, str>>,
    position_netting_mode: Option<Cow<'a, str>>,
    position_netting_profile: Option<Cow<'a, str>>,
    reduce_exposure_only: Option<bool>,
    supports_account_value_protection_limit: Option<bool>,
}

impl<'a> fmt::Display for Response<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpenAPIRequest: Get clients info")
    }
}

impl<'a> OpenAPIResponse for Response<'a> { }
