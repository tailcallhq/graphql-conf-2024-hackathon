use crate::endpoint::Endpoint;
use crate::mustache::model::Mustache;
use std::hash::Hash;
use hyper::Method;

#[derive(Debug, Clone)]
pub struct RequestTemplate {
    pub root_url: Mustache,
    pub query: Vec<Query>,
    pub method: Method,
    // pub headers: MustacheHeaders,
}

#[derive(Debug, Clone)]
pub struct Query {
    pub key: String,
    pub value: Mustache,
    pub skip_empty: bool,
}

impl TryFrom<Endpoint> for RequestTemplate {
    type Error = anyhow::Error;
    fn try_from(endpoint: Endpoint) -> anyhow::Result<Self> {
        let path = Mustache::parse(endpoint.path.as_str());
        let query = endpoint
            .query
            .iter()
            .map(|(k, v, skip)| {
                Ok(Query {
                    key: k.as_str().to_string(),
                    value: Mustache::parse(v.as_str()),
                    skip_empty: *skip,
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        let method = endpoint.method.clone().to_hyper();
        /*        let headers = endpoint
        .headers
        .iter()
        .map(|(k, v)| Ok((k.clone(), Mustache::parse(v.to_str()?))))
        .collect::<anyhow::Result<Vec<_>>>()?;*/

        Ok(Self {
            root_url: path,
            query,
            method,
            // headers,
        })
    }
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
