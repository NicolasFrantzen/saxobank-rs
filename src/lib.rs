#![allow(unused)]

pub mod client;
pub mod error;
pub mod messages;

use std::fmt;

pub(crate) trait OpenAPIRequest {
    type ResponseType;

    fn id(&self) -> &str;
    fn path() -> &'static str;
}

pub trait OpenAPIResponse: fmt::Display + fmt::Debug { }
