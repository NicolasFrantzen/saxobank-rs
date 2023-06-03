use saxobank_macro::{SaxoRequest, SaxoResponse};

use serde::Deserialize;

#[derive(SaxoRequest)]
#[saxo(openapi_path = "port/v1/users/")]
pub struct Request(pub &'static str);

#[derive(SaxoResponse, Deserialize, Debug, Default, PartialEq)]
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
