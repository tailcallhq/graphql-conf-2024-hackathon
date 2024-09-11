use crate::blueprint::Blueprint;
use crate::http::request::Request;
use bytes::Bytes;
use http_body_util::Full;
use std::sync::Arc;
use crate::http::method::Method;

pub async fn handle_request(
    req: Request,
    blueprint: Arc<Blueprint>,
) -> anyhow::Result<hyper::Response<Full<Bytes>>> {
    let resp = match req.method {
        Method::GET => {
            hyper::Response::new(Full::new(Bytes::from_static(
                b"Hello, World!",
            )))
        }
        Method::POST => {
            handle_gql_req(req, blueprint).await?
        }
        _ => {
            hyper::Response::builder()
                .status(hyper::StatusCode::METHOD_NOT_ALLOWED)
                .body(Full::new(Bytes::from_static(b"Method Not Allowed")))?
        }
    };
    Ok(resp)
}

async fn handle_gql_req(
    _request: Request,
    _blueprint: Arc<Blueprint>,
) -> anyhow::Result<hyper::Response<Full<Bytes>>> {
    todo!()
}