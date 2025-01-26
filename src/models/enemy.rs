use super::{Movable, Posture};
use crate::derive_collidable;
use crate::derive_position_movable;
use crate::geometry::Point;

pub struct Enemy {
    pub posture: Posture,
}

derive_position_movable!(Enemy);
derive_collidable!(Enemy, 10.0);

impl Enemy {
    pub fn new(posture: Posture) -> Self {
        Self { posture }
    }

    pub fn update(&mut self, speed: f64, player_position: Point) {
        self.set_direction_toward_point(player_position);
        self.move_by_units(speed);
    }
}
