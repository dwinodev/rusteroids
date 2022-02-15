use crate::prelude::*;
pub trait Collidable {
    fn center(&self) -> Vec2;
    fn radius(&self) -> f32;
    fn collision_detected(&self, other: &dyn Collidable) -> bool {
        self.center().distance(other.center()) <= self.radius() + other.radius()
            && self.center().x > 0.0
            && self.center().x < screen_width()
            && self.center().y > 0.0
            && self.center().y < screen_height()
    }
    fn collision_consequence(&mut self);
}
