use sdl2::render::{Canvas, RenderTarget};
use sdl2::rect::Rect;
use failure::{err_msg};

use ball;
use traits::{Collisionable, Renderable};
use utils;

pub const WALL_THICKNESS: utils::Pixels = 10.0;

pub struct Wall {
    pub origin: utils::Point,
    pub limits: utils::Point,
    pub bounce_direction: utils::Point,
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
        if (ball.position.x + ball::BALL_RADIUS) > self.origin.x &&
            ball.position.x < (self.limits.x + ball::BALL_RADIUS) &&
           (ball.position.y + ball::BALL_RADIUS) > self.origin.y &&
            ball.position.y < (self.limits.y + ball::BALL_RADIUS)
        {
            return utils::CollisionResult {
                collided: true,
                collision_vector: utils::Point {
                    x: 1.0 * self.bounce_direction.x * ball.speed.x,
                    y: 1.0 * self.bounce_direction.y * ball.speed.y,
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

impl<T> Renderable<T> for Wall
where
    T: RenderTarget,
{
    fn render(&self, canvas: &mut Canvas<T>)  -> Result<(), failure::Error> {
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(127, 127, 127, 255));
        canvas
            .fill_rect(Rect::new(
                self.origin.x as i32,
                self.origin.y as i32,
                (self.limits.x - self.origin.x) as u32,
                (self.limits.y - self.origin.y) as u32,
            )).map_err(err_msg)?;
        Ok(())
    }
}

pub mod WallFactory {
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
    pub fn make_walls(h: utils::Pixels, w: utils::Pixels) -> Vec<Wall> {
        return vec![top_wall(w), left_wall(h), right_wall(h, w), pit(h, w)];
    }
}
