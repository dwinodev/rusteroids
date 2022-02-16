use crate::prelude::*;

pub fn process_keys(game: &mut Game) {
    match game.state {
        State::Menu => {
            if is_key_pressed(KeyCode::R) {
                game.init();
                game.state = State::Playing;
            }
        }
        State::Playing => {
            if is_key_down(KeyCode::Up) {
                game.ship.thrust();
                if game.ship.thrust_sound_delay == 0 {
                    let mut params = PlaySoundParams::default();
                    params.volume /= 2.0;
                    play_sound(game.sounds[2], params);
                    game.ship.thrust_sound_delay = 5;
                }
            }

            if is_key_down(KeyCode::Left) {
                game.ship.change_angle(-0.1);
            }

            if is_key_down(KeyCode::Right) {
                game.ship.change_angle(0.1);
            }

            if is_key_pressed(KeyCode::Space)
                && game.ship.shot_delay == 0
                && game.ship.projectiles.len() < 4
            {
                game.ship.shoot();
                game.ship.shot_delay = 10;
                play_sound_once(game.sounds[0]);
            }
        }
        State::GameOver => {
            if is_key_down(KeyCode::R) {
                game.init();
                game.state = State::Playing;
            }
        }
    }
}
