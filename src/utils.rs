pub use std::f64::consts::PI;
use std::ops::{Add, BitOr, Mul, Sub};

pub type Rad = f64;
pub type Deg = f64;
pub type Pixels = f64;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Point {
    pub x: Pixels,
    pub y: Pixels,
}

impl Point {
    pub fn new(x: Pixels, y: Pixels) -> Self {
        Point { x, y }
    }

    pub fn norm(self) -> Pixels {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn angle(self) -> Rad {
        self.y.atan2(self.x)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<Vector> for Point {
    fn from(vector: Vector) -> Self {
        Self {
            x: vector.x(),
            y: vector.y(),
        }
    }
}

impl Into<sdl2::rect::Point> for Point {
    fn into(self) -> sdl2::rect::Point {
        sdl2::rect::Point::new(self.x as i32, self.y as i32)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub angle: Rad,
    pub norm: Pixels,
}

impl Vector {
    pub fn x(&self) -> Pixels {
        self.angle.cos() * self.norm
    }

    pub fn y(&self) -> Pixels {
        self.angle.sin() * self.norm
    }
}

impl From<Point> for Vector {
    fn from(point: Point) -> Self {
        Vector {
            angle: point.angle(),
            norm: point.norm(),
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector::from(Point::from(self) + Point::from(rhs))
    }
}

impl Mul<Pixels> for Vector {
    type Output = Self;

    fn mul(self, rhs: Pixels) -> Self {
        Vector {
            norm: self.norm * rhs,
            ..self
        }
    }
}

impl BitOr<Rad> for Vector {
    type Output = Self;

    fn bitor(self, rhs: Rad) -> Self {
        Vector {
            angle: -self.angle + 2. * rhs + PI % (2. * PI),
            ..self
        }
    }
}

#[cfg(test)]
mod utils_test {
    use super::*;

    #[test]
    fn test_point_norm() {
        let p1 = Point::new(332., 434.);
        let p2 = Point::new(143., 302.);
        assert_eq!((p2 - p1).norm() as u32, 230);
    }

    #[test]
    fn test_point_angle() {
        assert_eq!(Point::new(1., 0.).angle(), 0.);
        assert_eq!(Point::new(0., 1.).angle(), PI / 2.);
        assert_eq!(Point::new(-1., 0.).angle(), PI);
        assert_eq!(Point::new(0., -1.).angle(), -PI / 2.);
    }

    #[test]
    fn test_collision() {
        let v = Vector {
            norm: 1.0,
            angle: PI / 4.,
        };
        let v = v | (PI / 2.);
        assert_eq!(v.norm, 1.0);
        assert_eq!(v.angle, 7. * PI / 4.);
    }
}
