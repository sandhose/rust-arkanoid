use std::ops::{Add, Mul};

use ball;
use traits;

#[derive(Clone, Copy)]
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

pub type CollisionResult = Option<Point>;

pub fn distance(p1: &Point, p2: &Point) -> f32 {
    let l = (p1.x - p2.x).abs();
    let h = (p1.y - p2.y).abs();
    return (l * l + h * h).sqrt();
}

// Computes the vector according to which the ball
// bounces if it hits the angle of a brick
// Uses the coordinates of both the angle and the ball
pub fn angle_clsn_bnce_vect(angle: &Point, ball: &ball::Ball) -> Point {
    Point {
        x: ball.speed.y,
        y: ball.speed.x,
    }
}

fn x_col(xg: Pixels, xd: Pixels, yh: Pixels, yb: Pixels, ball: &ball::Ball) -> CollisionResult {
    if (ball.position.x + ball::BALL_RADIUS) > xg
        && ball.position.x < (xd + ball::BALL_RADIUS)
        && ball.position.y > yh
        && ball.position.y < yb
    {
        return Some(Point {
            x: -1.0 * ball.speed.x,
            y: 1.0 * ball.speed.y,
        });
    }
    None
}

fn y_col(xg: Pixels, xd: Pixels, yh: Pixels, yb: Pixels, ball: &ball::Ball) -> CollisionResult {
    if (ball.position.y + ball::BALL_RADIUS) > yh
        && ball.position.y < (yb + ball::BALL_RADIUS)
        && ball.position.x > xg
        && ball.position.x < xd
    {
        return Some(Point {
            x: 1.0 * ball.speed.x,
            y: -1.0 * ball.speed.y,
        });
    }
    None
}

fn angle_col(xg: Pixels, xd: Pixels, yh: Pixels, yb: Pixels, ball: &ball::Ball) -> CollisionResult {
    let corners = [
        Point { x: xg, y: yh },
        Point { x: xg, y: yb },
        Point { x: xd, y: yh },
        Point { x: xd, y: yb },
    ];
    for corner in corners.iter() {
        if distance(corner, &ball.position) < ball::BALL_RADIUS {
            let bounce_vector = angle_clsn_bnce_vect(corner, &ball);
            return Some(bounce_vector);
        }
    }
    None
}

pub fn collision<T: traits::Collisionable>(obj: &T, ball: &ball::Ball) -> CollisionResult {
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
