use crate::prelude::*;
///////
// SHIP
///////
pub struct Ship {
    pub height: f32,

    pub position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,

    pub angle: f32,
    pub color: Color,

    pub projectiles: Vec<Projectile>,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            height: 20.0,

            position: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
            velocity: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0),
            angle: 0.0,
            color: BLUE,

            projectiles: Vec::new(),
        }
    }
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

    pub fn update(&mut self) {
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

    pub fn draw(&self) {
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

impl Collidable for Ship {
    fn center(&self) -> Vec2 {
        self.position
    }

    fn radius(&self) -> f32 {
        self.height / 1.5
    }

    fn collision_consequence(&mut self) {
        self.color = RED;
    }
}
