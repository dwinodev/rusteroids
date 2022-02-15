use crate::prelude::*;
pub struct Asteroid {
    pub size: f32,
    pub position: Vec2,
    velocity: Vec2,
    pub color: Color,
}
impl Asteroid {
    pub fn new() -> Self {
        Self {
            size: 75.0,
            position: Vec2::new(
                gen_range(0.0, screen_width()),
                gen_range(0.0, screen_height()),
            ),
            velocity: Vec2::new(gen_range(0.5, 2.5), gen_range(0.5, 2.5)),
            color: GREEN,
        }
    }
    pub fn update(&mut self) {
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
    pub fn draw(&self) {
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

impl Collidable for Asteroid {
    fn center(&self) -> Vec2 {
        self.position
    }

    fn radius(&self) -> f32 {
        self.size / 1.1
    }
    fn collision_consequence(&mut self) {
        self.color = RED;
    }
}
