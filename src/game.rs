use crate::prelude::*;

pub struct Game {
    pub ship: Ship,
    asteroids: Vec<Asteroid>,
    score: u32,
    pub state: GameState,
}

pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

impl Game {
    pub fn new() -> Self {
        let mut asteroids_init = Vec::new();
        for _ in 1..5 {
            // MAGIC NUMBER
            asteroids_init.push(Asteroid::new());
        }
        Self {
            ship: Ship::new(),
            asteroids: asteroids_init,
            score: 0,
            state: GameState::Playing,
        }
    }
    pub fn init(&mut self) {
        self.ship = Ship::new();
        let mut asteroids_init = Vec::new();
        for _ in 1..5 {
            // MAGIC NUMBER
            asteroids_init.push(Asteroid::new());
        }
        self.asteroids = asteroids_init;
        self.score = 0;
    }
    pub fn update(&mut self) {
        process_keys(self);
        match self.state {
            GameState::Playing => {
                self.ship.update();
            }
            GameState::GameOver => {}
            GameState::Menu => {}
        }
        self.ship.update();
        self.update_asteroids();
        self.update_projectiles();
        self.check_game_state();
    }
    fn update_asteroids(&mut self) {
        self.cleanup_asteroids();
        self.asteroids.iter_mut().for_each(|asteroid| {
            asteroid.update();
            if self.ship.collision_detected(asteroid) {
                self.ship.collision_consequence();
                asteroid.collision_consequence();
            }
        });
    }

    fn cleanup_asteroids(&mut self) {
        let mut i: usize = 0;
        while i < self.asteroids.len() {
            if self.asteroids[i].hit {
                let old_size = self.asteroids[i].size;
                let old_position = self.asteroids[i].position;
                self.asteroids.remove(i);
                if old_size > 20.0 {
                    self.asteroids
                        .push(Asteroid::new_after_hit(old_size, old_position));
                    self.asteroids
                        .push(Asteroid::new_after_hit(old_size, old_position));
                }
            } else {
                i += 1;
            }
        }
    }

    fn update_projectiles(&mut self) {
        self.cleanup_projectiles();
        self.ship.projectiles.iter_mut().for_each(|proj| {
            proj.update();
            for asteroid in &mut self.asteroids {
                if proj.collision_detected(asteroid) {
                    proj.collision_consequence();
                    asteroid.collision_consequence();
                    self.score += 1;
                }
            }
        });
    }
    fn cleanup_projectiles(&mut self) {
        let mut i: usize = 0;
        while i < self.ship.projectiles.len() {
            if self.ship.projectiles[i].remove {
                self.ship.projectiles.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn check_game_state(&mut self) {
        if self.ship.lives < 1 {
            self.state = GameState::GameOver;
        }
    }
    pub fn draw(&self) {
        clear_background(BLACK);
        match self.state {
            GameState::Playing => {
                self.ship.draw();
                draw_text(&self.score.to_string(), 50.0, 50.0, 50.0, WHITE);
            }
            GameState::GameOver => {
                draw_text(
                    "GAME OVER",
                    screen_width() / 3.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
                draw_text(
                    "press 'r' to restart",
                    screen_width() / 3.0,
                    screen_height() / 2.0 + 50.0,
                    25.0,
                    WHITE,
                );
            }
            GameState::Menu => {}
        }
        for asteroid in &self.asteroids {
            asteroid.draw();
        }
    }
}
