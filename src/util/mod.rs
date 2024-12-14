pub mod vec2d;
pub mod point;

pub mod grid {
    pub mod prelude {
        pub use crate::util::point::*;
        pub use crate::util::vec2d::*;
    }
}