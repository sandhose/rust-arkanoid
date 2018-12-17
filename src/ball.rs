use failure::{err_msg, Error};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::{Canvas, RenderTarget};

use resize::RenderContext;
use shape::Circle;
use traits::{Collision, Renderable, Updatable};
use utils::{Pixels, Point, Vector};

const BALL_RADIUS: Pixels = 8.0;

pub struct Ball {
    pub position: Point,
    pub velocity: Vector,
    pub color: sdl2::pixels::Color,
}

impl Into<Circle> for &Ball {
    fn into(self) -> Circle {
        Circle::new(self.position, BALL_RADIUS)
    }
}

impl Ball {
    pub fn shape(&self) -> Circle {
        self.into()
    }

    pub fn bounce(&mut self, (normal, depth): Collision) {
        self.velocity = self.velocity | normal;
        self.position = self.position
            + Point::from(Vector {
                angle: normal,
                norm: depth,
            });
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
