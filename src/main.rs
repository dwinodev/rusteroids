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
    process_input(ship);
    ship.update();
    update_asteroids(asteroids, ship);
    update_projectiles(ship, asteroids);
}

fn update_asteroids(asteroids: &mut Vec<Asteroid>, ship: &mut Ship) {
    cleanup_asteroids(asteroids);
    asteroids.iter_mut().for_each(|asteroid| {
        asteroid.update();
        if ship.collision_detected(asteroid) {
            ship.collision_consequence();
            asteroid.collision_consequence();
        } else {
            // TO REMOVE
            ship.color = BLUE;
            asteroid.color = GREEN;
        }
    });
}

fn cleanup_asteroids(asteroids: &mut Vec<Asteroid>) {
    let mut i: usize = 0;
    while i < asteroids.len() {
        if asteroids[i].hit {
            let old_size = asteroids[i].size;
            let old_position = asteroids[i].position;
            asteroids.remove(i);
            if old_size > 20.0 {
                asteroids.push(Asteroid::new_after_hit(old_size, old_position));
                asteroids.push(Asteroid::new_after_hit(old_size, old_position));
            }
        } else {
            i += 1;
        }
    }
}

fn update_projectiles(ship: &mut Ship, asteroids: &mut Vec<Asteroid>) {
    cleanup_projectiles(ship);
    ship.projectiles.iter_mut().for_each(|proj| {
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
    });
}
fn cleanup_projectiles(ship: &mut Ship) {
    let mut i: usize = 0;
    while i < ship.projectiles.len() {
        if ship.projectiles[i].hit {
            ship.projectiles.remove(i);
        } else {
            i += 1;
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
