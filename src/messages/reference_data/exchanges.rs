use saxobank_macro::{SaxoRequest, SaxoResponse};
use crate::{saxo_request_odata, saxo_response_odata};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use strum::EnumString;

saxo_request_odata!{
    "ref/v1/exchanges/"
}


// OData protocol. See: https://msdn.microsoft.com/en-us/library/jj643270.aspx
saxo_response_odata!{
    all_day: bool,
    country_code: String,
    currency: String,
    exchange_id: String,
    exchange_sessions: ExchangeSession,
    iso_mic: String,
    name: String,
    operating_mic: String,
    price_source_name: String,
    time_zone: i32,
    time_zone_abbreviation: String,
    time_zone_id: String
    // TODO
    //time_zone_offset: TimeSpan,
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