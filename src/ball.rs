use brick::Brick;
use traits::{Updatable, Renderable, UpdateFrame};

use failure::{err_msg, Error};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};


type Pixels = u32;
pub const BALL_RADIUS: Pixels = 60;

pub struct Ball {
    pub position: (u32, u32),
    pub angle: f64,
    pub color: Color,
}

impl Ball {
    pub fn collides(&self, brick: &Brick) -> bool {
        let (xg, xd) = brick.get_x();
        let (yh, yb) = brick.get_y();
        if self.position.0 > xg && self.position.0 < xd &&
           self.position.1 > yh && self.position.1 < yb {
            return true;
        }
        return false;
    }
}

impl Updatable for Ball {
    //fn update(&mut self, frame: &UpdateFrame) {
    fn update(&mut self) {
        self.position.0 += 1;
        self.position.1 += 1;
    }
}

impl<T> Renderable<T> for Ball
where
    T: RenderTarget,
{
    fn render(&self, canvas: &mut Canvas<T>)
        -> Result<(), Error>
    {
        canvas.set_draw_color(self.color);
        canvas
            .fill_rect(Rect::new(
                (self.position.0 - (BALL_RADIUS / 2)) as i32,
                (self.position.1 - (BALL_RADIUS / 2)) as i32,
                BALL_RADIUS,
                BALL_RADIUS,
            )).map_err(err_msg)?;
        Ok(())
    }
}
