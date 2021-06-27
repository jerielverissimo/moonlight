#![feature(capture_disjoint_fields)]
#![feature(min_type_alias_impl_trait)]

pub mod components;
pub mod core;

pub use crate::core::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
