#![allow(unused)]

pub mod port;
pub mod client;


pub(crate) trait OpenAPIResponse {
    fn path() -> String;
}
