mod blueprint;
mod config;
mod directive;
mod endpoint;
mod from_doc;
mod hasher;
mod helpers;
mod http;
mod ir;
mod json;
mod mustache;
mod path;
pub mod run;
mod value;

pub fn is_default<T: Default + Eq>(val: &T) -> bool {
    *val == T::default()
}
