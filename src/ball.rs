use resize::RenderContext;
use shape::Circle;
use traits::{Renderable, Updatable};
use utils::{Pixels, Point, Vector};

use failure::{err_msg, Error};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::{Canvas, RenderTarget};

pub const BALL_RADIUS: Pixels = 8.0;

pub struct Ball {
    pub position: Point,
    pub velocity: Vector,
    pub color: sdl2::pixels::Color,
}

impl Ball {
    pub fn shape(&self) -> Circle {
        Circle::from(self)
    }
}

impl Updatable for Ball {
    fn update(&mut self, dt: f64) {
        self.position = self.position + Point::from(self.velocity * dt);
    }
}

impl<T> Renderable<T> for Ball
where
    T: RenderTarget,
{
    fn render(&self, canvas: &mut Canvas<T>, context: &RenderContext) -> Result<(), Error> {
        let center = context.translate_point(self.position);
        canvas.set_draw_color(self.color);
        canvas
            .filled_circle(
                center.x as i16,
                center.y as i16,
                context.scale(BALL_RADIUS) as i16,
                self.color,
            )
            .map_err(err_msg)?;
        Ok(())
    }
}
