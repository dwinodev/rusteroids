#![warn(clippy::all, clippy::pedantic)]

mod projectile;
mod ship;

mod prelude {
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
    detect_collision(ship, asteroid);
    for projectile in &mut ship.projectiles {
        detect_collision_projectile(projectile, asteroid);
    }
    asteroid.update();
    detect_collision(ship, asteroid);
    for projectile in &mut ship.projectiles {
        detect_collision_projectile(projectile, asteroid);
    }
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

///////////
// ASTEROID
///////////
struct Asteroid {
    size: f32,
    position: Vec2,
    velocity: Vec2,
    color: Color,
    //acceleration: Vec2,
    //angle: f32,
}

impl Asteroid {
    fn new() -> Self {
        let x_pos_rand = gen_range(0.0, screen_width());
        let y_pos_rand = gen_range(0.0, screen_height());
        let x_vel_rand: f32 = gen_range(0.5, 2.5);
        let y_vel_rand: f32 = gen_range(0.5, 2.5);
        Self {
            size: 75.0,
            position: Vec2::new(x_pos_rand, y_pos_rand),
            velocity: Vec2::new(x_vel_rand, y_vel_rand),
            color: GREEN,
        }
    }
    fn update(&mut self) {
        self.position += self.velocity;

        if self.position.x < 0.0 - self.size * 2.0 {
            self.position.x = screen_width() + self.size * 2.0;
        } else if self.position.x > screen_width() + self.size * 2.0 {
            self.position.x = 0.0 - self.size * 2.0;
        }

        if self.position.y < 0.0 - self.size * 2.0 {
            self.position.y = screen_height() + self.size * 2.0;
        } else if self.position.y > screen_height() + self.size * 2.0 {
            self.position.y = 0.0 - self.size * 2.0;
        }
    }

    fn draw(&self) {
        draw_poly_lines(
            self.position.x,
            self.position.y,
            6,
            self.size,
            0.0,
            1.0,
            WHITE,
        );

        draw_circle(
            self.position.x,
            self.position.y,
            self.size / 1.1,
            self.color,
        );
    }
}
