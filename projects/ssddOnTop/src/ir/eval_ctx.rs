use std::borrow::Cow;
use std::sync::Arc;
use crate::request_context::RequestContext;
use crate::value::Value;

#[derive(Clone)]
pub struct EvalContext<'a> {
    // Context create for each GraphQL Request
    pub request_ctx: &'a RequestContext,

    // graphql_ctx: &'a Ctx,

    graphql_ctx_value: Option<Arc<Value>>,

    graphql_ctx_args: Option<Arc<Value>>,
}


impl<'a> EvalContext<'a> {
    pub fn path_arg<T: AsRef<str>>(&self, path: &[T]) -> Option<Cow<'a, Value>> {
        let args = self.graphql_ctx_args.as_ref()?;
        get_path_value(args.as_ref(), path).map(|a| Cow::Owned(a.clone()))
    }

    pub fn path_value<T: AsRef<str>>(&self, path: &[T]) -> Option<Cow<'a, Value>> {
        // TODO: add unit tests for this
        if let Some(value) = self.graphql_ctx_value.as_ref() {
            get_path_value(value.as_ref(), path).map(|a| Cow::Owned(a))
        } else {
            Some(Cow::Owned(Value::new(serde_json::Value::Null)))
            // get_path_value(self.graphql_ctx.value()?, path).map(Cow::Borrowed)
        }
    }
}

pub fn get_path_value<'a, T: AsRef<str>>(input: &'a Value, path: &[T]) -> Option<Value> {
    let mut value = Some(input.serde());
    for name in path {
        match value {
            Some(serde_json::Value::Object(map)) => {
                value = map.get(name.as_ref());
            }

            Some(serde_json::Value::Array(list)) => {
                value = list.get(name.as_ref().parse::<usize>().ok()?);
            }
            _ => return None,
        }
    }

    value.map(|v| Value::new(v.clone()))
}

