use crate::prelude::*;

pub trait Collidable {
    fn center(&self) -> Vec2;
    fn radius(&self) -> f32;
    fn collision_detected(&self, other: &dyn Collidable) -> bool {
        self.center().distance(other.center()) <= self.radius() + other.radius()
    }
    fn collision_consequence(&mut self);
}
