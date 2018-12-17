use failure::err_msg;
use resize::RenderContext;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::{Canvas, RenderTarget};
use traits::{Renderable, Updatable};

use shape::Rect;
use utils;

pub const PLAYER_WIDTH: utils::Pixels = 80.0;
pub const PLAYER_THICKNESS: utils::Pixels = 16.0;
pub const PLAYER_END_RADIUS: utils::Pixels = PLAYER_THICKNESS * 0.5;
const PLAYER_FRICTION: f64 = 10.;
const PLAYER_ACCELERATION: f64 = 5000.;

pub struct Player {
    pub position: utils::Point,
    pub velocity: utils::Pixels,
    pub acceleration: utils::Pixels,
}

impl Into<Rect> for &Player {
    fn into(self) -> Rect {
        Rect::new(self.position, PLAYER_WIDTH, PLAYER_THICKNESS)
    }
}

impl Player {
    pub fn shape(&self) -> Rect {
        self.into()
    }
}

impl<T> Renderable<T> for Player
where
    T: RenderTarget,
{
    fn render(
        &self,
        canvas: &mut Canvas<T>,
        context: &RenderContext,
    ) -> Result<(), failure::Error> {
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 255, 255, 255));
        canvas
            .fill_rect(SDLRect::from_center(
                context.translate_point(self.position),
                context.scale(PLAYER_WIDTH),
                context.scale(PLAYER_THICKNESS),
            ))
            .map_err(err_msg)?;
        Ok(())
    }
}

impl Updatable for Player {
    fn update(&mut self, dt: f64) {
        let acceleration =
            (self.acceleration * PLAYER_ACCELERATION) - (self.velocity * PLAYER_FRICTION);
        self.velocity += acceleration * dt;

        if self.velocity.abs() < 1. {
            self.velocity = 0.;
        }

        self.position.x += self.velocity * dt;
    }
}
