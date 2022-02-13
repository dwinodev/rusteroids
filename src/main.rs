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
    if ship.collision_detected(asteroid) {
        ship.color = RED;
        asteroid.color = RED;
    } else {
        ship.color = BLUE;
        asteroid.color = GREEN;
    }
    //detect_collision(ship, asteroid);
    //for projectile in &mut ship.projectiles {
    //    detect_collision_projectile(projectile, asteroid);
    //}
    asteroid.update();
    //if asteroid.collision_detected(ship) {}
    // detect_collision(ship, asteroid);
    // for projectile in &mut ship.projectiles {
    //     detect_collision_projectile(projectile, asteroid);
    // }
}

fn draw(ship: &Ship, asteroid: &Asteroid) {
    clear_background(BLACK);

    ship.draw();
    asteroid.draw();
}

fn collision_detected(ship: &Ship, asteroid: &Asteroid) -> bool {
    ship.position.distance(asteroid.position) < (ship.height / 1.5) + (asteroid.size / 1.1)
}
fn collision_detected_projectile(proj: &Projectile, asteroid: &Asteroid) -> bool {
    proj.position.distance(asteroid.position) < (proj.size) + (asteroid.size / 1.1)
}

fn detect_collision(ship: &mut Ship, asteroid: &mut Asteroid) {
    if collision_detected(ship, asteroid) {
        ship.color = RED;
        asteroid.color = RED;
    } else {
        ship.color = BLUE;
        asteroid.color = GREEN;
    }
}
fn detect_collision_projectile(proj: &mut Projectile, asteroid: &mut Asteroid) {
    if collision_detected_projectile(proj, asteroid) {
        proj.color = RED;
        asteroid.color = RED;
    } else {
        proj.color = YELLOW;
        asteroid.color = GREEN;
    }
}
