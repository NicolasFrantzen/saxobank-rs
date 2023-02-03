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
    pub ClientKey: Option<Cow<'a, str>>,
    pub Culture: Option<Cow<'a, str>>,
    pub Language: Option<Cow<'a, str>>,
    pub LastLoginStatus: Option<Cow<'a, str>>,
    pub LastLoginTime: Option<Cow<'a, str>>,
    pub LegalAssetTypes: Option<Vec<Cow<'a, str>>>,
    pub MarketDataViaOpenApiTermsAccepted: Option<bool>,
    pub Name: Option<Cow<'a, str>>,
    pub TimeZoneId: Option<i32>,
    pub UserId: Option<Cow<'a, str>>,
    pub UserKey: Option<Cow<'a, str>>,
}
