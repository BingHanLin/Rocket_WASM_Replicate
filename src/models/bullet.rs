use super::{Movable, Posture};
use crate::derive_collidable;
use crate::derive_position_movable;

pub struct Bullet {
    pub posture: Posture,
}

derive_position_movable!(Bullet);
derive_collidable!(Bullet, 3.0);

impl Bullet {
    pub fn new(posture: Posture) -> Self {
        Self { posture }
    }

    pub fn update(&mut self, units: f64) {
        self.move_by_units(units);
    }
}
