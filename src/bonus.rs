use failure::{err_msg, Error};
use sdl2::pixels::Color;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::{Canvas, RenderTarget};

use resize::RenderContext;
use shape::Circle;
use state::{State, BALL_SPEED};
use traits::{Renderable, Updatable};
use utils::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BonusType {
    Slow,
}

impl BonusType {
    pub fn activate(self, state: &mut State) {
        match self {
            BonusType::Slow => state.queue_bonus(ActiveBonus::from(self)),
        }
    }

    pub fn stack(self, state: &mut State, count: usize) {
        match self {
            BonusType::Slow => {
                state.ball.velocity.norm = BALL_SPEED / (count + 1) as f64;
            }
        }
    }
}

#[derive(Debug)]
pub struct FallingBonus {
    pub bonus_type: BonusType,
    pub position: Point,
}

impl FallingBonus {
    pub fn random(position: Point) -> Self {
        FallingBonus {
            bonus_type: BonusType::Slow,
            position,
        }
    }

    pub fn shape(&self) -> Circle {
        Circle::from(self)
    }
}

impl<T> Renderable<T> for FallingBonus
where
    T: RenderTarget,
{
    fn render(&self, canvas: &mut Canvas<T>, context: &RenderContext) -> Result<(), Error> {
        canvas.set_draw_color(Color::RGBA(200, 200, 200, 200));
        canvas
            .fill_rect(SDLRect::from_center(
                context.translate_point(self.position),
                context.scale(10.),
                context.scale(10.),
            ))
            .map_err(err_msg)?;
        Ok(())
    }
}

impl Updatable for FallingBonus {
    fn update(&mut self, dt: f64) {
        self.position.y += dt * 200.;
    }
}

#[derive(Debug)]
pub struct ActiveBonus {
    pub bonus_type: BonusType,
    timer: f64,
}

impl ActiveBonus {
    pub fn active(&self) -> bool {
        self.timer > 0.
    }
}
impl From<&FallingBonus> for ActiveBonus {
    fn from(bonus: &FallingBonus) -> ActiveBonus {
        ActiveBonus::from(bonus.bonus_type)
    }
}

impl From<BonusType> for ActiveBonus {
    fn from(bonus_type: BonusType) -> ActiveBonus {
        ActiveBonus {
            bonus_type,
            timer: 10.,
        }
    }
}

impl Updatable for ActiveBonus {
    fn update(&mut self, dt: f64) {
        self.timer -= dt;
    }
}
