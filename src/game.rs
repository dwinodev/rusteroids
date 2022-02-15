use crate::prelude::*;

pub struct Game {
    ship: Ship,
    asteroids: Vec<Asteroid>,
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
    }
    pub fn update(&mut self) {
        process_input(&mut self.ship);
        self.ship.update();
        self.update_asteroids();
        self.update_projectiles();
    }
    fn update_asteroids(&mut self) {
        self.cleanup_asteroids();
        self.asteroids.iter_mut().for_each(|asteroid| {
            asteroid.update();
            if self.ship.collision_detected(asteroid) {
                self.ship.collision_consequence();
                asteroid.collision_consequence();
            } else {
                // TO REMOVE
                self.ship.color = BLUE;
                asteroid.color = GREEN;
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
            for asteroid in self.asteroids.iter_mut() {
                if proj.collision_detected(asteroid) {
                    proj.collision_consequence();
                    asteroid.collision_consequence();
                } else {
                    // TO REMOVE
                    proj.color = YELLOW;
                    //asteroid.color = GREEN;
                }
            }
        });
    }
    fn cleanup_projectiles(&mut self) {
        let mut i: usize = 0;
        while i < self.ship.projectiles.len() {
            if self.ship.projectiles[i].hit {
                self.ship.projectiles.remove(i);
            } else {
                i += 1;
            }
        }
    }
    pub fn draw(&self) {
        clear_background(BLACK);
        self.ship.draw();
        for asteroid in &self.asteroids {
            asteroid.draw();
        }
    }
}
