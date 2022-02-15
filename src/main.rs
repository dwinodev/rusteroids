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
    let mut asteroids = Vec::new();
    for _ in 1..5 {
        asteroids.push(Asteroid::new());
    }
    loop {
        update(&mut ship, &mut asteroids);
        draw(&ship, &asteroids);
        next_frame().await;
    }
}
fn update(ship: &mut Ship, asteroids: &mut Vec<Asteroid>) {
    ship.update();
    for asteroid in asteroids.iter_mut() {
        asteroid.update();
        if ship.collision_detected(asteroid) {
            ship.collision_consequence();
            asteroid.collision_consequence();
        } else {
            // TO REMOVE
            ship.color = BLUE;
            asteroid.color = GREEN;
        }
    }
    for proj in ship.projectiles.iter_mut() {
        proj.update();
        for asteroid in asteroids.iter_mut() {
            if proj.collision_detected(asteroid) {
                proj.collision_consequence();
                asteroid.collision_consequence();
            } else {
                // TO REMOVE
                proj.color = YELLOW;
                //asteroid.color = GREEN;
            }
        }
    }
}
fn draw(ship: &Ship, asteroids: &[Asteroid]) {
    clear_background(BLACK);
    ship.draw();
    for asteroid in asteroids {
        asteroid.draw();
    }
}
