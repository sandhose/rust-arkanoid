use traits::{Updatable, Renderable, Collisionable};
use sdl2::render::{Canvas, RenderTarget};
use sdl2::rect::Rect;
use failure::{err_msg};

use ball;
use utils;
use wall;

pub const BRICK_WIDTH: utils::Pixels = 80.0;
pub const BRICK_HEIGHT: utils::Pixels = 40.0;
pub const BRICK_V_PAD: utils::Pixels = 5.0;
pub const BRICK_H_PAD: utils::Pixels = 5.0;

pub enum BrickType {
    Undefined,
    Simple,
    Hard,
    Super,
}

pub struct Brick {
    pub position: utils::Point,
    pub breakable: bool,
    pub hitpoints: u8,
    pub color: sdl2::pixels::Color,
}

impl Brick {
    pub fn get_x(&self) -> (utils::Pixels, utils::Pixels) {
        return (
            (self.position.x * BRICK_WIDTH +
                (self.position.x + 1.0) * BRICK_H_PAD +
                wall::WALL_THICKNESS),
            ((self.position.x + 1.0) * BRICK_WIDTH +
                (self.position.x + 1.0) * BRICK_H_PAD) +
                wall::WALL_THICKNESS,
        );
    }
    pub fn get_y(&self) -> (utils::Pixels, utils::Pixels) {
        return (
            (self.position.y * BRICK_HEIGHT +
                (self.position.y + 1.0) * BRICK_V_PAD) +
                wall::WALL_THICKNESS,
            ((self.position.y + 1.0) * BRICK_HEIGHT +
                (self.position.y + 1.0) * BRICK_V_PAD) +
                wall::WALL_THICKNESS,
        );
    }
}

impl Updatable for Brick {
    fn update(&mut self) {
        return;
    }
}

impl<T> Renderable<T> for Brick
where
    T: RenderTarget,
{
    fn render(&self, canvas: &mut Canvas<T>)  -> Result<(), failure::Error> {
        canvas.set_draw_color(self.color);
        let (xg, xd) = self.get_x();
        let (yh, yb) = self.get_y();
        canvas
            .fill_rect(Rect::new(
                xg as i32,
                yh as i32,
                (xd - xg) as u32,
                (yb - yh) as u32,
            )).map_err(err_msg)?;
        Ok(())
    }
}

impl Collisionable for Brick {
    fn collides(&self, ball: &ball::Ball) -> utils::CollisionResult {
        let (xg, xd) = self.get_x();
        let (yh, yb) = self.get_y();
        if (ball.position.x + ball::BALL_RADIUS) > xg &&
           ball.position.x < (xd + ball::BALL_RADIUS) &&
           ball.position.y > yh && ball.position.y < yb
        {
            return utils::CollisionResult {
                collided: true,
                collision_vector: utils::Point {x: -1.0, y: 1.0}
            };
        }
        if (ball.position.y + ball::BALL_RADIUS) > yh &&
           ball.position.y < (yb + ball::BALL_RADIUS) &&
           ball.position.x > xg && ball.position.x < xd
        {
            return utils::CollisionResult {
                collided: true,
                collision_vector: utils::Point {x: 1.0, y: -1.0}
            };
        }
        let corners = [
            utils::Point {x: xg, y: yh},
            utils::Point {x: xg, y: yb},
            utils::Point {x: xd, y: yh},
            utils::Point {x: xd, y: yb},
        ];
        for corner in corners.iter() {
            if utils::distance(corner, &ball.position) < ball::BALL_RADIUS {
                let bounce_vector = utils::angle_clsn_bnce_vect(
                    corner, &ball.position); 
                return utils::CollisionResult {
                    collided: true,
                    collision_vector: bounce_vector,
                };
            }
        }
        return utils::CollisionResult {
            collided: false,
            collision_vector: utils::Point {x: 1.0, y: 1.0}
        };
    }
}

pub mod BrickFactory {
    use super::*;

    pub fn simple_brick(x: utils::Pixels, y: utils::Pixels) -> Brick {
        Brick {
            position: utils::Point {x: x, y: y},
            breakable: true,
            hitpoints: 1,
            color: sdl2::pixels::Color::RGBA(200, 0, 200, 200),
        }
    } 
    pub fn hard_brick(x: utils::Pixels, y: utils::Pixels) -> Brick {
        Brick {
            position: utils::Point {x: x, y: y},
            breakable: true,
            hitpoints: 2,
            color: sdl2::pixels::Color::RGBA(0, 200, 200, 200),
        }
    }
    pub fn super_brick(x: utils::Pixels, y: utils::Pixels) -> Brick {
        Brick {
            position: utils::Point {x: x, y: y},
            breakable: false,
            hitpoints: 0,
            color: sdl2::pixels::Color::RGBA(200, 200, 0, 200),
        }
    }
}
