use saxobank_macro::{SaxoRequest, SaxoResponse};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::borrow::Cow;
use std::fmt;

#[derive(SaxoRequest)]
#[saxo(openapi_path = "ref/v1/exchanges/")]
pub struct Request(pub &'static str);

#[derive(SaxoResponse, Deserialize, Debug, Default, PartialEq)]
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

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct ExchangeSession {
    pub EndTime: Option<DateTime<Utc>>,
    pub StartTime: Option<DateTime<Utc>>,
    pub State: Option<String>, // TODO: Parse ExchangeSessionState enums with descriptions

}
