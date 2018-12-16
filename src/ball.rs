use shape::Circle;
use traits::{Renderable, Updatable};
use utils;

use failure::{err_msg, Error};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::{Canvas, RenderTarget};

pub const BALL_RADIUS: utils::Pixels = 20.0;

pub struct Ball {
    pub position: utils::Point,
    pub velocity: utils::Vector,
    pub acceleration: utils::Pixels,
    pub color: sdl2::pixels::Color,
}

impl Ball {
    pub fn bounce(&mut self, new_angle: utils::Rad) {
        self.velocity.angle = new_angle;
    }

    pub fn shape(&self) -> Circle {
        Circle::from(self)
    }
}

impl Updatable for Ball {
    fn update(&mut self) {
        self.position = self.position + utils::Point::from(self.velocity);
        self.velocity.norm += self.acceleration;
    }
}

impl<T> Renderable<T> for Ball
where
    T: RenderTarget,
{
    fn render(&self, canvas: &mut Canvas<T>) -> Result<(), Error> {
        canvas.set_draw_color(self.color);
        canvas
            .filled_circle(
                self.position.x as i16,
                self.position.y as i16,
                BALL_RADIUS as i16,
                self.color,
            )
            .map_err(err_msg)?;
        Ok(())
    }
}
