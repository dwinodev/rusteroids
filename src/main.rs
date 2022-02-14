#![warn(clippy::all, clippy::pedantic)]

mod asteroid;
mod collidable;
mod projectile;
mod ship;
mod prelude {
    pub use crate::asteroid::*;
    pub use crate::collidable::*;
    pub use crate::projectile::*;
    pub use crate::ship::*;
    pub use macroquad::{prelude::*, rand::gen_range};
}
use prelude::*;

#[macroquad::main("Rusteroids")]
async fn main() {
    let mut ship = Ship::new();
    let mut asteroid = Asteroid::new();
    loop {
        update(&mut ship, &mut asteroid);
        draw(&ship, &asteroid);
        next_frame().await;
    }
}

fn update(ship: &mut Ship, asteroid: &mut Asteroid) {
    ship.update();
    asteroid.update();
    if ship.collision_detected(asteroid) {
        ship.collision_consequence();
        asteroid.collision_consequence();
    } else {
        ship.color = BLUE;
        asteroid.color = GREEN;
    }
}

fn draw(ship: &Ship, asteroid: &Asteroid) {
    clear_background(BLACK);
    ship.draw();
    asteroid.draw();
}
