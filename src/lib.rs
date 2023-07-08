#![allow(unused)]

pub mod client;
pub mod error;
pub mod messages;

use std::fmt;

pub trait SaxoRequest {
    type ResponseType;

    fn id(&self) -> &str;
    fn path() -> &'static str where Self: Sized;
}

pub trait SaxoResponse: fmt::Display + fmt::Debug { }

pub trait SaxoResponseOData {
    fn next(&self) -> Option<Box<dyn SaxoRequest<ResponseType = Self>>>;
}

/// Defines a Request and implements SaxoRequest trait with specified path.
/// Following example shows usage:
/// TODO: Create example
#[macro_export]
macro_rules! saxo_request {
    ($str: tt) => {
        use $crate::SaxoRequest;

        pub struct Request {
            pub id: &'static str,
        }

        impl SaxoRequest for Request {
            type ResponseType = Response;

            // TODO: Concat these
            fn id(&self) -> &str {
                self.id
            }

            fn path() -> &'static str {
                $str
            }
        }
    };
}

#[macro_export]
macro_rules! saxo_request_odata {
    ($str: tt) => {
        use $crate::SaxoRequest;

        pub struct Request {
            pub next: String,
        }

        impl SaxoRequest for Request {
            type ResponseType = Response;

            // TODO: Concat these
            fn id(&self) -> &str {
                &self.next
            }

            fn path() -> &'static str {
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

        impl $crate::SaxoResponse for $name { }

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

        impl $crate::SaxoResponse for $name { }
        impl $crate::SaxoResponseOData for $name {
            fn next(&self) -> Option<Box<dyn SaxoRequest<ResponseType = Self>>> {
                Some(Box::new(Request{
                    next: self.next.clone()?,
                }))
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