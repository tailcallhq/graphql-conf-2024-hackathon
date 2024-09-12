use std::fmt::{Display, Formatter};
use derive_getters::Getters;

pub trait ToInner {
    type Inner;
    fn to_inner(&self) -> Self::Inner;
}

impl ToInner for Value {
    type Inner = serde_json_borrow::Value<'static>;

    fn to_inner(&self) -> Self::Inner {
        self.borrowed.clone()
    }
}

#[derive(Getters, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Value {
    serde: serde_json::Value,
    borrowed: serde_json_borrow::Value<'static>,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serde)
    }
}

impl Value {
    pub fn new(serde: serde_json::Value) -> Self {
        let borrowed = serde_json_borrow::Value::from(&serde);
        let borrowed = extend_lifetime(borrowed);
        Self { serde, borrowed }
    }
    pub fn into_serde(self) -> serde_json::Value {
        self.serde
    }
}

fn extend_lifetime<'b>(r: serde_json_borrow::Value<'b>) -> serde_json_borrow::Value<'static> {
    unsafe {
        std::mem::transmute::<serde_json_borrow::Value<'b>, serde_json_borrow::Value<'static>>(r)
    }
}
