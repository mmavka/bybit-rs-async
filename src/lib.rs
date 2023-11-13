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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
