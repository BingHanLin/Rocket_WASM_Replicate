use std::thread::sleep_ms;

use pcg_rand::seeds::PcgSeeder;
use pcg_rand::Pcg32Basic;
use rand::SeedableRng;

use crate::geometry::Rect;
use crate::models::{Position, World};

pub struct GameState {
    pub world: World,
    pub score: u32,
}

impl GameState {
    pub fn new(rect: Rect) -> Self {
        let mut rng = Pcg32Basic::from_seed(PcgSeeder::seed(45));
        GameState {
            world: World::new(&mut rng, rect),
            score: 0,
        }
    }

    pub fn reset(&mut self) {
        let mut rng = Pcg32Basic::from_seed(PcgSeeder::seed(42));

        self.world.player.set_x(self.world.rect.random_x(&mut rng));
        self.world.player.set_y(self.world.rect.random_y(&mut rng));

        self.score = 0;

        self.world.enemies.clear();
        self.world.bullets.clear();
        self.world.particles.clear();
    }
}

// impl GameState {
//     pub fn new(rect: Rect) -> GameState {
//         GameState { world, score: 0 }
//     }
// }
