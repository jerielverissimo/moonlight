pub mod channels;
pub mod input;
pub mod program;
pub mod render;

pub use channels::*;
pub use input::*;
pub use program::*;
pub(crate) use render::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
