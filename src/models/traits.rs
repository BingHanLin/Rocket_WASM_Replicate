use crate::geometry::{Point, Rect};

pub trait Position {
    fn x(&self) -> f64;
    fn set_x(&mut self, x: f64);

    fn y(&self) -> f64;
    fn set_y(&mut self, y: f64);

    fn position(&self) -> Point {
        Point::new(self.x(), self.y())
    }
}

pub trait Movable: Position {
    // Direction of the object, measured in radians from the positive x-axis to the position.
    fn direction(&self) -> f64;

    fn set_direction(&mut self, direction: f64);

    fn set_direction_toward_point(&mut self, point: Point) {
        let m = (self.y() - point.y) / (self.x() - point.x);

        if point.x > self.x() {
            self.set_direction(m.atan());
        } else {
            self.set_direction(m.atan() + std::f64::consts::PI);
        };
    }

    fn move_by_units(&mut self, units: f64) {
        let dx = self.direction().cos() * units;
        let dy = self.direction().sin() * units;

        self.set_x(self.x() + dx);
        self.set_y(self.y() + dy);
    }

    // Moves the object by a certain amount of units, but keeps it within the bounds.
    // If the object would go out of bounds, it will be moved to the opposite side of the bounds.
    fn move_within_bounds(&mut self, units: f64, bounds: Rect) {
        self.move_by_units(units);

        if self.x() < bounds.origin.x {
            self.set_x(bounds.origin.x + bounds.width);
        } else if self.x() > bounds.origin.x + bounds.width {
            self.set_x(bounds.origin.x);
        }

        if self.y() < bounds.origin.y {
            self.set_y(bounds.origin.y + bounds.height);
        } else if self.y() > bounds.origin.y + bounds.height {
            self.set_y(bounds.origin.y);
        }
    }
}

pub trait Collidable: Position {
    fn radius(&self) -> f64;

    fn diameter(&self) -> f64 {
        self.radius() * 2.0
    }

    fn collides_with<T: Collidable>(&self, other: &T) -> bool {
        self.position().squared_distance(&other.position())
            <= (self.radius() + other.radius()).powi(2)
    }
}

#[macro_export]
macro_rules! derive_position_movable {
    ($t:ty) => {
        impl super::traits::Position for $t {
            fn x(&self) -> f64 {
                self.posture.position.x
            }

            fn set_x(&mut self, x: f64) {
                self.posture.position.x = x;
            }

            fn y(&self) -> f64 {
                self.posture.position.y
            }

            fn set_y(&mut self, y: f64) {
                self.posture.position.y = y;
            }
        }

        impl super::traits::Movable for $t {
            fn direction(&self) -> f64 {
                self.posture.direction
            }

            fn set_direction(&mut self, direction: f64) {
                self.posture.direction = direction;
            }
        }
    };
}

#[macro_export]
macro_rules! derive_collidable {
    ($t:ty, $radius:expr) => {
        impl super::traits::Collidable for $t {
            fn radius(&self) -> f64 {
                $radius
            }
        }
    };
}
