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
