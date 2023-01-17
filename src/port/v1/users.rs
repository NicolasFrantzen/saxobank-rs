use crate::OpenAPIRequest;

use serde::Deserialize;
use std::borrow::Cow;

pub struct Request(pub &'static str);

impl OpenAPIRequest for Request {
    type ResponseType<'a> = Response<'a>;

    fn id(&self) -> &str {
        self.0
    }

    fn path() -> &'static str {
        "port/v1/users/"
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct Response<'a> {
    ClientKey: Option<Cow<'a, str>>,
    Culture: Option<Cow<'a, str>>,
    Language: Option<Cow<'a, str>>,
    LastLoginStatus: Option<Cow<'a, str>>,
    LastLoginTime: Option<Cow<'a, str>>,
    LegalAssetTypes: Option<Vec<Cow<'a, str>>>,
    MarketDataViaOpenApiTermsAccepted: Option<bool>,
    Name: Option<Cow<'a, str>>,
    TimeZoneId: Option<i32>,
    UserId: Option<Cow<'a, str>>,
    UserKey: Option<Cow<'a, str>>,
}
