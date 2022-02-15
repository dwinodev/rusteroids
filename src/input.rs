use crate::prelude::*;

pub fn process_input(game: &mut Game) {
    if is_key_down(KeyCode::Up) {
        game.ship.thrust();
    }

    if is_key_down(KeyCode::Left) {
        game.ship.change_angle(-0.1)
    }

    if is_key_down(KeyCode::Right) {
        game.ship.change_angle(0.1)
    }

    if is_key_down(KeyCode::Space) {
        if game.ship.shot_delay == 0 && game.ship.projectiles.len() < 4 {
            game.ship.shoot();
            game.ship.shot_delay = 10;
        }
    }
}
