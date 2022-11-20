#![allow(non_snake_case)]
use crate::OpenAPIResponse;

use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize, Debug, Default)]
pub struct Response<'a> {
    AccountValueProtectionLimit: f32,
    AllowedNettingProfiles: Vec<Cow<'a, str>>,
    AllowedTradingSessions: Cow<'a, str>,
    ClientId: Cow<'a, str>,
    ClientKey: Cow<'a, str>,
    CurrencyDecimals: u8,
    DefaultAccountId: Cow<'a, str>,
    DefaultAccountKey: Cow<'a, str>,
    DefaultCurrency: Cow<'a, str>,
    ForceOpenDefaultValue: bool,
    IsMarginTradingAllowed: bool,
    IsVariationMarginEligible: bool,
    LegalAssetTypes: Vec<Cow<'a, str>>,
    LegalAssetTypesAreIndicative: bool,
    MarginCalculationMethod: Cow<'a, str>,
    MarginMonitoringMode: Cow<'a, str>,
    MutualFundsCashAmountOrderCurrency: Cow<'a, str>,
    Name: Cow<'a, str>,
    PartnerPlatformId: Cow<'a, str>,
    PositionNettingMethod: Cow<'a, str>,
    PositionNettingMode: Cow<'a, str>,
    PositionNettingProfile: Cow<'a, str>,
    ReduceExposureOnly: bool,
    SupportsAccountValueProtectionLimit: bool,
}

impl<'a> OpenAPIResponse for Response<'a> {
    fn path() -> String {
        String::from("port/v1/clients/")
    }
}
