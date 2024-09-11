use std::hash::{Hash, Hasher};
use crate::hasher::MyHasher;
use crate::helpers::headers::MustacheHeaders;
use crate::ir::IoId;
use crate::mustache::model::Mustache;

#[derive(Debug, Clone)]
pub struct RequestTemplate {
    pub root_url: Mustache,
    pub query: Vec<Query>,
    pub method: reqwest::Method,
    pub headers: MustacheHeaders,
}

#[derive(Debug, Clone)]
pub struct Query {
    pub key: String,
    pub value: Mustache,
    pub skip_empty: bool,
}

/*impl RequestTemplate {
    pub fn cache_key(&self, ctx: &EvalContext) -> IoId {
        let mut hasher = MyHasher::default();
        let state = &mut hasher;

        self.method.hash(state);

        for (name, mustache) in self.headers.iter() {
            name.hash(state);
            mustache.render(ctx).hash(state);
        }

        for (name, value) in ctx.headers().iter() {
            name.hash(state);
            value.hash(state);
        }

        let url = self.create_url(ctx).unwrap();
        url.hash(state);

        IoId::new(hasher.finish())
    }
}*/