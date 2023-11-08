use crate::error::{ErrorCode, SaxoBadRequest, SaxoClientError, SaxoError};
use crate::messages::{portfolio, reference_data};
use crate::{EndPointArgument, ODataParams, SaxoRequest, SaxoResponse, SaxoResponseOData};

use async_trait::async_trait;
use mockall::automock;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use std::borrow::Cow;
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy)]
enum Env {
    Sim,
    Live,
}

impl From<Env> for String {
    fn from(env: Env) -> Self {
        match env {
            Env::Sim => String::from("sim"),
            Env::Live => String::from("live"),
        }
    }
}

#[automock]
#[async_trait]
pub trait HttpSend {
    async fn send(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, Box<dyn Error>>;
}

pub struct Sender;

#[async_trait]
impl HttpSend for Sender {
    async fn send(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        Ok(request.send().await?)
    }
}

pub struct SaxoClient<S: HttpSend = Sender> {
    client: reqwest::Client,
    sender: S,
    env: Env,
}

impl SaxoClient<Sender> {
    fn new(token: &str, env: Env) -> Result<Self, SaxoClientError> {
        Ok(SaxoClient {
            client: Self::build_client(token)?,
            sender: Sender,
            env,
        })
    }

    pub fn new_sim(token: &str) -> Result<Self, SaxoClientError> {
        Self::new(token, Env::Sim)
    }

    pub fn new_live(token: &str) -> Result<Self, SaxoClientError> {
        Self::new(token, Env::Live)
    }
}

impl<S: HttpSend> SaxoClient<S> {
    pub fn sim_with_sender(sender: S, token: &str) -> Result<Self, SaxoClientError> {
        Ok(SaxoClient {
            client: Self::build_client(token)?,
            sender,
            env: Env::Sim,
        })
    }

    fn build_client(token: &str) -> reqwest::Result<reqwest::Client> {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));
        headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("BEARER {}", token).as_str())
                .unwrap_or(HeaderValue::from_static("*/*")),
        );
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
    }

    async fn get<T: SaxoRequest>(&self, request: T) -> Result<T::ResponseType, SaxoError>
    where
        <T as SaxoRequest>::ResponseType: DeserializeOwned,
    {
        let env = String::from(self.env);
        let response = self
            .sender
            .send(self.client.get(format!(
                "https://gateway.saxobank.com/{}/openapi/{}{}", // TODO: make configurable and use .join instead
                env,
                T::endpoint(),
                request.argument()
            )))
            .await?;

        #[cfg(debug_assertions)]
        dbg!(&response);

        Self::parse_response::<T>(response).await
    }

    async fn parse_response<T: SaxoRequest>(
        response: reqwest::Response,
    ) -> Result<T::ResponseType, SaxoError>
    where
        <T as SaxoRequest>::ResponseType: DeserializeOwned,
    {
        match response.status() {
            // Bad request contains a body that needs to be serialized
            reqwest::StatusCode::BAD_REQUEST => Err(SaxoError::BadRequest(
                response.json::<SaxoBadRequest>().await?,
            )),
            reqwest::StatusCode::UNAUTHORIZED => Err(SaxoError::Unauthorized),

            // Otherwise continue deserialization
            // If error > 401 return deserialized HTTP error
            _ => Ok(response
                .error_for_status()?
                .json::<T::ResponseType>()
                .await?),
        }
    }

    pub async fn get_next<T: SaxoResponseOData>(
        &self,
        resp: &T,
    ) -> Result<<<T as SaxoResponse>::RequestType as SaxoRequest>::ResponseType, SaxoError>
    where
        <T as SaxoResponse>::RequestType: SaxoRequest,
        for<'de> <<T as SaxoResponse>::RequestType as SaxoRequest>::ResponseType: Deserialize<'de>,
    {
        self.get(resp.next().unwrap()).await // TODO: Fix unwrap
    }

    pub async fn get_port_user_info<'de>(&self) -> Result<portfolio::users::Response, SaxoError> {
        self.get(portfolio::users::Request::new("me")).await
    }

    pub async fn get_port_client_info(&self) -> Result<portfolio::clients::Response, SaxoError> {
        self.get(portfolio::clients::Request::new("me")).await
    }

    pub async fn get_ref_exchanges(
        &self,
    ) -> Result<reference_data::exchanges::Response, SaxoError> {
        // TODO: return a next handle?
        self.get(reference_data::exchanges::Request::new(ODataParams {
            top: Some(5),
            skip: Some(0),
        }))
        .await
    }

    pub async fn get_ref_exchanges2(
        &self,
        params: ODataParams,
    ) -> Result<NextHandle<'_, S, reference_data::exchanges::Response>, SaxoError> {
        let resp = self
            .get(reference_data::exchanges::Request::new(params))
            .await;
        Ok(NextHandle {
            client: self,
            resp: resp?,
        })
    }
}

pub struct NextHandle<'a, S: HttpSend, T: SaxoResponseOData> {
    client: &'a SaxoClient<S>,
    resp: T,
}

