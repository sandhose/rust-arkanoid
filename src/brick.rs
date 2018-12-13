use sdl2::pixels::Color;
use traits::{Updatable, Renderable};
use sdl2::render::{Canvas, RenderTarget};
use sdl2::rect::Rect;
use failure::{err_msg, Error};

type Pixels = u32;
pub const BRICK_WIDTH: Pixels = 80;
pub const BRICK_HEIGHT: Pixels = 40;
pub const BRICK_V_PAD: Pixels = 5;
pub const BRICK_H_PAD: Pixels = 5;

pub enum BrickType {
    Undefined,
    Simple,
    Hard,
    Super,
}

pub struct Brick {
    pub x: u32,
    pub y: u32,
    pub breakable: bool,
    pub hitpoints: u8,
    pub color: Color,
}

impl Brick {
    pub fn get_x(&self) -> (u32, u32) {
        let xg: u32 = (self.x * BRICK_WIDTH + (self.x + 1) * BRICK_H_PAD) as u32;
        let xd: u32 = ((self.x + 1) * BRICK_WIDTH + (self.x + 1) * BRICK_H_PAD) as u32;
        (xg, xd)
    }
    pub fn get_y(&self) -> (u32, u32) {
        let yh: u32 = (self.y * BRICK_HEIGHT + (self.y + 1) * BRICK_V_PAD) as u32;
        let yb: u32 = ((self.y + 1) * BRICK_HEIGHT + (self.y + 1) * BRICK_V_PAD) as u32;
        (yh, yb)
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
                xd - xg,
                yb - yh,
            )).map_err(err_msg)?;
        Ok(())
    }
}

pub mod BrickFactory {
    use super::*;

    pub fn simple_brick(x: Pixels, y: Pixels) -> Brick {
        Brick {
            x: x,
            y: y,
            breakable: true,
            hitpoints: 1,
            color: Color::RGBA(200, 0, 200, 200),
        }
    } 
    pub fn hard_brick(x: Pixels, y: Pixels) -> Brick {
        Brick {
            x: x,
            y: y,
            breakable: true,
            hitpoints: 2,
            color: Color::RGBA(0, 200, 200, 200),
        }
    }
    pub fn super_brick(x: Pixels, y: Pixels) -> Brick {
        Brick {
            x: x,
            y: y,
            breakable: false,
            hitpoints: 0,
            color: Color::RGBA(200, 200, 0, 200),
        }
    }
}
