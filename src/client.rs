use crate::error::{OpenAPIBadRequest, OpenAPIError, ErrorCode};
use crate::messages::{portfolio, reference_data};
use crate::OpenAPIRequest;

use async_trait::async_trait;
use mockall::automock;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Url;
use serde::Deserialize;

use std::borrow::Cow;
use std::error::Error;

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

pub struct OpenAPIClient<S: HttpSend = Sender> {
    client: reqwest::Client,
    sender: S,
    env: Env,
}

impl OpenAPIClient<Sender> {
    fn new(token: &str, env: Env) -> Self {
        OpenAPIClient {
            client: Self::build_client(token),
            sender: Sender,
            env,
        }
    }

    pub fn new_sim(token: &str) -> Self {
        Self::new(token, Env::Sim)
    }

    pub fn new_live(token: &str) -> Self {
        Self::new(token, Env::Live)
    }
}

impl<S: HttpSend> OpenAPIClient<S> {
    pub fn sim_with_sender(sender: S, token: &str) -> OpenAPIClient<S> {
        OpenAPIClient {
            client: Self::build_client(token),
            sender,
            env: Env::Sim,
        }
    }

    fn build_client(token: &str) -> reqwest::Client {
        let mut headers = HeaderMap::new(); // TODO: Create header builder
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
            .unwrap_or_default()
    }

    async fn get<'a, T: OpenAPIRequest>(
        &self,
        request: T,
    ) -> Result<T::ResponseType<'a>, OpenAPIError>
    where
        for<'de> <T as OpenAPIRequest>::ResponseType<'a>: Deserialize<'de>,
    {
        let env = String::from(self.env);
        let response = self
            .sender
            .send(self.client.get(format!(
                "https://gateway.saxobank.com/{}/openapi/{}{}", // TODO: make configurable and use .join instead
                env,
                T::path(),
                request.id()
            )))
            .await?;

        #[cfg(debug_assertions)]
        dbg!(&response);

        Self::parse_response::<T>(response).await
    }

    async fn parse_response<'a, T: OpenAPIRequest>(
        response: reqwest::Response,
    ) -> Result<T::ResponseType<'a>, OpenAPIError>
    where
        for<'de> <T as OpenAPIRequest>::ResponseType<'a>: Deserialize<'de>,
    {
        match response.status() {
            // Bad request contains a body that needs to be serialized
            reqwest::StatusCode::BAD_REQUEST => Err(OpenAPIError::BadRequest(
                response.json::<OpenAPIBadRequest>().await?,
            )),
            reqwest::StatusCode::UNAUTHORIZED => Err(OpenAPIError::Unauthorized),

            // Otherwise continue deserialization
            // If error > 401 return deserialized HTTP error
            _ => Ok(response
                .error_for_status()?
                .json::<T::ResponseType<'a>>()
                .await?),
        }
    }

    pub async fn get_port_user_info<'a>(&self) -> Result<portfolio::users::Response, OpenAPIError> {
        self.get(portfolio::users::Request("me")).await
    }

    pub async fn get_port_client_info<'a>(&self) -> Result<portfolio::clients::Response, OpenAPIError> {
        self.get(portfolio::clients::Request("me")).await
    }

    pub async fn get_ref_exchanges<'a>(&self) -> Result<reference_data::exchanges::Response, OpenAPIError> {
        self.get(reference_data::exchanges::Request("?$top=3&$skip=2")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use reqwest::Response;
    use serde_json::json;

    use std::{cell::RefCell, error::Error};

    #[tokio::test]
    async fn test_parse_ok() {
        let mut mock_sender = MockHttpSend::new();
        let client = OpenAPIClient::sim_with_sender(mock_sender, "");

        let response =
            reqwest::Response::from(http::Response::builder().status(200).body("{}").unwrap());
        let api_response =
            OpenAPIClient::<Sender>::parse_response::<portfolio::users::Request>(response).await;

        #[cfg(debug_assertions)]
        dbg!(&api_response);
        assert!(api_response.is_ok());
    }

    #[tokio::test]
    async fn test_parse_unauthorized() {
        let mut mock_sender = MockHttpSend::new();
        let client = OpenAPIClient::sim_with_sender(mock_sender, "");

        let response =
            reqwest::Response::from(http::Response::builder().status(401).body("{}").unwrap());
        let api_response =
            OpenAPIClient::<Sender>::parse_response::<portfolio::users::Request>(response).await;

        #[cfg(debug_assertions)]
        dbg!(&api_response);
        assert!(api_response.is_err());

        assert!(matches!(api_response.unwrap_err(), OpenAPIError::Unauthorized));
    }

    #[tokio::test]
    async fn test_parse_bad_request() {
        let mut mock_sender = MockHttpSend::new();
        let client = OpenAPIClient::sim_with_sender(mock_sender, "");

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
            OpenAPIClient::<Sender>::parse_response::<portfolio::users::Request>(response).await;

        #[cfg(debug_assertions)]
        dbg!(&api_response);

        assert!(api_response.is_err());

        if let OpenAPIError::BadRequest(c) = api_response.unwrap_err() {
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
                http::Response::builder().status(200).body(json!({
                    "Name": "Foo",
                    "UserId": "Bar",
                    "Language": "C++",
                }).to_string()).unwrap(),
            ))
        });

        let client = OpenAPIClient::sim_with_sender(mock_sender, "");

        // Check that the values came out properly
        let resp = client.get_port_user_info().await.unwrap();

        assert_eq!(resp.name.unwrap().as_ref(), "Foo");
        assert_eq!(resp.user_id.unwrap().as_ref(), "Bar");
        assert_eq!(resp.language.unwrap().as_ref(), "C++");
    }
}
