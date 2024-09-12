use std::ops::Deref;
use crate::blueprint::Blueprint;
use crate::http::request::Request;
use bytes::Bytes;
use http_body_util::Full;
use std::sync::Arc;
use crate::app_ctx::AppCtx;
use crate::http::method::Method;
use crate::ir::eval_ctx::EvalContext;
use crate::request_context::RequestContext;

pub async fn handle_request(
    req: Request,
    app_ctx: AppCtx,
) -> anyhow::Result<hyper::Response<Full<Bytes>>> {
    let resp = match req.method {
        Method::GET => {
            hyper::Response::new(Full::new(Bytes::from_static(
                b"Hello, World!",
            )))
        }
        Method::POST => {
            handle_gql_req(req, app_ctx).await?
        }
        _ => {
            hyper::Response::builder()
                .status(hyper::StatusCode::METHOD_NOT_ALLOWED)
                .body(Full::new(Bytes::from_static(b"Method Not Allowed")))?
        }
    };
    Ok(resp)
}

fn create_request_context(app_ctx: &AppCtx) -> RequestContext {
    RequestContext::from(app_ctx)
}


async fn handle_gql_req(
    request: Request,
    app_ctx: AppCtx,
) -> anyhow::Result<hyper::Response<Full<Bytes>>> {
    let gql_req: async_graphql::Request = serde_json::from_slice(&request.body)?;
    let doc = async_graphql::parser::parse_query(&gql_req.query)?;
    let req_ctx = create_request_context(&app_ctx);
    if let Some(query) = app_ctx.blueprint.schema.query.as_ref() {
        let mut eval_ctx = EvalContext::new(&req_ctx);

        for (_,field) in app_ctx.blueprint.fields.iter() {
            if let Some(ir) = field.ir.as_ref() {
                println!("hx: {}", field.name.as_ref());
                println!("{}", ir.eval(&mut eval_ctx).await?);
            }else {
                println!("hx1: {}", field.name.as_ref());
            }
        }
        Ok(
            hyper::Response::new(Full::new(Bytes::from_static(
                b"Printed",
            )))
        )
    } else {
        Ok(
            hyper::Response::new(Full::new(Bytes::from_static(
                b"Only queries are suppored",
            )))
        )
    }
}