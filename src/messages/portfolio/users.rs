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

#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Response<'a> {
    pub client_key: Option<Cow<'a, str>>,
    pub culture: Option<Cow<'a, str>>,
    pub language: Option<Cow<'a, str>>,
    pub last_login_status: Option<Cow<'a, str>>,
    pub last_login_time: Option<Cow<'a, str>>,
    pub legal_asset_types: Option<Vec<Cow<'a, str>>>,
    pub market_data_via_open_api_terms_accepted: Option<bool>,
    pub name: Option<Cow<'a, str>>,
    pub time_zone_id: Option<i32>,
    pub user_id: Option<Cow<'a, str>>,
    pub user_key: Option<Cow<'a, str>>,
}

impl<'a> fmt::Display for Response<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpenAPIRequest: Get user info")
    }
}

impl<'a> OpenAPIResponse for Response<'a> { }
