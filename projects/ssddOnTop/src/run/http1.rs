use crate::blueprint::{Blueprint, Server};
use crate::http::request_handler::handle_request;
use hyper::service::service_fn;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

pub async fn start(blueprint: Blueprint) -> anyhow::Result<()> {
    let blueprint = Arc::new(blueprint);
    let addr = blueprint.server.addr();
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Listening on: http://{}", addr);
    loop {
        let blueprint = blueprint.clone();

        let stream_result = listener.accept().await;
        match stream_result {
            Ok((stream, _)) => {
                let io = hyper_util::rt::TokioIo::new(stream);
                tokio::spawn(async move {
                    let blueprint = blueprint.clone();

                    let server = hyper::server::conn::http1::Builder::new()
                        .serve_connection(
                            io,
                            service_fn(move |req| {
                                let blueprint = blueprint.clone();

                                async move {
                                    let req =
                                        crate::http::request::Request::from_hyper(req).await?;
                                    handle_request(req, blueprint).await
                                }
                            }),
                        )
                        .await;
                    if let Err(e) = server {
                        tracing::error!("An error occurred while handling a request: {e}");
                    }
                });
            }
            Err(e) => tracing::error!("An error occurred while handling request: {e}"),
        }
    }
}
