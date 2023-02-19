use std::error::Error;
use std::fmt;

use serde::ser::StdError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub enum OpenAPIError {
    HTTPError(Box<dyn StdError>),
    Unauthorized,
    BadRequest(OpenAPIBadRequest),
}

impl Error for OpenAPIError {}

impl fmt::Display for OpenAPIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO
        write!(f, "OpenAPIError")
    }
}

impl From<reqwest::Error> for OpenAPIError {
    fn from(err: reqwest::Error) -> Self {
        OpenAPIError::HTTPError(Box::new(err))
    }
}

impl From<Box<dyn StdError>> for OpenAPIError {
    fn from(err: Box<dyn StdError>) -> Self {
        OpenAPIError::HTTPError(err)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAPIBadRequest {
    ErrorCode: ErrorCode,
    Message: String,
    //modelState: Option<String>, // TODO: fix to proper format
}

impl OpenAPIBadRequest {
    pub fn error_code(&self) -> &ErrorCode {
        &self.ErrorCode
    }
    pub fn message(&self) -> &str {
        &self.Message
    }
}

impl fmt::Display for OpenAPIBadRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpenAPIBadRequest")
    }
}

// TODO: Parse the errorCode
#[derive(Debug, PartialEq)]
pub enum ErrorCode {
    /// Default error code returned when it cannot be determined which part of the request is malformed.
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
            _ => todo!(),
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
            &_ => todo!()
        })
    }
}

impl Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::InvalidRequest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_test::{Token, assert_tokens};

    #[test]
    fn test_bad_request()
    {
        let bad_request = OpenAPIBadRequest {
            ErrorCode: ErrorCode::InvalidRequest,
            Message: "foo".to_string(),
        };

        assert_tokens(&bad_request, &[
            Token::Struct{ name: "OpenAPIBadRequest", len: 2 },
            Token::Str("ErrorCode"),
            Token::Str("InvalidRequest"),
            Token::Str("Message"),
            Token::Str("foo"),
            Token::StructEnd,
        ]);
    }

}