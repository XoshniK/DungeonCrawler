mod render;
mod player;
mod enemy;
mod moving_randomly;
mod wants_to_move;

pub mod prelude {
    pub use super::render::*;
    pub use super::player::*;
    pub use super::enemy::*;
    pub use super::moving_randomly::*;
    pub use super::wants_to_move::*;
}
