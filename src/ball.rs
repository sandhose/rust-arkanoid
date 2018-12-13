use brick::Brick;
use traits::{Updatable, Renderable};

use failure::{err_msg, Error};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

#[derive(PartialEq, Eq)]
pub enum Side {
    No,
    Left,
    Right,
    Up,
    Down,
}

type Pixels = u32;
pub const BALL_RADIUS: Pixels = 60;

pub struct Ball {
    pub position: (u32, u32),
    pub speed: (u32, u32),
    pub color: Color,
}

impl Ball {
    pub fn collides(&self, brick: &Brick) -> Side {
        let (xg, xd) = brick.get_x();
        let (yh, yb) = brick.get_y();
        return Side::No;
    }
    pub fn bounce(&mut self, new_speed: (u32, u32)) {
        self.speed = new_speed;
    }
}

impl Updatable for Ball {
    //fn update(&mut self, frame: &UpdateFrame) {
    fn update(&mut self) {
        self.position.0 += self.speed.0;
        self.position.1 += self.speed.1;
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
