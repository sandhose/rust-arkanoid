use failure::err_msg;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use traits::{Renderable, Updatable};

use shape;
use utils::{Pixels, Point};

pub const BRICK_WIDTH: Pixels = 80.0;
pub const BRICK_HEIGHT: Pixels = 40.0;
pub const BRICK_V_PAD: Pixels = 5.0;
pub const BRICK_H_PAD: Pixels = 5.0;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BrickType {
    Simple,
    Hard,
    Super,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Brick {
    #[serde(rename = "type")]
    brick_type: BrickType,
    pub center: Point,
    pub breakable: bool,
    pub hitpoints: u8,
}

impl Brick {
    pub fn new(brick_type: BrickType, center: Point) -> Brick {
        match brick_type {
            BrickType::Simple => Brick {
                brick_type,
                center,
                breakable: true,
                hitpoints: 1,
            },
            BrickType::Hard => Brick {
                brick_type,
                center,
                breakable: true,
                hitpoints: 2,
            },
            BrickType::Super => Brick {
                brick_type,
                center,
                breakable: false,
                hitpoints: 0,
            },
        }
    }

    fn color(&self) -> sdl2::pixels::Color {
        match &self.brick_type {
            BrickType::Simple => sdl2::pixels::Color::RGBA(200, 0, 200, 200),
            BrickType::Hard => sdl2::pixels::Color::RGBA(0, 200, 200, 200),
            BrickType::Super => sdl2::pixels::Color::RGBA(200, 200, 0, 200),
        }
    }

    pub fn shape(&self) -> shape::Rect {
        shape::Rect::from(self)
    }
}

impl Updatable for Brick {
    fn update(&mut self) {
        return;
    }
}

impl<T> Renderable<T> for Brick
where
    T: RenderTarget,
{
    fn render(&self, canvas: &mut Canvas<T>) -> Result<(), failure::Error> {
        canvas.set_draw_color(self.color());
        canvas
            .fill_rect(Rect::from_center(
                self.center,
                BRICK_WIDTH as u32,
                BRICK_HEIGHT as u32,
            ))
            .map_err(err_msg)?;
        Ok(())
    }
}
