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
    pub use macroquad::{audio::*, prelude::*, rand::gen_range};
}
use prelude::*;

#[macroquad::main("Rusteroids")]
async fn main() {
    let mut game = Game::new();

    let fire_sound = load_sound("assets/fire.wav").await.unwrap();
    let explosion_sound = load_sound("assets/explosion.wav").await.unwrap();
    let thrust_sound = load_sound("assets/thrust.wav").await.unwrap();
    let game_over_sound = load_sound("assets/game_over.wav").await.unwrap();
    game.sounds.push(fire_sound);
    game.sounds.push(explosion_sound);
    game.sounds.push(thrust_sound);
    game.sounds.push(game_over_sound);

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
