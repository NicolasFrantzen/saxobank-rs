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
        "ref/v1/exchanges/"
    }
}

#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct Response<'a> {
    #[serde(rename = "__next")]
    pub next: Option<Cow<'a, str>>,
    #[serde(rename = "Data")]
    pub data: Vec<ResponseData<'a>>,
}


#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseData<'a> {
    pub all_day: Option<bool>,
    pub country_code: Option<Cow<'a, str>>,
    pub currency: Option<Cow<'a, str>>,
    pub exchange_id: Option<Cow<'a, str>>,
    pub exchange_session: Option<ExchangeSession<'a>>,
    // TODO: Fill in the rest
}

impl<'a> fmt::Display for Response<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpenAPIRequest: Get all exchanges")
    }
}

impl<'a> OpenAPIResponse for Response<'a> { }


#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct ExchangeSession<'a> {
    pub EndTime: Option<Cow<'a, str>>, // TODO: Parse UtcDateTime
    pub StartTime: Option<Cow<'a, str>>, // TODO: Parse UtcDateTime
    pub State: Option<Cow<'a, str>>, // TODO: Parse ExchangeSessionState enums with descriptions

}
