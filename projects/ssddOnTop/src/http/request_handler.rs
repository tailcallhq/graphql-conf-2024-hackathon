use crate::app_ctx::AppCtx;
use crate::blueprint::model::{FieldName, TypeName};
use crate::blueprint::FieldHash;
use crate::http::method::Method;
use crate::http::request::Request;
use crate::ir::eval_ctx::EvalContext;
use crate::request_context::RequestContext;
use crate::value::Value;
use bytes::Bytes;
use http_body_util::Full;

pub async fn handle_request(
    req: Request,
    app_ctx: AppCtx,
) -> anyhow::Result<hyper::Response<Full<Bytes>>> {
    let resp = match req.method {
        Method::GET => hyper::Response::new(Full::new(Bytes::from_static(b"Hello, World!"))),
        Method::POST => handle_gql_req(req, app_ctx).await?,
        _ => hyper::Response::builder()
            .status(hyper::StatusCode::METHOD_NOT_ALLOWED)
            .body(Full::new(Bytes::from_static(b"Method Not Allowed")))?,
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
        let x = app_ctx.blueprint.fields.get(&FieldHash {
            name: FieldName("post".to_string()),
            id: TypeName(query.to_string()),
        });
        let x = x.as_ref().unwrap().ir.as_ref().unwrap();
        let mut map = serde_json::Map::new();
        let key = "id".to_string();
        let val = serde_json::Value::Number(serde_json::Number::from(1));
        map.insert(key, val);
        eval_ctx = eval_ctx.with_args(Value::new(serde_json::Value::Object(map)));

        let x = x.eval(&mut eval_ctx).await?;
        println!("{}", x);
        /*for (_,field) in app_ctx.blueprint.fields.iter() {
            if let Some(ir) = field.ir.as_ref() {
                println!("hx: {}", field.name.as_ref());
                println!("{}", ir.eval(&mut eval_ctx).await?);
            }else {
                println!("hx1: {}", field.name.as_ref());
            }
        }*/
        Ok(hyper::Response::new(Full::new(Bytes::from_static(
            b"Printed",
        ))))
    } else {
        Ok(hyper::Response::new(Full::new(Bytes::from_static(
            b"Only queries are suppored",
        ))))
    }
}
