use super::{Movable, Posture};
use crate::derive_position_movable;
use crate::geometry::Point;

/// A model representing a particle
///
/// Particles are visible objects that have a time to live and move around
/// in a given direction until their time is up. They are spawned when the
/// player or an enemy is killed
pub struct Particle {
    pub posture: Posture,
    pub time_out: f64,
}

derive_position_movable!(Particle);

impl Particle {
    pub fn new(posture: Posture, time_out: f64) -> Self {
        Self { posture, time_out }
    }

    pub fn update(&mut self, elapsed_time: f64) {
        self.time_out -= elapsed_time;
        let speed = 500.0 * self.time_out * self.time_out;
        self.move_by_units(speed * elapsed_time);
    }
}
