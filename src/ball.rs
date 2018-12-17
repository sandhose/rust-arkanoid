use failure::{err_msg, Error};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, RenderTarget, Texture};

use resize::RenderContext;
use shape::Circle;
use traits::{Collision, Renderable, Updatable};
use utils::{Pixels, Point, Rad, Vector, PI};
use textures::{TextureMaker, BallSprite};

pub const BALL_RADIUS: Pixels = 8.0;
const BALL_SPEED: f64 = 400.;

#[derive(Clone)]
pub struct Ball {
    position: Point,
    pub velocity: Vector,
    color: sdl2::pixels::Color,
    hold_timer: f64,
}

impl Into<Circle> for &Ball {
    fn into(self) -> Circle {
        Circle::new(self.position, BALL_RADIUS)
    }
}

impl Ball {
    pub fn new(position: Point, angle: Rad) -> Ball {
        Ball {
            position,
            velocity: Vector {
                angle,
                norm: BALL_SPEED,
            },
            hold_timer: 3.,
            color: Color::RGBA(120, 120, 200, 230),
        }
    }

    pub fn shape(&self) -> Circle {
        self.into()
    }

    pub fn speed(&mut self, n: usize) {
        self.velocity.norm = BALL_SPEED / (n + 1) as f64;
    }

    pub fn rotate(&mut self, angle: Rad) {
        self.velocity.angle = (self.velocity.angle + angle) % 2. * PI;
    }

    pub fn bounce(&mut self, (normal, depth): Collision) {
        self.velocity = self.velocity | normal;
        self.position = self.position
            + Point::from(Vector {
                angle: normal,
                norm: depth,
            });
    }

    pub fn set_position(&mut self, p: Point) {
        self.position = p;
    }

    pub fn on_hold(&self) -> bool {
        self.hold_timer > 0.
    }
}

impl Updatable for Ball {
    fn update(&mut self, dt: f64) {
        if self.on_hold() {
            self.hold_timer -= dt;
        } else {
            self.position = self.position + Point::from(self.velocity * dt);
        }
    }
}

impl<T> Renderable<T> for Ball
where
    T: RenderTarget,
{
    fn render(&self, canvas: &mut Canvas<T>, context: &RenderContext, texture: &Texture) -> Result<(), Error> {
        let copy_rects = TextureMaker::ball(
            BallSprite::Ball4,
            Rect::from_center(
                context.translate_point(self.position),
                context.scale(BALL_RADIUS * 2.),
                context.scale(BALL_RADIUS * 2.),
            )
        );
        canvas.copy(texture, copy_rects.src, copy_rects.dst).map_err(err_msg)?;
        Ok(())
    }
}
