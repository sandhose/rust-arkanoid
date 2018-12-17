use sdl2::pixels::Color;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::{Canvas, RenderTarget};
use failure::{err_msg, Error};

use shape::Circle;
use resize::RenderContext;
use utils::Point;
use traits::{Renderable, Updatable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BonusType {
    Slow
}

#[derive(Debug)]
pub struct Bonus {
    pub bonus_type: BonusType,
    pub position: Point
}

impl Bonus {
    pub fn random(position: Point) -> Self {
        Bonus { bonus_type: BonusType::Slow, position }
    }

    pub fn shape(&self) -> Circle {
        Circle::from(self)
    }
}


impl<T> Renderable<T> for Bonus
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

impl Updatable for Bonus {
    fn update(&mut self, dt: f64) {
        self.position.y += dt * 200.;
    }
}

#[derive(Debug)]
pub struct ActiveBonus {
    pub bonus_type: BonusType,
    timer: f64
}

impl ActiveBonus {
    pub fn active(&self) -> bool {
        self.timer > 0.
    }
}

impl From<&Bonus> for ActiveBonus {
    fn from(bonus: &Bonus) -> ActiveBonus {
        ActiveBonus { bonus_type: bonus.bonus_type, timer: 10. }
    }
}

impl Updatable for ActiveBonus {
    fn update(&mut self, dt: f64) {
        self.timer -= dt;
    }
}
