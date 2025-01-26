use rand::Rng;

use crate::geometry::{Point, Rect};

#[derive(Clone, Default)]
pub struct Posture {
    pub position: Point,
    pub direction: f64,
}

impl Posture {
    pub fn new(position: Point, direction: f64) -> Self {
        Posture {
            position,
            direction,
        }
    }

    pub fn random<R: Rng>(rng: &mut R, bounds: Rect) -> Self {
        let x = rng.gen_range(bounds.origin.x..bounds.origin.x + bounds.width);
        let y = rng.gen_range(bounds.origin.y..bounds.origin.y + bounds.height);

        let position = Point::new(x, y);
        let direction = rng.gen_range(0.0..std::f64::consts::PI * 2.0);

        Posture {
            position,
            direction,
        }
    }

    pub fn reverse_direction(&self) -> Self {
        Posture {
            position: self.position.clone(),
            direction: self.direction + std::f64::consts::PI,
        }
    }
}
