use sdl2::pixels::Color;
use traits::{Updatable, Renderable, Collisionable};
use sdl2::render::{Canvas, RenderTarget};
use sdl2::rect::Rect;
use failure::{err_msg};
use ball;
use utils;
use utils::{Point,Pixels};

pub const BRICK_WIDTH: Pixels = 80.0;
pub const BRICK_HEIGHT: Pixels = 40.0;
pub const BRICK_V_PAD: Pixels = 5.0;
pub const BRICK_H_PAD: Pixels = 5.0;

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
    pub color: Color,
}

impl Brick {
    pub fn get_x(&self) -> (Pixels, Pixels) {
        return (
            (self.position.x * BRICK_WIDTH +
                (self.position.x + 1.0) * BRICK_H_PAD),
            ((self.position.x + 1.0) * BRICK_WIDTH +
                (self.position.x + 1.0) * BRICK_H_PAD),
        );
    }
    pub fn get_y(&self) -> (Pixels, Pixels) {
        return (
            (self.position.y * BRICK_HEIGHT +
                (self.position.y + 1.0) * BRICK_V_PAD),
            ((self.position.y + 1.0) * BRICK_HEIGHT +
                (self.position.y + 1.0) * BRICK_V_PAD),
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
    fn collides(&self, ball: &ball::Ball) -> (bool, (f32, f32)) {
        let (xg, xd) = self.get_x();
        let (yh, yb) = self.get_y();
        let corners = [
            Point {x: xg, y: yh},
            Point {x: xg, y: yb},
            Point {x: xd, y: yh},
            Point {x: xd, y: yb},
        ];
        for corner in corners.iter() {
            if utils::distance(corner, &ball.position) < ball::BALL_RADIUS {
                return (true, (-1.0, 1.0));
            }
        }
        if (ball.position.x + ball::BALL_RADIUS) > xg &&
           ball.position.x < (xd + ball::BALL_RADIUS) &&
           ball.position.y > yh && ball.position.y < yb
        {
           return (true, (-1.0, 1.0)); 
        }
        if (ball.position.y + ball::BALL_RADIUS) > yh &&
           ball.position.y < (yb + ball::BALL_RADIUS) &&
           ball.position.x > xg && ball.position.x < xd
        {
           return (true, (1.0, -1.0)); 
        }
        return (false, (1.0, 1.0));
    }
}

pub mod BrickFactory {
    use super::*;

    pub fn simple_brick(x: Pixels, y: Pixels) -> Brick {
        Brick {
            position: Point {x: x, y: y},
            breakable: true,
            hitpoints: 1,
            color: Color::RGBA(200, 0, 200, 200),
        }
    } 
    pub fn hard_brick(x: Pixels, y: Pixels) -> Brick {
        Brick {
            position: Point {x: x, y: y},
            breakable: true,
            hitpoints: 2,
            color: Color::RGBA(0, 200, 200, 200),
        }
    }
    pub fn super_brick(x: Pixels, y: Pixels) -> Brick {
        Brick {
            position: Point {x: x, y: y},
            breakable: false,
            hitpoints: 0,
            color: Color::RGBA(200, 200, 0, 200),
        }
    }
}
