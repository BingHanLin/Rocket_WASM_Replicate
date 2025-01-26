use rand::Rng;

use super::Point;

#[derive(PartialEq, Clone, Copy, Default)]
pub struct Rect {
    pub origin: Point,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            origin: Point::default(),
            width,
            height,
        }
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x >= 0.0 && p.x < self.width && p.y >= 0.0 && p.y < self.height
    }

    pub fn random_x<R: rand::Rng>(&self, rng: &mut R) -> f64 {
        rng.gen_range(self.origin.x..self.origin.x + self.width)
    }

    pub fn random_y<R: rand::Rng>(&self, rng: &mut R) -> f64 {
        rng.gen_range(self.origin.y..self.origin.y + self.height)
    }
}
