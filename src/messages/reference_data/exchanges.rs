use saxobank_macro::{SaxoRequest, SaxoResponse};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use strum::EnumString;
use std::borrow::Cow;
use std::fmt;

#[derive(SaxoRequest)]
#[saxo(openapi_path = "ref/v1/exchanges/")]
pub struct Request(pub &'static str);

// TODO: Make proc macro for OData messages. https://msdn.microsoft.com/en-us/library/jj643270.aspx
#[derive(SaxoResponse, Deserialize, Debug, Default, PartialEq)]
pub struct Response {
    #[serde(rename = "__count")]
    pub count: Option<i32>,
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
    pub exchange_sessions: Option<ExchangeSession>,
    pub iso_mic: Option<String>,
    pub name: Option<String>,
    pub operating_mic: Option<String>,
    pub price_source_name: Option<String>,
    pub time_zone: Option<i32>,
    pub time_zone_abbreviation: Option<String>,
    pub time_zone_id: Option<String>,
    // TODO
    //pub time_zone_offset: Option<TimeSpan>,
}

#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangeSession {
    pub end_time: Option<DateTime<Utc>>,
    pub start_time: Option<DateTime<Utc>>,
    pub state: Option<ExchangeSessionState>,
}

// TODO: Make own EnumString proc implementation?
#[derive(EnumString, Deserialize, Debug, PartialEq)]
pub enum ExchangeSessionState {
    /// Participants place orders to buy or sell units at certain buying or selling prices. Orders collected during an auction are matched to form a contract
    Auction,
    /// Normal trading
    AutomatedTrading,
    /// Break in exchange opening hours
    Break,
    /// Participants place orders to buy or sell units at certain buying or selling prices. Orders collected during an auction are matched to form a contract
    CallAuctionTrading,
    /// Exchange is closed
    Closed,
    /// Trading is halted on the Exchange,
    Halt,
    /// Opening auction exchange state
    OpeningAuction,
    /// Transactions are conducted in trading pits on the floor of the Exchange
    PitTrading,
    /// Extended trading session after normal opening hours
    PostAutomatedTrading,
    /// Extended trading session after normal opening hours
    PostMarket,
    /// Extended trading session after normal opening hours
    PostTrading,
    /// Extended trading session before normal opening hours
    PreAutomatedTrading,
    /// Extended trading session before normal opening hours
    PreMarket,
    /// Extended trading session before normal opening hours
    PreTrading,
    /// Trading is suspended on the Exchange
    Suspended,
    /// Orders collected at last price in absence of closing auction price
    TradingAtLast,
    /// The state of the Exchange is unknown
    Undefined,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn test_serde_exchange_session()
    {
        let response = json!({
            "EndTime": "2021-05-21T00:00:00Z",
            "StartTime": "2021-05-21T00:00:00Z",
            "State": "PreTrading"
        });

        let response_deserialized = serde_json::from_str::<ExchangeSession>(&response.to_string());
        assert!(response_deserialized.is_ok());
        println!("{:?}", response_deserialized);
    }

    #[test]
    fn test_serde_exchanges()
    {
        let response = json!({
            "__next": "/openapi/....../?$top=1&$skip=1",
            "Data": [
              {
                "CountryCode": "ES",
                "Currency": "EUR",
                "ExchangeId": "SIBE",
                "IsoMic": "XMAD",
                "Mic": "XMCE",
                "Name": "BME Spanish Exchanges",
                "OperatingMic": "BMEX",
                "TimeZone": 4,
                "TimeZoneOffset": "00:00:00"
              }
            ]
          });

        let response_deserialized = serde_json::from_str::<Response>(&response.to_string());
        assert!(response_deserialized.is_ok());
        println!("{:?}", response_deserialized);
    }
}