#![allow(non_snake_case)]
use crate::OpenAPIResponse;

use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize, Debug, Default)]
pub struct Response<'a> {
    ClientKey: Cow<'a, str>,
    Culture: Cow<'a, str>,
    Language: Cow<'a, str>,
    LastLoginStatus: Cow<'a, str>,
    LastLoginTime: Cow<'a, str>,
    LegalAssetTypes: Vec<Cow<'a, str>>,
    MarketDataViaOpenApiTermsAccepted: bool,
    Name: Cow<'a, str>,
    TimeZoneId: i32,
    UserId: Cow<'a, str>,
    UserKey: Cow<'a, str>,
}

impl<'a> OpenAPIResponse for Response<'a> {
    fn path() -> String {
        String::from("port/v1/users/")
    }
}
