use failure::err_msg;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use traits::Renderable;

use resize::RenderContext;
use shape;
use utils::{Pixels, Point};

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
    pub center: Point,
    pub width: Pixels,
    pub height: Pixels,
    pub breakable: bool,
    pub hitpoints: u8,
}

impl Brick {
    pub fn new(brick_type: BrickType, center: Point, width: Pixels, height: Pixels) -> Brick {
        match brick_type {
            BrickType::Simple => Brick {
                center,
                width,
                height,
                breakable: true,
                hitpoints: 1,
            },
            BrickType::Hard => Brick {
                center,
                width,
                height,
                breakable: true,
                hitpoints: 2,
            },
            BrickType::Super => Brick {
                center,
                width,
                height,
                breakable: false,
                hitpoints: 0,
            },
        }
    }

    fn color(&self) -> sdl2::pixels::Color {
        match &self.hitpoints {
            1 => sdl2::pixels::Color::RGBA(200, 0, 200, 200),
            2 => sdl2::pixels::Color::RGBA(0, 200, 200, 200),
            _ => sdl2::pixels::Color::RGBA(200, 200, 0, 200),
        }
    }

    pub fn shape(&self) -> shape::Rect {
        shape::Rect::from(self)
    }

    pub fn damage(&mut self) {
        if self.breakable {
            self.hitpoints -= 1;
        }
    }
}

impl<T> Renderable<T> for Brick
where
    T: RenderTarget,
{
    fn render(
        &self,
        canvas: &mut Canvas<T>,
        context: &RenderContext,
    ) -> Result<(), failure::Error> {
        canvas.set_draw_color(self.color());
        canvas
            .fill_rect(Rect::from_center(
                context.translate_point(self.center),
                context.scale(self.width),
                context.scale(self.height),
            ))
            .map_err(err_msg)?;
        Ok(())
    }
}
