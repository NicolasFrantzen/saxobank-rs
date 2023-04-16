use crate::{SaxoRequest, SaxoResponse};

use serde::Deserialize;
use std::borrow::Cow;
use std::fmt;

pub struct Request(pub &'static str);

impl SaxoRequest for Request {
    type ResponseType = Response;

    fn id(&self) -> &str {
        self.0
    }

    fn path() -> &'static str {
        "ref/v1/exchanges/"
    }
}

#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct Response {
    #[serde(rename = "__next")]
    pub next: Option<String>,
    #[serde(rename = "Data")]
    pub data: Vec<ResponseData>,
}


#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseData {
    pub all_day: Option<bool>,
    pub country_code: Option<String>,
    pub currency: Option<String>,
    pub exchange_id: Option<String>,
    pub exchange_session: Option<ExchangeSession>,
    // TODO: Fill in the rest
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SaxoRequest: Get all exchanges")
    }
}

impl SaxoResponse for Response { }


#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct ExchangeSession {
    pub EndTime: Option<String>, // TODO: Parse UtcDateTime
    pub StartTime: Option<String>, // TODO: Parse UtcDateTime
    pub State: Option<String>, // TODO: Parse ExchangeSessionState enums with descriptions

}
