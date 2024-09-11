mod config;
mod from_doc;
mod blueprint;
mod mustache;
mod directive;
mod http;
mod helpers;
mod ir;
mod json;
mod path;
mod hasher;
mod value;

pub fn is_default<T: Default + Eq>(val: &T) -> bool {
    *val == T::default()
}
