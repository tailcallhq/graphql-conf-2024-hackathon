use std::str::FromStr;

#[derive(Clone, Debug, Default)]
pub struct HeaderMap(hyper::header::HeaderMap);

impl Into<hyper::header::HeaderMap> for HeaderMap {
    fn into(self) -> hyper::header::HeaderMap {
        self.0
    }
}

/*impl Into<reqwest::header::HeaderMap> for HeaderMap {
    fn into(self) -> reqwest::header::HeaderMap {
        let mut map = reqwest::header::HeaderMap::new();
        for (k, v) in self.0.iter() {
            map.insert(k.as_str().parse().unwrap(),v.as_bytes().to_vec().into());
        }
        map
    }
}*/

impl From<hyper::header::HeaderMap> for HeaderMap {
    fn from(value: http::HeaderMap) -> Self {
        Self(value)
    }
}
impl From<reqwest::header::HeaderMap> for HeaderMap {
    fn from(value: reqwest::header::HeaderMap) -> Self {
        let mut map = hyper::header::HeaderMap::new();
        for (k, v) in value.iter() {
            map.insert(
                hyper::header::HeaderName::from_str(k.as_str()).unwrap(),
                hyper::header::HeaderValue::from_str(v.to_str().unwrap()).unwrap(),
            );
        }
        Self(map)
    }
}
