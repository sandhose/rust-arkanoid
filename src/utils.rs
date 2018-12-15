use ball;
use traits;

pub struct Point {pub x: f32, pub y: f32}
pub type Pixels = f32;

pub struct CollisionResult {
    pub collided: bool,
    pub collision_vector: Point,
}

impl CollisionResult {
    fn no_bounce(ball: &ball::Ball) -> CollisionResult {
        return CollisionResult {
            collided: false,
            collision_vector: Point {
                x: 1.0 * ball.speed.x,
                y: 1.0 * ball.speed.y,
            }
        }
    }
}

pub fn distance(p1: &Point, p2: &Point) -> f32 {
    let l = (p1.x - p2.x).abs();
    let h = (p1.y - p2.y).abs();
    return (l*l + h*h).sqrt();
}

// Computes the vector according to which the ball
// bounces if it hits the angle of a brick
// Uses the coordinates of both the angle and the ball
pub fn angle_clsn_bnce_vect(angle: &Point, ball: &ball::Ball) -> Point {
    return Point {
        x: ball.speed.y,
        y: ball.speed.x,
    };
}

fn x_col(xg: Pixels, xd: Pixels, yh: Pixels, yb: Pixels, ball: &ball::Ball)
    -> CollisionResult
{
    if (ball.position.x + ball::BALL_RADIUS) > xg &&
       ball.position.x < (xd + ball::BALL_RADIUS) &&
       ball.position.y > yh && ball.position.y < yb
    {
        return CollisionResult {
            collided: true,
            collision_vector: Point {
                x: -1.0 * ball.speed.x,
                y: 1.0 * ball.speed.y,
            },
        };
    }
    return CollisionResult::no_bounce(&ball);
}

fn y_col(xg: Pixels, xd: Pixels, yh: Pixels, yb: Pixels, ball: &ball::Ball)
    -> CollisionResult
{
    if (ball.position.y + ball::BALL_RADIUS) > yh &&
       ball.position.y < (yb + ball::BALL_RADIUS) &&
       ball.position.x > xg && ball.position.x < xd
    {
        return CollisionResult {
            collided: true,
            collision_vector: Point {
                x: 1.0 * ball.speed.x,
                y: -1.0 * ball.speed.y,
            },
        };
    }
    return CollisionResult::no_bounce(&ball);
}

fn angle_col(xg: Pixels, xd: Pixels, yh: Pixels, yb: Pixels, ball: &ball::Ball)
    -> CollisionResult
{
    let corners = [
        Point {x: xg, y: yh},
        Point {x: xg, y: yb},
        Point {x: xd, y: yh},
        Point {x: xd, y: yb},
    ];
    for corner in corners.iter() {
        if distance(corner, &ball.position) < ball::BALL_RADIUS {
            let bounce_vector = angle_clsn_bnce_vect(corner, &ball); 
            return CollisionResult {
                collided: true,
                collision_vector: bounce_vector,
            };
        }
    }
    return CollisionResult::no_bounce(&ball);
}

pub fn collision<T: traits::Collisionable>(obj: &T, ball: &ball::Ball)
    -> CollisionResult
{
    let (xg, xd) = obj.get_x();
    let (yh, yb) = obj.get_y();
    let collision = x_col(xg, xd, yh, yb, &ball);
    if collision.collided { return collision; };
    let collision = y_col(xg, xd, yh, yb, &ball);
    if collision.collided { return collision; };
    let collision = angle_col(xg, xd, yh, yb, &ball);
    if collision.collided { return collision; };
    return CollisionResult::no_bounce(&ball);
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
