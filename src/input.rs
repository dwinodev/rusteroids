use crate::prelude::*;

pub fn process_input(ship: &mut Ship) {
    if is_key_down(KeyCode::Up) {
        ship.thrust();
    }

    if is_key_down(KeyCode::Left) {
        ship.change_angle(-0.1)
    }

    if is_key_down(KeyCode::Right) {
        ship.change_angle(0.1)
    }

    if is_key_down(KeyCode::Space) {
        ship.shoot();
    }
}
