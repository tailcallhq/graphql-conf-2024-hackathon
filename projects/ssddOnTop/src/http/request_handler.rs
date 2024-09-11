use crate::blueprint::Blueprint;
use crate::http::request::Request;
use bytes::Bytes;
use http_body_util::Full;
use std::sync::Arc;

pub async fn handle_request(
    req: Request,
    blueprint: Arc<Blueprint>,
) -> anyhow::Result<hyper::Response<Full<Bytes>>> {
    Ok(hyper::Response::new(Full::new(Bytes::from_static(
        b"Hello, World!",
    ))))
}
