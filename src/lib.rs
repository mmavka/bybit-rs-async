// #[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
extern crate serde_qs as qs;

mod client;
pub mod errors;
pub mod config;
pub mod api;
pub mod rest_model;
pub mod util;
pub mod market;

#[derive(Clone)]
pub enum Category {
    Spot,
    Linear,
    Inverse,
    Option
}
