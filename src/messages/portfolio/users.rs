use crate::{saxo_request, saxo_response};

saxo_request!("port/v1/users/");

saxo_response!{
    client_key: String,
    culture: String,
    language: String,
    last_login_status: String,
    last_login_time: String,
    legal_asset_types: Vec<String>,
    market_data_via_open_api_terms_accepted: bool,
    name: String,
    time_zone_id: i32,
    user_id: String,
    user_key: String
}

