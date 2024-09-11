use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Default, Display)]
pub enum Method {
    #[default]
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
}

impl From<hyper::Method> for Method {
    fn from(value: hyper::Method) -> Self {
        match value {
            hyper::Method::GET => Method::GET,
            hyper::Method::POST => Method::POST,
            hyper::Method::PUT => Method::PUT,
            hyper::Method::PATCH => Method::PATCH,
            hyper::Method::DELETE => Method::DELETE,
            hyper::Method::HEAD => Method::HEAD,
            hyper::Method::OPTIONS => Method::OPTIONS,
            hyper::Method::CONNECT => Method::CONNECT,
            hyper::Method::TRACE => Method::TRACE,
            _ => unreachable!(),
        }
    }
}

impl Method {
    pub fn to_hyper(self) -> hyper::Method {
        match self {
            Method::GET => hyper::Method::GET,
            Method::POST => hyper::Method::POST,
            Method::PUT => hyper::Method::PUT,
            Method::PATCH => hyper::Method::PATCH,
            Method::DELETE => hyper::Method::DELETE,
            Method::HEAD => hyper::Method::HEAD,
            Method::OPTIONS => hyper::Method::OPTIONS,
            Method::CONNECT => hyper::Method::CONNECT,
            Method::TRACE => hyper::Method::TRACE,
        }
    }
}
