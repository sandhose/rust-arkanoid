use failure::{err_msg, Error};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

use sdl2::pixels::Color;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::{Canvas, RenderTarget, Texture};

use resize::RenderContext;
use shape::Circle;
use traits::{Renderable, Updatable};
use utils::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BonusType {
    Slow,
    Expand,
    Divide,
    Life,
}

impl Distribution<BonusType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BonusType {
        match rng.gen_range(0, 4) {
            0 => BonusType::Slow,
            1 => BonusType::Expand,
            2 => BonusType::Divide,
            _ => BonusType::Life,
        }
    }
}

impl BonusType {
    pub fn color(self) -> Color {
        match self {
            BonusType::Slow => Color::RGBA(255, 0, 0, 255),
            BonusType::Expand => Color::RGBA(0, 255, 0, 255),
            BonusType::Divide => Color::RGBA(0, 0, 255, 255),
            BonusType::Life => Color::RGBA(0, 255, 255, 255),
        }
    }
}

#[derive(Debug)]
pub struct FallingBonus {
    pub bonus_type: BonusType,
    position: Point,
}

impl Into<Circle> for &FallingBonus {
    fn into(self) -> Circle {
        Circle::new(self.position, 12.)
    }
}

impl FallingBonus {
    pub fn random(position: Point) -> Self {
        FallingBonus {
            bonus_type: rand::random(),
            position,
        }
    }

    pub fn shape(&self) -> Circle {
        self.into()
    }
}

impl<T> Renderable<T> for FallingBonus
where
    T: RenderTarget,
{
    fn render(
        &self,
        canvas: &mut Canvas<T>,
        context: &RenderContext,
        _texture: &Texture,
    ) -> Result<(), Error> {
        canvas.set_draw_color(self.bonus_type.color());
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
