#![allow(unused)]

pub mod client;
pub mod error;
pub mod messages;

use std::fmt;

pub enum EndPointArgument {
    Id(&'static str),
    OData(ODataParams),
}

impl fmt::Display for EndPointArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EndPointArgument::Id(id) => write!(f, "{}", id),
            EndPointArgument::OData(odata) => {
                let mid = if odata.top.is_some() && odata.skip.is_some() {
                    "&"
                } else {
                    ""
                };
                write!(
                    f,
                    "?{}{}{}",
                    odata
                        .top
                        .map_or("".to_string(), |top| format!("$top={}", top)),
                    mid,
                    odata
                        .skip
                        .map_or("".to_string(), |skip| format!("$skip={}", skip))
                )
            }
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq, Default)]
pub struct ODataParams {
    #[serde(rename = "$top")]
    pub top: Option<i32>,
    #[serde(rename = "$skip")]
    pub skip: Option<i32>,
}

pub trait SaxoRequest {
    type ResponseType;

    fn argument(&self) -> &EndPointArgument;
    fn endpoint() -> &'static str
    where
        Self: Sized;
}
pub trait SaxoResponse: fmt::Display + fmt::Debug {
    type RequestType;
}

pub trait SaxoResponseOData: SaxoResponse {
    fn next(&self) -> Option<Self::RequestType>;
}

/// Defines a Request and implements SaxoRequest trait with specified path.
/// Following example shows usage:
/// TODO: Create example
#[macro_export]
macro_rules! saxo_request {
    ($str: tt) => {
        use $crate::EndPointArgument;
        use $crate::SaxoRequest;

        pub struct Request {
            argument: EndPointArgument, // TODO: maybe make constructor
        }

        impl Request {
            pub fn new(id: &'static str) -> Self {
                Request {
                    argument: EndPointArgument::Id(id),
                }
            }
        }

        impl SaxoRequest for Request {
            type ResponseType = Response;

            fn argument(&self) -> &EndPointArgument {
                &self.argument
            }

            fn endpoint() -> &'static str {
                $str
            }
        }
    };
}

#[macro_export]
macro_rules! saxo_request_odata {
    ($str: tt) => {
        use $crate::EndPointArgument;
        use $crate::ODataParams;
        use $crate::SaxoRequest;

        pub struct Request {
            argument: EndPointArgument, // TODO: maybe make constructor
        }

        impl Request {
            pub fn new(params: ODataParams) -> Self {
                Request {
                    argument: EndPointArgument::OData(params),
                }
            }
        }

        impl SaxoRequest for Request {
            type ResponseType = Response;

            fn argument(&self) -> &EndPointArgument {
                &self.argument
            }

            fn endpoint() -> &'static str {
                $str
            }
        }
    };
}

#[macro_export]
macro_rules! saxo_response {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        #[derive(serde::Deserialize, Debug, Default, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub struct $name {
            $(pub $fname : Option<$ftype>),*
        }

        impl $crate::SaxoResponse for $name {
            type RequestType = Request;
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "SaxoResponse") // TODO: Automatically print all tokens would be cool
            }
        }
    };

    ( $($fname:ident : $ftype:ty),* ) => {
        saxo_response!{struct Response { $($fname : $ftype),* }}
    };
}

#[macro_export]
macro_rules! saxo_response_odata {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        #[derive(serde::Deserialize, Debug, Default, PartialEq)]
        pub struct $name {
            #[serde(rename = "__count")]
            pub count: Option<i32>,
            #[serde(rename = "__next")]
            pub next: Option<String>,
            #[serde(rename = "Data")]
            pub data: Vec<ResponseData>,
        }

        impl $crate::SaxoResponse for $name {
            type RequestType = Request;
        }

        impl $crate::SaxoResponseOData for $name {
            fn next(&self) -> Option<Self::RequestType> {
                let uri = self.next.as_ref()?.parse::<http::Uri>().ok()?;
                let params: ODataParams = serde_qs::from_str(
                    uri.query()?
                ).ok()?;

                Some(Request::new(params))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "SaxoResponse") // TODO: Automatically print all tokens would be cool
            }
        }
    };

    ( $($fname:ident : $ftype:ty),* ) => {
        saxo_response_odata!{struct Response { $($fname : $ftype),* }}
        $crate::saxo_response!{struct ResponseData { $($fname : $ftype),* }}
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn test_serde_o_data_params() {
        assert_de_tokens(
            &ODataParams {
                top: Some(123),
                skip: Some(42),
            },
            &[
                Token::Struct {
                    name: "ODataParams",
                    len: 2,
                },
                Token::Str("$top"),
                Token::Some,
                Token::U8(123),
                Token::Str("$skip"),
                Token::Some,
                Token::U8(42),
                Token::StructEnd,
            ],
        );

        assert_de_tokens(
            &ODataParams::default(),
            &[
                Token::Struct {
                    name: "ODataParams",
                    len: 2,
                },
                Token::Str("$top"),
                Token::None,
                Token::Str("$skip"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_end_point_argument_display() {
        assert_eq!(format!("{}", EndPointArgument::Id("me")), "me");
        assert_eq!(
            format!("{}", EndPointArgument::OData(ODataParams::default())),
            "?"
        );
        assert_eq!(
            format!(
                "{}",
                EndPointArgument::OData(ODataParams {
                    top: Some(10),
                    skip: None
                })
            ),
            "?$top=10"
        );
        assert_eq!(
            format!(
                "{}",
                EndPointArgument::OData(ODataParams {
                    top: None,
                    skip: Some(42)
                })
            ),
            "?$skip=42"
        );
        assert_eq!(
            format!(
                "{}",
                EndPointArgument::OData(ODataParams {
                    top: Some(10),
                    skip: Some(42)
                })
            ),
            "?$top=10&$skip=42"
        );
    }
}
