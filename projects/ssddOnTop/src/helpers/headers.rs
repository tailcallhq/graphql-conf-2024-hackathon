use crate::config::KeyValue;
use crate::mustache::model::Mustache;
use anyhow::Result;
use reqwest::header::HeaderName;

pub type MustacheHeaders = Vec<(HeaderName, Mustache)>;

pub fn to_mustache_headers(headers: &[KeyValue]) -> Result<MustacheHeaders> {
    let mut ans = vec![];
    for key_value in headers {
        let name = HeaderName::from_bytes(key_value.key.as_bytes())?;
        let value = Mustache::parse(key_value.value.as_str());
        let header = (name, value);
        ans.push(header);
    }
    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::to_mustache_headers;
    use crate::config::KeyValue;
    use crate::mustache::model::Mustache;
    use anyhow::Result;
    use reqwest::header::HeaderName;

    #[test]
    fn valid_headers() -> Result<()> {
        let input: Vec<KeyValue> = serde_json::from_str(
            r#"[{"key": "a", "value": "str"}, {"key": "b", "value": "123"}]"#,
        )?;

        let headers = to_mustache_headers(&input)?;

        assert_eq!(
            headers,
            vec![
                (HeaderName::from_bytes(b"a")?, Mustache::parse("str")),
                (HeaderName::from_bytes(b"b")?, Mustache::parse("123"))
            ]
        );

        Ok(())
    }

    #[test]
    fn not_valid_due_to_utf8() {
        let input: Vec<KeyValue> =
            serde_json::from_str(r#"[{"key": "😅", "value": "str"}, {"key": "b", "value": "🦀"}]"#)
                .unwrap();
        let error = to_mustache_headers(&input).unwrap_err();

        // HeaderValue should be parsed just fine despite non-visible ascii symbols
        // range see https://github.com/hyperium/http/issues/519
        assert_eq!(
            error.to_string(),
            r"Validation Error
• invalid HTTP header name [😅]
"
        );
    }
}
