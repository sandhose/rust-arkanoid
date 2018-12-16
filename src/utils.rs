use std::f64::consts::PI;
use std::ops::{Add, Mul};

use ball;
use traits;

pub type Rad = f64;
pub type Deg = f64;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
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

pub type Pixels = f32;

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

pub fn distance(p1: &Point, p2: &Point) -> f32 {
    let l = (p1.x - p2.x).abs();
    let h = (p1.y - p2.y).abs();
    return (l * l + h * h).sqrt();
}

// Computes the vector according to which the ball
// bounces if it hits the angle of a brick
// Uses the coordinates of both the angle and the ball
pub fn angle_clsn_bnce_vect(angle: &Point, ball: &ball::Ball) -> Rad {
    if ball.position.x < angle.x && ball.position.y < angle.y {
        return (PI / 2.0) - (ball.angle + PI);
    } else if ball.position.x < angle.x && ball.position.y >= angle.y {
        return -(ball.angle + (PI / 2.0));
    } else if ball.position.x >= angle.x && ball.position.y < angle.y {
        return PI - (ball.angle - (3.0 * PI / 2.0));
    } else
    /*ball.position.x >= angle.x
    && ball.position.y >= angle.y*/
    {
        return (3.0 * PI / 2.0) - (ball.angle - (2.0 * PI));
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
            angle: PI - (ball.angle - 2.0 * PI),
            speed: ball.speed,
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
            angle: -ball.angle,
            speed: ball.speed,
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
                speed: ball.speed,
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
