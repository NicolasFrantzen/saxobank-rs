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

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct Response<'a> {
    AccountValueProtectionLimit: Option<f32>,
    AllowedNettingProfiles: Option<Vec<Cow<'a, str>>>,
    AllowedTradingSessions: Option<Cow<'a, str>>,
    ClientId: Option<Cow<'a, str>>,
    ClientKey: Option<Cow<'a, str>>,
    CurrencyDecimals: Option<u8>,
    DefaultAccountId: Option<Cow<'a, str>>,
    DefaultAccountKey: Option<Cow<'a, str>>,
    DefaultCurrency: Option<Cow<'a, str>>,
    ForceOpenDefaultValue: Option<bool>,
    IsMarginTradingAllowed: Option<bool>,
    IsVariationMarginEligible: Option<bool>,
    LegalAssetTypes: Option<Vec<Cow<'a, str>>>,
    LegalAssetTypesAreIndicative: Option<bool>,
    MarginCalculationMethod: Option<Cow<'a, str>>,
    MarginMonitoringMode: Option<Cow<'a, str>>,
    MutualFundsCashAmountOrderCurrency: Option<Cow<'a, str>>,
    Name: Option<Cow<'a, str>>,
    PartnerPlatformId: Option<Cow<'a, str>>,
    PositionNettingMethod: Option<Cow<'a, str>>,
    PositionNettingMode: Option<Cow<'a, str>>,
    PositionNettingProfile: Option<Cow<'a, str>>,
    ReduceExposureOnly: Option<bool>,
    SupportsAccountValueProtectionLimit: Option<bool>,
}

impl<'a> fmt::Display for Response<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpenAPIRequest: Get clients info")
    }
}

impl<'a> OpenAPIResponse for Response<'a> { }
