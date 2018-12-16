use std::f64::consts::PI;
use std::ops::{Add, Mul};

use ball;
use traits;

pub type Rad = f64;
pub type Deg = f64;
pub type Pixels = f64;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Point {
    pub x: Pixels,
    pub y: Pixels,
}

impl Point {
    fn norm(self) -> Pixels {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn angle(self) -> Rad {
        self.y.atan2(self.x)
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
            y: vector.y()
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vector {
    pub angle: Rad,
    pub norm: Pixels,
}

impl Vector {
    fn x(&self) -> Pixels {
        self.angle.sin() * self.norm
    }

    fn y(&self) -> Pixels {
        self.angle.cos() * self.norm
    }
}

impl From<Point> for Vector {
    fn from(point: Point) -> Self {
        Vector { angle: point.angle(), norm: point.norm() }
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
        Vector { angle: self.angle, norm: self.norm * rhs }
    }
}

pub struct CollisionResult {
    pub angle: Rad,
    pub speed: Pixels,
}

impl Mul for CollisionResult {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            angle: self.angle * rhs.angle,
            speed: self.speed * rhs.speed,
        }
    }
}

pub fn distance(p1: &Point, p2: &Point) -> Pixels {
    let l = (p1.x - p2.x).abs();
    let h = (p1.y - p2.y).abs();
    return (l * l + h * h).sqrt();
}

// Computes the vector according to which the ball
// bounces if it hits the angle of a brick
// Uses the coordinates of both the angle and the ball
pub fn angle_clsn_bnce_vect(corner: &Point, ball: &ball::Ball) -> Rad {
    if ball.position.x < corner.x && ball.position.y < corner.y {
        return (PI / 2.0) - (ball.velocity.angle + PI);
    } else if ball.position.x < corner.x && ball.position.y >= corner.y {
        return -(ball.velocity.angle + (PI / 2.0));
    } else if ball.position.x >= corner.x && ball.position.y < corner.y {
        return PI - (ball.velocity.angle - (3.0 * PI / 2.0));
    } else
    /*ball.position.x >= corner.x
    && ball.position.y >= corner.y*/
    {
        return (3.0 * PI / 2.0) - (ball.velocity.angle - (2.0 * PI));
    }
}

fn x_col(
    xg: Pixels,
    xd: Pixels,
    yh: Pixels,
    yb: Pixels,
    ball: &ball::Ball,
) -> Option<CollisionResult> {
    if (ball.position.x + ball::BALL_RADIUS) > xg
        && ball.position.x < (xd + ball::BALL_RADIUS)
        && ball.position.y > yh
        && ball.position.y < yb
    {
        return Some(CollisionResult {
            angle: PI - (ball.velocity.angle - 2.0 * PI),
            speed: ball.velocity.norm,
        });
    }
    None
}

fn y_col(
    xg: Pixels,
    xd: Pixels,
    yh: Pixels,
    yb: Pixels,
    ball: &ball::Ball,
) -> Option<CollisionResult> {
    if (ball.position.y + ball::BALL_RADIUS) > yh
        && ball.position.y < (yb + ball::BALL_RADIUS)
        && ball.position.x > xg
        && ball.position.x < xd
    {
        return Some(CollisionResult {
            angle: -ball.velocity.angle,
            speed: ball.velocity.norm,
        });
    }
    None
}

fn angle_col(
    xg: Pixels,
    xd: Pixels,
    yh: Pixels,
    yb: Pixels,
    ball: &ball::Ball,
) -> Option<CollisionResult> {
    let corners = [
        Point { x: xg, y: yh },
        Point { x: xg, y: yb },
        Point { x: xd, y: yh },
        Point { x: xd, y: yb },
    ];
    for corner in corners.iter() {
        if distance(corner, &ball.position) < ball::BALL_RADIUS {
            let bounce_angle = angle_clsn_bnce_vect(corner, &ball);
            return Some(CollisionResult {
                angle: bounce_angle,
                speed: ball.velocity.norm,
            });
        }
    }
    None
}

pub fn collision<T: traits::Collisionable>(obj: &T, ball: &ball::Ball) -> Option<CollisionResult> {
    let (xg, xd) = obj.get_x();
    let (yh, yb) = obj.get_y();
    x_col(xg, xd, yh, yb, &ball)
        .or_else(|| y_col(xg, xd, yh, yb, &ball))
        .or_else(|| angle_col(xg, xd, yh, yb, &ball))
}

#[cfg(test)]
mod utils_test {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = (332, 434);
        let p2 = (143, 302);
        assert_eq!(distance(p1, p2), 230);
    }
}
