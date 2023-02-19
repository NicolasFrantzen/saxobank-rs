use crate::{OpenAPIRequest, OpenAPIResponse};

use serde::Deserialize;
use std::borrow::Cow;
use std::fmt;

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

impl<'a> fmt::Display for Response<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpenAPIRequest: Get user info")
    }
}

impl<'a> OpenAPIResponse for Response<'a> { }
