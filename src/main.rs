#![warn(clippy::all, clippy::pedantic)]
use macroquad::{prelude::*, rand::gen_range};

#[macroquad::main("Rusteroids")]
async fn main() {
    let mut ship = Ship {
        height: 20.0,

        position: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
        velocity: Vec2::new(0.0, 0.0),
        acceleration: Vec2::new(0.0, 0.0),
        angle: 0.0,
        color: BLUE,

        projectiles: Vec::new(),
    };

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

///////
// SHIP
///////
struct Ship {
    height: f32,

    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,

    angle: f32,
    color: Color,

    projectiles: Vec<Projectile>,
}

impl Ship {
    fn calculate_points(&self) -> (Vec2, Vec2, Vec2) {
        let x: f32 = self.position.x + (self.height * self.angle.cos());
        let y: f32 = self.position.y + (self.height * self.angle.sin());
        let front_point = Vec2::new(x, y);

        let x: f32 = self.position.x + (self.height * (self.angle + 120.0_f32.to_radians()).cos());
        let y: f32 = self.position.y + (self.height * (self.angle + 120.0_f32.to_radians()).sin());
        let back_point_l = Vec2::new(x, y);

        let x: f32 = self.position.x + (self.height * (self.angle + 240.0_f32.to_radians()).cos());
        let y: f32 = self.position.y + (self.height * (self.angle + 240.0_f32.to_radians()).sin());
        let back_point_r = Vec2::new(x, y);

        (front_point, back_point_l, back_point_r)
    }

    fn shoot(&mut self) {
        self.projectiles.push(Projectile::new(self));
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::Up) {
            let x: f32 = self.position.x + (-0.1 * self.angle.cos());
            let y: f32 = self.position.y + (-0.1 * self.angle.sin());
            let direction = Vec2::new(x, y);

            let distance = self.position - direction;

            self.acceleration = distance;
        }

        if is_key_down(KeyCode::Left) {
            self.angle -= 0.05;
        }

        if is_key_down(KeyCode::Right) {
            self.angle += 0.05;
        }

        self.velocity += self.acceleration;
        self.velocity.clamp_length(-2.0, 2.0);
        self.position += self.velocity;

        self.acceleration = Vec2::new(0.0, 0.0);

        if self.position.x < 0.0 {
            self.position.x = screen_width();
        } else if self.position.x > screen_width() {
            self.position.x = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = screen_height();
        } else if self.position.y > screen_height() {
            self.position.y = 0.0;
        }

        if is_key_down(KeyCode::Space) {
            self.shoot();
        }

        for projectile in &mut self.projectiles {
            projectile.update();
        }
    }

    fn draw(&self) {
        let ship_points = self.calculate_points();

        draw_line(
            ship_points.0.x,
            ship_points.0.y,
            ship_points.1.x,
            ship_points.1.y,
            1.0,
            WHITE,
        );
        draw_line(
            ship_points.1.x,
            ship_points.1.y,
            self.position.x,
            self.position.y,
            1.0,
            WHITE,
        );

        draw_line(
            self.position.x,
            self.position.y,
            ship_points.2.x,
            ship_points.2.y,
            1.0,
            WHITE,
        );

        draw_line(
            ship_points.2.x,
            ship_points.2.y,
            ship_points.0.x,
            ship_points.0.y,
            1.0,
            WHITE,
        );

        draw_circle(
            self.position.x,
            self.position.y,
            self.height / 1.5,
            self.color,
        );

        for projectile in &self.projectiles {
            projectile.draw();
        }
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

/////////////
// PROJECTILE
/////////////
struct Projectile {
    size: f32,
    position: Vec2,
    velocity: Vec2,
    color: Color,
    //acceleration: Vec2,
    //angle: f32,
}

impl Projectile {
    fn new(ship: &Ship) -> Self {
        let x: f32 = ship.position.x + (-2.01 * ship.angle.cos());
        let y: f32 = ship.position.y + (-2.01 * ship.angle.sin());
        let direction = Vec2::new(x, y);
        let distance = ship.position - direction;
        Self {
            size: 5.0,
            position: ship.position,
            velocity: distance,
            color: YELLOW,
        }
    }
    fn update(&mut self) {
        self.position += self.velocity;

        println!("{:?}", self.velocity);

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
        draw_circle_lines(self.position.x, self.position.y, self.size, 1.0, WHITE);

        draw_circle(
            self.position.x,
            self.position.y,
            self.size / 1.1,
            self.color,
        );
    }
}
