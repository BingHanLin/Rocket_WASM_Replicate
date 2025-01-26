use rand::Rng;

use super::{Movable, Position, Posture};
use crate::derive_collidable;
use crate::derive_position_movable;
use crate::geometry::{Point, Rect};

pub struct Player {
    pub posture: Posture,
}

derive_position_movable!(Player);
derive_collidable!(Player, 6.0);

pub const POLYGON: &'static [[f64; 2]] = &[[0.0, -8.0], [20.0, 0.0], [0.0, 8.0]];

impl Player {
    pub fn random<R: Rng>(rng: &mut R, bounds: Rect) -> Self {
        Self {
            posture: Posture::random(rng, bounds),
        }
    }

    pub fn front(&self) -> Point {
        Point::new(POLYGON[1][0], POLYGON[1][1])
            .rotate_to_origin(self.direction())
            .translate(&self.position())
    }
}
