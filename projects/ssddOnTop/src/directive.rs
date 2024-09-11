use anyhow::Result;
use async_graphql::parser::types::ConstDirective;
use async_graphql::{Name, Pos, Positioned};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_path_to_error::deserialize;

fn pos<A>(a: A) -> Positioned<A> {
    Positioned::new(a, Pos::default())
}

pub trait DirectiveCodec: Sized {
    fn directive_name() -> String;
    fn from_directive(directive: &ConstDirective) -> Result<Self>;
    fn to_directive(&self) -> ConstDirective;
    fn trace_name() -> String {
        format!("@{}", Self::directive_name())
    }
    fn from_directives<'a>(
        directives: impl Iterator<Item = &'a Positioned<ConstDirective>>,
    ) -> Result<Option<Self>> {
        for directive in directives {
            if directive.node.name.node == Self::directive_name() {
                return Self::from_directive(&directive.node).map(Some);
            }
        }
        Ok(None)
    }
}
fn lower_case_first_letter(s: &str) -> String {
    if s.len() <= 2 {
        s.to_lowercase()
    } else if let Some(first_char) = s.chars().next() {
        first_char.to_string().to_lowercase() + &s[first_char.len_utf8()..]
    } else {
        s.to_string()
    }
}

impl<'a, A: Deserialize<'a> + Serialize + 'a> DirectiveCodec for A {
    fn directive_name() -> String {
        lower_case_first_letter(
            std::any::type_name::<A>()
                .split("::")
                .last()
                .unwrap_or_default(),
        )
    }

    fn from_directive(directive: &ConstDirective) -> Result<A> {
        let mut map = Map::new();
        for (k, v) in directive.arguments.iter() {
            let (k, v) = serde_json::to_value(&v.node).map(|v| (k.node.as_str().to_string(), v))?;
            map.insert(k, v);
        }

        Ok(deserialize(Value::Object(map))?)
    }

    fn to_directive(&self) -> ConstDirective {
        let name = Self::directive_name();
        let value = serde_json::to_value(self).unwrap();
        let default_map = &Map::new();
        let map = value.as_object().unwrap_or(default_map);

        let mut arguments = Vec::new();
        for (k, v) in map {
            arguments.push((
                pos(Name::new(k.clone())),
                pos(serde_json::from_value(v.to_owned()).unwrap()),
            ));
        }

        ConstDirective {
            name: pos(Name::new(name)),
            arguments,
        }
    }
}
