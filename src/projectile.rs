use crate::prelude::*;
pub struct Projectile {
    pub size: f32,
    pub position: Vec2,
    velocity: Vec2,
    pub color: Color,
    pub remove: bool,
    pub frame_count: u32,
}

impl Projectile {
    pub fn new(ship: &Ship) -> Self {
        let x: f32 = ship.position.x + (-2.01 * ship.angle.cos());
        let y: f32 = ship.position.y + (-2.01 * ship.angle.sin());
        let direction = Vec2::new(x, y);
        let distance = ship.position - direction;
        Self {
            size: 5.0,
            position: ship.position,
            velocity: distance,
            color: YELLOW,
            remove: false,
            frame_count: 0,
        }
    }
    pub fn update(&mut self) {
        if self.frame_count >= 250 {
            self.remove = true;
        }
        self.position += self.velocity;

        self.frame_count += 1;

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

        // if self.position.x < 0.0 - self.size * 2.0 {
        //     self.position.x = screen_width() + self.size * 2.0;
        // } else if self.position.x > screen_width() + self.size * 2.0 {
        //     self.position.x = 0.0 - self.size * 2.0;
        // }

        // if self.position.y < 0.0 - self.size * 2.0 {
        //     self.position.y = screen_height() + self.size * 2.0;
        // } else if self.position.y > screen_height() + self.size * 2.0 {
        //     self.position.y = 0.0 - self.size * 2.0;
        // }
    }

    pub fn draw(&self) {
        draw_circle_lines(self.position.x, self.position.y, self.size, 1.0, WHITE);

        // draw_circle(
        //     self.position.x,
        //     self.position.y,
        //     self.size / 1.1,
        //     self.color,
        // );
    }
}

impl Collidable for Projectile {
    fn center(&self) -> Vec2 {
        self.position
    }

    fn radius(&self) -> f32 {
        self.size / 1.1
    }
    fn collision_consequence(&mut self) {
        self.remove = true;
    }
}
