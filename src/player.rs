use failure::err_msg;
use resize::RenderContext;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::{Canvas, RenderTarget};

use shape::Rect;
use traits::{Collision, Renderable, Updatable};
use utils::{Pixels, Point, Vector};

const PLAYER_WIDTH: Pixels = 80.0;
const PLAYER_THICKNESS: Pixels = 16.0;
const PLAYER_FRICTION: f64 = 10.;
const PLAYER_ACCELERATION: f64 = 5000.;

pub struct Player {
    position: Point,
    velocity: Pixels,
    acceleration: Pixels,
}

impl Into<Rect> for &Player {
    fn into(self) -> Rect {
        Rect::new(self.position, PLAYER_WIDTH, PLAYER_THICKNESS)
    }
}

impl Player {
    pub fn new(position: Point) -> Player {
        Player {
            position,
            velocity: 0.,
            acceleration: 0.,
        }
    }

    pub fn input(&mut self, input: f64) {
        self.acceleration = input;
    }

    pub fn bounce(&mut self, (normal, depth): Collision) {
        self.velocity = -self.velocity;
        self.position.x = self.position.x
            + Vector {
                angle: normal,
                norm: depth,
            }
            .x();
    }

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
