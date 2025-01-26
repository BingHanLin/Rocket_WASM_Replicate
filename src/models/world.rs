use rand::Rng;

use super::{Bullet, Enemy, Particle, Player};
use crate::geometry::Rect;

/// A model that contains the other models and renders them
pub struct World {
    pub player: Player,
    pub particles: Vec<Particle>,
    pub bullets: Vec<Bullet>,
    pub enemies: Vec<Enemy>,
    pub rect: Rect,
}

impl World {
    /// Returns a new world of the given rect
    pub fn new<R: Rng>(rng: &mut R, rect: Rect) -> World {
        World {
            player: Player::random(rng, rect),
            particles: Vec::with_capacity(1000),
            bullets: vec![],
            enemies: vec![],
            rect: rect,
        }
    }
}
