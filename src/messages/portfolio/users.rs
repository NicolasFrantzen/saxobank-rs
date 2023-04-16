use crate::{OpenAPIRequest, OpenAPIResponse};

use serde::Deserialize;
use std::borrow::Cow;
use std::fmt;

pub struct Request(pub &'static str);

impl OpenAPIRequest for Request {
    type ResponseType = Response;

    fn id(&self) -> &str {
        self.0
    }

    fn path() -> &'static str {
        "port/v1/users/"
    }
}

#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    pub client_key: Option<String>,
    pub culture: Option<String>,
    pub language: Option<String>,
    pub last_login_status: Option<String>,
    pub last_login_time: Option<String>,
    pub legal_asset_types: Option<Vec<String>>,
    pub market_data_via_open_api_terms_accepted: Option<bool>,
    pub name: Option<String>,
    pub time_zone_id: Option<i32>,
    pub user_id: Option<String>,
    pub user_key: Option<String>,
}

impl<'a> fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpenAPIRequest: Get user info")
    }
}

impl<'a> OpenAPIResponse for Response { }
