use crate::prelude::*;
/////////////
// PROJECTILE
/////////////
pub struct Projectile {
    pub size: f32,
    pub position: Vec2,
    velocity: Vec2,
    pub color: Color,
    //acceleration: Vec2,
    //angle: f32,
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
        }
    }
    pub fn update(&mut self) {
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

    pub fn draw(&self) {
        draw_circle_lines(self.position.x, self.position.y, self.size, 1.0, WHITE);

        draw_circle(
            self.position.x,
            self.position.y,
            self.size / 1.1,
            self.color,
        );
    }
}
