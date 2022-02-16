use crate::prelude::*;
pub struct Ship {
    pub height: f32,

    pub position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,

    pub angle: f32,
    pub color: Color,

    pub projectiles: Vec<Projectile>,

    pub shot_delay: u32,

    pub lives: i8,
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

            shot_delay: 0,

            lives: 5,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            height: 0.0,
            position: Vec2::new(-1000.0, -1000.0),
            velocity: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0),
            angle: 0.0,
            color: BLUE,
            projectiles: Vec::new(),
            shot_delay: 0,
            lives: 0,
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

    pub fn thrust(&mut self) {
        let x: f32 = self.position.x + (-0.075 * self.angle.cos());
        let y: f32 = self.position.y + (-0.075 * self.angle.sin());
        let direction = Vec2::new(x, y);

        let distance = self.position - direction;

        self.acceleration = distance;
    }

    pub fn change_angle(&mut self, delta_angle: f32) {
        self.angle += delta_angle;
    }

    pub fn shoot(&mut self) {
        self.projectiles.push(Projectile::new(self));
    }

    pub fn update(&mut self) {
        if self.shot_delay > 0 {
            self.shot_delay -= 1;
        }

        self.velocity += self.acceleration;
        self.velocity.clamp_length(-0.2, 0.2);
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

        // draw_circle(
        //     self.position.x,
        //     self.position.y,
        //     self.height / 1.5,
        //     self.color,
        // );

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
        self.position = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        self.velocity = Vec2::new(0.0, 0.0);
        self.angle = 0.0;
        self.lives -= 1;
    }
}
