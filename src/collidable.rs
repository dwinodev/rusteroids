use crate::prelude::*;

pub trait Collidable {
    fn center(&self) -> Vec2;
    fn radius(&self) -> f32;
    fn collision_detected(&self, other: &dyn Collidable) -> bool;
}
