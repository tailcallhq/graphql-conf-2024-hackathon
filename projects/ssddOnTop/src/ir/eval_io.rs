use crate::ir::eval_ctx::EvalContext;
use crate::ir::eval_http::EvalHttp;
use crate::ir::IO;
use crate::request_context::CacheErr;
use crate::value::Value;

pub async fn eval_io(io: &IO, ctx: &mut EvalContext<'_>) -> anyhow::Result<Value> {
    let key = io.cache_key(ctx);

    ctx.request_ctx
        .cache
        .dedupe(&key, || async {
            ctx.request_ctx
                .cache
                .dedupe(&key, || eval_io_inner(io, ctx))
                .await
        })
        .await.map_err(|v| v.into())
}

async fn eval_io_inner(io: &IO, ctx: &mut EvalContext<'_>) -> Result<Value, CacheErr> {
    match io {
        IO::Http { req_template, .. } => {
            let eval_http = EvalHttp::new(ctx, req_template);
            let request = eval_http.init_request()?;
            let response = eval_http.execute(request).await?;

            Ok(response.body)
        }
    }
}