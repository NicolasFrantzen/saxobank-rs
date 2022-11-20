use crate::port;
use crate::OpenAPIRequest;

use reqwest::header::{HeaderValue, HeaderMap};
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
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));
        headers.insert("Authorization", HeaderValue::from_str(format!("BEARER {}", token)
            .as_str()).unwrap_or(HeaderValue::from_static("*/*")));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        Self {
            environment,
            client: reqwest::ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap_or_default()
        }
    }

    async fn get<'a, T: OpenAPIRequest>(&self, request: T) -> Result<T::ResponseType<'a>, reqwest::Error>
        where for<'de> <T as OpenAPIRequest>::ResponseType<'a>: Deserialize<'de> {
        let env = String::from(self.environment);
        let body = self.client.get(format!("https://gateway.saxobank.com/{}/openapi/{}{}", env, T::path(), request.id())) // TODO: Create builder?
            .send()
            .await?
            .json::<T::ResponseType<'a>>()
            .await?;

        Ok(body)
    }

    pub async fn get_user_info<'a>(&self) -> Result<port::v1::users::Response, reqwest::Error> {
        Ok(self.get(port::v1::users::Request("me")).await?)
    }

    pub async fn get_client_info<'a>(&self) -> Result<port::v1::clients::Response, reqwest::Error> {
        Ok(self.get(port::v1::clients::Request("me")).await?)
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
