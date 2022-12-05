use std::error::Error;
use std::fmt;

use serde::Deserialize;

#[derive(Debug)]
pub enum OpenAPIError {
    HTTPError(reqwest::Error),
    BadRequest(OpenAPIBadRequest),
}

impl Error for OpenAPIError {}

impl fmt::Display for OpenAPIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpenAPIError")
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct OpenAPIBadRequest {
    ErrorCode: String,
    Message: String,
    //modelState: Option<String>, // TODO: fix to proper format
}

impl fmt::Display for OpenAPIBadRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpenAPIBadRequest")
    }
}

impl From<reqwest::Error> for OpenAPIError {
    fn from(err: reqwest::Error) -> Self {
        OpenAPIError::HTTPError(err)
    }
}

// TODO: Parse the errorCode
#[derive(Debug, Deserialize)]
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
