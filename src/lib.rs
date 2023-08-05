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
            EndPointArgument::OData(odata) => write!(f, "?$top={}&$skip={}", odata.top, odata.skip),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ODataParams {
    top: i32,
    skip: i32,
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
        use $crate::SaxoRequest;
        use $crate::EndPointArgument;

        pub struct Request {
            argument: EndPointArgument, // TODO: maybe make constructor
        }

        impl Request {
            pub fn new(id: &'static str) -> Self {
                Request { argument: EndPointArgument::Id(id) }
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
                let params: ODataParams = serde_qs::from_str(
                    self.next.as_ref()?
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
