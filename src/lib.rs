#![allow(unused)]

pub mod port;
pub mod client;


pub(crate) trait OpenAPIRequest {
    type ResponseType<'a>;

    fn id(&self) -> &str;
    fn path() -> &'static str;
}
