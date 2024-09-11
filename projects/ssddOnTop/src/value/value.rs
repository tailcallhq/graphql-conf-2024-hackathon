use derive_getters::Getters;

#[derive(Clone, Getters)]
pub struct Value {
    serde: serde_json::Value,
    borrowed: serde_json_borrow::Value<'static>,
}

impl Value {
    pub fn new(serde: serde_json::Value) -> Self {
        let borrowed = serde_json_borrow::Value::from(&serde);
        let borrowed = extend_lifetime(borrowed);
        Self { serde, borrowed }
    }
}

fn extend_lifetime<'b>(r: serde_json_borrow::Value<'b>) -> serde_json_borrow::Value<'static> {
    unsafe {
        std::mem::transmute::<serde_json_borrow::Value<'b>, serde_json_borrow::Value<'static>>(r)
    }
}
