use std::error;
use std::fmt;
use std::collections::HashMap;

use serde::ser::StdError;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Map;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct SaxoClientError(#[from] reqwest::Error);


#[derive(thiserror::Error, Debug)]
pub enum SaxoError {
    #[error("HTTP error")]
    HTTPError(#[from] Box<dyn StdError>),
    #[error("unauthorized. Did you forget a valid Open API token?")]
    Unauthorized,
    #[error("Bad request")]
    BadRequest(SaxoBadRequest),
}

impl From<reqwest::Error> for SaxoError {
    fn from(err: reqwest::Error) -> Self {
        SaxoError::HTTPError(Box::new(err))
    }
}

type ModelStateType = HashMap<String, Vec<String>>;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct SaxoBadRequest {
    ErrorCode: ErrorCode,
    Message: String,
    ModelState: Option<ModelStateType>,
}

impl SaxoBadRequest {
    pub fn error_code(&self) -> &ErrorCode {
        &self.ErrorCode
    }
    pub fn message(&self) -> &str {
        &self.Message
    }
}

impl fmt::Display for SaxoBadRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SaxoBadRequest")
    }
}

/// Generic and domain specific error codes
#[derive(Debug, PartialEq, Default)]
pub enum ErrorCode {
    /// Default error code returned when it cannot be determined which part of the request is malformed.
    #[default]
    InvalidRequest,
    /// Error code returned when one or more of the request headers are invalid.
    /// Used when the specific request header cannot be determined.
    InvalidRequestHeader,
    /// Error code returned when the Accept or Content-Type headers contains an invalid media type or is malformed.
    InvalidMediaTypeHeader,
    /// Error code returned when the Accept-Language header contains an invalid language or is malformed.
    InvalidAcceptLanguageHeader,
    /// Error code returned from query end points, when query parameters are invalid.
    InvalidQueryParameters,
    /// Error code returned when model state is invalid.
    InvalidModelState,
    /// Error code returned when type-conversion failed (TypeConverter's and ModelBinder's).
    TypeConversionError,
    /// Error code returned when more than the maximum allowed number of subscriptions for a specified type,
    /// is exceeded.
    SubscriptionLimitExceeded,
    /// Error code returned when a throttling policy quota has been exceeded.
    RateLimitExceeded,
    /// Error code returned when an Open Api feature has been disabled via Front Office.
    FeatureNotEnabled,
    /// Error code returned when a timeout occurs internally in the application.
    InternalTimeout,
    /// Error code returned when a subscription format that isn't supported by the publisher is requested.
    UnsupportedSubscriptionFormat,
    /// Error code returned if a request is not allowed.
    RequestNotAllowed,
    /// Error code returned when domain validation fails.
    DomainValidationError,
}

impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(match *self {
            ErrorCode::InvalidRequest => "InvalidRequest",
            ErrorCode::InvalidRequestHeader => "InvalidRequestHeader",
            ErrorCode::InvalidMediaTypeHeader => "InvalidMediaTypeHeader",
            ErrorCode::InvalidAcceptLanguageHeader => "InvalidAcceptLanguageHeader",
            ErrorCode::InvalidQueryParameters => "InvalidQueryParameters",
            ErrorCode::InvalidModelState => "InvalidModelState",
            ErrorCode::TypeConversionError => "TypeConversionError",
            ErrorCode::SubscriptionLimitExceeded => "SubscriptionLimitExceeded",
            ErrorCode::RateLimitExceeded => "RateLimitExceeded",
            ErrorCode::FeatureNotEnabled => "FeatureNotEnabled",
            ErrorCode::InternalTimeout => "InternalTimeout",
            ErrorCode::UnsupportedSubscriptionFormat => "UnsupportedSubscriptionFormat",
            ErrorCode::RequestNotAllowed => "RequestNotAllowed",
            ErrorCode::DomainValidationError => "DomainValidationError",
        })
    }
}

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "InvalidRequest" => ErrorCode::InvalidRequest,
            "InvalidRequestHeader" => ErrorCode::InvalidRequestHeader,
            "InvalidMediaTypeHeader" => ErrorCode::InvalidMediaTypeHeader,
            "InvalidAcceptLanguageHeader" => ErrorCode::InvalidAcceptLanguageHeader,
            "InvalidQueryParameters" => ErrorCode::InvalidQueryParameters,
            "InvalidModelState" => ErrorCode::InvalidModelState,
            "TypeConversionError" => ErrorCode::TypeConversionError,
            "SubscriptionLimitExceeded" => ErrorCode::SubscriptionLimitExceeded,
            "RateLimitExceeded" => ErrorCode::RateLimitExceeded,
            "FeatureNotEnabled" => ErrorCode::FeatureNotEnabled,
            "InternalTimeout" => ErrorCode::InternalTimeout,
            "UnsupportedSubscriptionFormat" => ErrorCode::UnsupportedSubscriptionFormat,
            "RequestNotAllowed" => ErrorCode::RequestNotAllowed,
            "DomainValidationError" => ErrorCode::DomainValidationError,
            &_ => {
                return Err(D::Error::custom("Unknown ErrorCode!"))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;
    use serde_test::{Token, assert_tokens, assert_de_tokens_error};

    #[test]
    fn test_serde_bad_request()
    {
        let bad_request = SaxoBadRequest {
            ErrorCode: ErrorCode::InvalidRequestHeader,
            Message: "foo".to_string(),
            ModelState: None,
        };

        assert_tokens(&bad_request, &[
            Token::Struct{ name: "SaxoBadRequest", len: 3 },
            Token::Str("ErrorCode"),
            Token::Str("InvalidRequestHeader"),
            Token::Str("Message"),
            Token::Str("foo"),
            Token::Str("ModelState"),
            Token::None,
            Token::StructEnd,
        ]);
    }

    #[test]
    fn test_serde_bad_request_modal_state()
    {
        let model_state = HashMap::from([
            ("$skip".to_owned(), vec!["Invalid $skip query parameter value: 2s".to_owned()])
        ]);

        let bad_request = SaxoBadRequest {
            ErrorCode: ErrorCode::InvalidRequestHeader,
            Message: "foo".to_string(),
            ModelState: Some(model_state),
        };

        assert_tokens(&bad_request, &[
            Token::Struct{ name: "SaxoBadRequest", len: 3 },
            Token::Str("ErrorCode"),
            Token::Str("InvalidRequestHeader"),
            Token::Str("Message"),
            Token::Str("foo"),
            Token::Str("ModelState"),
            Token::Some,
            Token::Map { len: Some(1), },
            Token::Str("$skip"),
            Token::Seq { len: Some(1), },
            Token::Str("Invalid $skip query parameter value: 2s"),
            Token::SeqEnd,
            Token::MapEnd,
            Token::StructEnd,
        ]);
    }

    #[test]
    fn test_serde_error_code_unknown()
    {
        assert_de_tokens_error::<ErrorCode>(
            &[
                Token::Str("ErrorCode"),
                Token::Str("Foo"),
            ],
            "Unknown ErrorCode!",
        );
    }

    /// Test that the format sampled in
    /// https://www.developer.saxo/openapi/learn/openapi-request-response
    /// Indeed can be deserialized, with our struct.
    #[test]
    fn test_serde_model_state()
    {
        let bad_request = json!({
            "ErrorCode":"InvalidModelState",
            "Message":"One or more properties of the request are invalid!",
            "ModelState":
             {
               "$skip":["Invalid $skip query parameter value: 2s"]
             },
        });

        println!("{:?}", bad_request);
        let _bad_request_deserialized: SaxoBadRequest = serde_json::from_str(&bad_request.to_string()).unwrap();
    }
}