use utils::distance;
use brick::Brick;
use traits::{Updatable, Renderable};

use failure::{err_msg, Error};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::gfx::primitives::DrawRenderer;

type Pixels = u32;
pub const BALL_RADIUS: Pixels = 20;

pub struct Ball {
    pub position: (u32, u32),
    pub speed: (u32, u32),
    pub color: Color,
}

impl Ball {
    pub fn collides(&self, brick: &Brick) -> (bool, f32) {
        let (xg, xd) = brick.get_x();
        let (yh, yb) = brick.get_y();
        let corners = [(xg, yh), (xg, yb), (xd, yh), (xd, yb)];
        for corner in corners.iter() {
            if distance(*corner, self.position) < BALL_RADIUS {
                return (true, 90.0);
            }
        }
        return (false, 0.0);
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
        canvas.filled_circle(
            self.position.0 as i16,
            self.position.1 as i16,
            BALL_RADIUS as i16,
            self.color,
        );
        Ok(())
    }
}
