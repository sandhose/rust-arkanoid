use utils;
use traits::{Updatable, Renderable};

use failure::{err_msg, Error};
use sdl2::render::{Canvas, RenderTarget};
use sdl2::gfx::primitives::DrawRenderer;

pub const BALL_RADIUS: utils::Pixels = 20.0;

pub struct Ball {
    pub position: utils::Point,
    pub speed: utils::Point,
    pub color: sdl2::pixels::Color,
}

impl Ball {
    pub fn bounce(&mut self, change_vector: utils::Point) {
        self.speed.x *= change_vector.x;
        self.speed.y *= change_vector.y;
    }
}

impl Updatable for Ball {
    fn update(&mut self) {
        self.position.x += self.speed.x;
        self.position.y += self.speed.y;
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
            self.position.x as i16,
            self.position.y as i16,
            BALL_RADIUS as i16,
            self.color,
        ).map_err(err_msg)?;
        Ok(())
    }
}