impl<'a, S: HttpSend, T: SaxoResponseOData> NextHandle<'a, S, T> {
    pub async fn next(
        self,
    ) -> Result<
        NextHandle<'a, S, <<T as SaxoResponse>::RequestType as SaxoRequest>::ResponseType>,
        SaxoError,
    >
    where
        <T as SaxoResponse>::RequestType: SaxoRequest,
        for<'de> <<T as SaxoResponse>::RequestType as SaxoRequest>::ResponseType: Deserialize<'de>,
        <<T as SaxoResponse>::RequestType as SaxoRequest>::ResponseType: SaxoResponseOData,
    {
        let resp = self.client.get_next(&self.resp).await;
        Ok(NextHandle {
            client: self.client,
            resp: resp?,
        })
    }
}

impl<'a, S: HttpSend, T: SaxoResponseOData> fmt::Debug for NextHandle<'a, S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.resp, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{saxo_request_odata, saxo_response_odata};
    use reqwest::Response;
    use serde_json::json;

    #[tokio::test]
    async fn test_parse_ok() {
        let mut mock_sender = MockHttpSend::new();
        let client = SaxoClient::sim_with_sender(mock_sender, "");

        let response =
            reqwest::Response::from(http::Response::builder().status(200).body("{}").unwrap());
        let api_response =
            SaxoClient::<Sender>::parse_response::<portfolio::users::Request>(response).await;

        #[cfg(debug_assertions)]
        dbg!(&api_response);
        assert!(api_response.is_ok());
    }

    #[tokio::test]
    async fn test_parse_unauthorized() {
        let mut mock_sender = MockHttpSend::new();
        let client = SaxoClient::sim_with_sender(mock_sender, "");

        let response =
            reqwest::Response::from(http::Response::builder().status(401).body("{}").unwrap());
        let api_response =
            SaxoClient::<Sender>::parse_response::<portfolio::users::Request>(response).await;

        #[cfg(debug_assertions)]
        dbg!(&api_response);
        assert!(api_response.is_err());

        assert!(matches!(api_response.unwrap_err(), SaxoError::Unauthorized));
    }

    #[tokio::test]
    async fn test_parse_bad_request() {
        let mut mock_sender = MockHttpSend::new();
        let client = SaxoClient::sim_with_sender(mock_sender, "");

        let status = 400;
        let response_body = json!({
            "ErrorCode": "InvalidRequest",
            "Message": "Invalid request message",
        });
        let response = reqwest::Response::from(
            http::Response::builder()
                .status(status)
                .body(response_body.to_string())
                .unwrap(),
        );
        let api_response =
            SaxoClient::<Sender>::parse_response::<portfolio::users::Request>(response).await;

        #[cfg(debug_assertions)]
        dbg!(&api_response);

        assert!(api_response.is_err());

        if let SaxoError::BadRequest(c) = api_response.unwrap_err() {
            assert_eq!(c.error_code(), &ErrorCode::InvalidRequest);
            assert_eq!(c.message(), "Invalid request message");
        } else {
            assert!(false);
        }
    }

    #[tokio::test]
    async fn test_get_port_user_info() {
        let mut mock_sender = MockHttpSend::new();

        mock_sender.expect_send().once().returning(move |_| {
            Ok(reqwest::Response::from(
                http::Response::builder()
                    .status(200)
                    .body(
                        json!({
                            "Name": "Foo",
                            "UserId": "Bar",
                            "Language": "C++",
                        })
                        .to_string(),
                    )
                    .unwrap(),
            ))
        });

        let client = SaxoClient::sim_with_sender(mock_sender, "").unwrap();

        // Check that the values came out properly
        let resp = client.get_port_user_info().await.unwrap(); // TODO: We should create messages with the macro instead of using specific ones

        assert_eq!(resp.name.unwrap(), "Foo");
        assert_eq!(resp.user_id.unwrap(), "Bar");
        assert_eq!(resp.language.unwrap(), "C++");
    }

    #[tokio::test]
    async fn test_get_odata_next() {
        saxo_request_odata!("foo/bar/");
        saxo_response_odata! {
            foo: String
        }

        let mut mock_sender = MockHttpSend::new();

        mock_sender.expect_send().return_once(move |_| {
            Ok(reqwest::Response::from(
                http::Response::builder()
                    .status(200)
                    .body(
                        json!({
                          "__next": "/foo/bar/?$top=123&$skip=42",
                          "Data": [
                            {
                              "Foo": "Bar",
                            }
                          ]
                        })
                        .to_string(),
                    )
                    .unwrap(),
            ))
        });

        let client = SaxoClient::sim_with_sender(mock_sender, "").unwrap();
        let resp = client
            .get(Request::new(ODataParams {
                top: Some(123),
                skip: Some(42),
            }))
            .await;

        #[cfg(debug_assertions)]
        dbg!(&resp);

        // TODO: We want to check that get_next calls get with a Request with the correct params
        // TODO: Create new mock, test calls
        //let next_resp = client.get_next(&resp.unwrap()).await; // Maybe we don't care about returning anything here
    }

    #[tokio::test]
    async fn test_next_handle() {
        // TODO
    }
}
