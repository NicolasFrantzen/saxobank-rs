#![allow(unused)]

pub mod client;
pub mod error;
pub mod messages;

use std::fmt;

pub(crate) trait SaxoRequest {
    type ResponseType;

    fn id(&self) -> &str;
    fn path() -> &'static str;
}

pub trait SaxoResponse: fmt::Display + fmt::Debug { }


/// Defines a Request and implements SaxoRequest trait with specified path.
/// Following example shows usage:
/// TODO
#[macro_export]
macro_rules! saxo_request {
    ($str: tt) => {
        use $crate::SaxoRequest;

        pub struct Request(pub &'static str);

        impl SaxoRequest for Request {
            type ResponseType = Response;

            fn id(&self) -> &str {
                self.0
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
        use $crate::SaxoResponse;
        use serde::Deserialize;

        #[derive(Deserialize, Debug, Default, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub struct $name {
            $(pub $fname : Option<$ftype>),*
        }
    };

    ( $($fname:ident : $ftype:ty),* ) => {
        saxo_response!{struct Response { $($fname : $ftype),* }}
    };
}
