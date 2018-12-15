use ball;
use traits::{Collisionable, Renderable};
use utils;

const WALL_THICKNESS: utils::Pixels = 10.0;

pub struct Wall {
    origin: utils::Point,
    limits: utils::Point,
    bounce_direction: utils::Point,
}

impl Wall {
    // Admettant un rectangle :
    // p1 - p2
    // |    |
    // p3 - p4
    pub fn get_boundaries(&self) -> (utils::Point, utils::Point, utils::Point, utils::Point) {
        let p1 = utils::Point {x: self.origin.x, y: self.origin.y};
        let p2 = utils::Point {x: self.origin.x, y: self.limits.y};
        let p3 = utils::Point {x: self.limits.x, y: self.origin.y};
        let p4 = utils::Point {x: self.limits.x, y: self.limits.y};
        return (p1, p2, p3, p4);
    }
}

impl Collisionable for Wall {
    fn collides(&self, ball: &ball::Ball) -> utils::CollisionResult {
        // TODO : Correct collision detection for walls
        if ball.position.x < self.limits.x &&
           ball.position.y < self.limits.y
        {
            return utils::CollisionResult {
                collided: true,
                collision_vector: utils::Point {
                    x: 1.0 * self.bounce_direction.x,
                    y: 1.0 * self.bounce_direction.y,
                },
            };
        }
        return utils::CollisionResult {
            collided: false,
            collision_vector: utils::Point {
                x: 1.0,
                y: 1.0,
            }
        }
    }
}
// TODO :
// impl Renderable for Wall

mod WallFactory {
    use super::*;

    fn top_wall(width: utils::Pixels) -> Wall {
        return Wall {
            origin: utils::Point {x: 0.0, y: 0.0},
            limits: utils::Point {x: width, y: WALL_THICKNESS},
            bounce_direction: utils::Point {
                x: 1.0,
                y: -1.0
            },
        };
    }
    fn left_wall(height: utils::Pixels) -> Wall {
        return Wall {
            origin: utils::Point {x: 0.0, y: 0.0},
            limits: utils::Point {x: WALL_THICKNESS, y: height},
            bounce_direction: utils::Point {
                x: -1.0,
                y: 1.0
            },
        };
    }
    fn right_wall(height: utils::Pixels, width: utils::Pixels) -> Wall {
        return Wall {
            origin: utils::Point {x: width - WALL_THICKNESS, y: 0.0},
            limits: utils::Point {x: width, y: height},
            bounce_direction: utils::Point {
                x: -1.0,
                y: 1.0
            },
        };
    }
    fn pit(height: utils::Pixels, width: utils::Pixels) -> Wall {
        return Wall {
            origin: utils::Point {x: 0.0, y: height - WALL_THICKNESS},
            limits: utils::Point {x: height, y: width},
            bounce_direction: utils::Point {
                x: 0.0,
                y: 0.0
            },
        }
    }
    pub fn make_walls(height: utils::Pixels, width: utils::Pixels)
        -> (Wall, Wall, Wall, Wall)
    {
        (
            top_wall(width),
            left_wall(height),
            right_wall(height, width),
            pit(height, width),
        )
    }
}