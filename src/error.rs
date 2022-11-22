pub enum ErrorCode {
    InvalidRequest,	// Default error code returned when it cannot be determined which part of the request is malformed.
    InvalidRequestHeader(String), // Error code returned when one or more of the request headers are invalid. Used when the specific request header cannot be determined.
    InvalidMediaTypeHeader(String), // Error code returned when the Accept or Content-Type headers contains an invalid media type or is malformed.
    InvalidAcceptLanguageHeader(String), // Error code returned when the Accept-Language header contains an invalid language or is malformed.
    InvalidQueryParameters(String), // Error code returned from query end points, when query parameters are invalid.
    InvalidModelState(String), // Error code returned when model state is invalid.
    TypeConversionError(String), // Error code returned when type-conversion failed (TypeConverter's and ModelBinder's).
    SubscriptionLimitExceeded, // Error code returned when more than the maximum allowed number of subscriptions for a specified type, is exceeded
    RateLimitExceeded, // Error code returned when a throttling policy quota has been exceeded.
    FeatureNotEnabled, // Error code returned when an Open Api feature has been disabled via Front Office.
    InternalTimeout, // Error code returned when a timeout occurs internally in the application.
    UnsupportedSubscriptionFormat(String), // Error code returned when a subscription format that isn't supported by the publisher is requested.
    RequestNotAllowed, // Error code returned if a request is not allowed.
    DomainValidationError(String), // Error code returned when domain validation fails.
}
