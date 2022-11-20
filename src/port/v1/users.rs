#![allow(non_snake_case)]
use crate::OpenAPIRequest;

use serde::Deserialize;
use std::borrow::Cow;

pub struct Request(pub &'static str);

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

impl OpenAPIRequest for Request {
    type ResponseType<'a> = Response<'a>;

    fn id(&self) -> &str {
        &self.0
    }

    fn path() -> &'static str {
        "port/v1/users/"
    }
}
