#![warn(clippy::all, clippy::pedantic)]

mod asteroid;
mod collidable;
mod game;
mod input;
mod projectile;
mod ship;
mod prelude {
    pub use crate::asteroid::*;
    pub use crate::collidable::*;
    pub use crate::game::*;
    pub use crate::input::*;
    pub use crate::projectile::*;
    pub use crate::ship::*;
    pub use macroquad::{prelude::*, rand::gen_range};
}
use prelude::*;

#[macroquad::main("Rusteroids")]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
