pub mod channels;
pub mod color;
pub mod commands;
pub mod components;
pub mod input;
pub mod program;
pub mod render;

pub use channels::*;
use input::*;
pub use program::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
