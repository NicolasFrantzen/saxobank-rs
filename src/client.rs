use crate::error::OpenAPIBadRequest;
use crate::port;
use crate::OpenAPIRequest;
use crate::error::OpenAPIError;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

use std::borrow::Cow;

#[derive(Clone, Copy)]
enum Env {
    Sim,
    Live,
}

impl From<Env> for String {
    fn from(env: Env) -> Self {
        match env {
            Env::Sim => "sim".to_owned(),
            Env::Live => "live".to_owned(),
        }
    }
}

pub struct OpenAPIClient {
    client: reqwest::Client,
    environment: Env,
}

impl OpenAPIClient {
    pub fn new_sim(token: &str) -> Self {
        Self::new(Env::Sim, token)
    }

    pub fn new_live(token: &str) -> Self {
        Self::new(Env::Live, token)
    }

    fn new(environment: Env, token: &str) -> Self {
        let mut headers = HeaderMap::new(); // TODO: Create header builder
        headers.insert("Accept", HeaderValue::from_static("*/*"));
        headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("BEARER {}", token).as_str())
                .unwrap_or(HeaderValue::from_static("*/*")),
        );
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        Self {
            environment,
            client: reqwest::ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap_or_default(),
        }
    }

    async fn get<'a, T: OpenAPIRequest>(
        &self,
        request: T,
    ) -> Result<T::ResponseType<'a>, OpenAPIError>
    where
        for<'de> <T as OpenAPIRequest>::ResponseType<'a>: Deserialize<'de>,
    {
        let env = String::from(self.environment);
        let response = self
            .client
            .get(format!(
                "https://gateway.saxobank.com/{}/openapi/{}{}", // TODO: make configurable
                env,
                T::path(),
                request.id()
            ))
            .send()
            .await?;

        dbg!(&response);
        Self::parse_response::<T>(response).await
    }

    async fn parse_response<'a, T: OpenAPIRequest>(response: reqwest::Response) -> Result<T::ResponseType<'a>, OpenAPIError>
    where
        for<'de> <T as OpenAPIRequest>::ResponseType<'a>: Deserialize<'de> {
        match response.status() {
            // Bad request contains a body that needs to be serialized
            reqwest::StatusCode::BAD_REQUEST => {
                Err(
                    OpenAPIError::BadRequest(
                        response
                        .json::<OpenAPIBadRequest>()
                        .await?
                    )
                )
            },
            // If the error code is > 400 return an OpenAPIError
            // Otherwise continue deserialization
            _ => {
                Ok(
                    response
                    .error_for_status()?
                    .json::<T::ResponseType<'a>>()
                    .await?
                )
            }
        }
    }

    pub async fn get_user_info<'a>(&self) -> Result<port::v1::users::Response, OpenAPIError> {
        self.get(port::v1::users::Request("me")).await
    }

    pub async fn get_client_info<'a>(&self) -> Result<port::v1::clients::Response, OpenAPIError> {
        self.get(port::v1::clients::Request("me")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment() {
        assert_eq!(String::from(Env::Sim), "sim".to_owned());
        assert_eq!(String::from(Env::Live), "live".to_owned());
    }
}
