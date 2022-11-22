#![allow(unused)]

pub mod client;
pub mod error;
pub mod port;

pub(crate) trait OpenAPIRequest {
    type ResponseType<'a>;

    fn id(&self) -> &str;
    fn path() -> &'static str;
}
