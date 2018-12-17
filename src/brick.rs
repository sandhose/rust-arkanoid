use failure::err_msg;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::{Canvas, RenderTarget};
use traits::Renderable;

use resize::RenderContext;
use shape::Rect;
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
    width: Pixels,
    height: Pixels,
    breakable: bool,
    hitpoints: u8,
}

impl Into<Rect> for &Brick {
    fn into(self) -> Rect {
        Rect::new(self.center, self.width, self.height)
    }
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

    pub fn alive(&self) -> bool {
        self.hitpoints > 0 || !self.breakable
    }

    fn color(&self) -> sdl2::pixels::Color {
        match &self.hitpoints {
            1 => sdl2::pixels::Color::RGBA(200, 0, 200, 200),
            2 => sdl2::pixels::Color::RGBA(0, 200, 200, 200),
            _ => sdl2::pixels::Color::RGBA(200, 200, 0, 200),
        }
    }

    pub fn shape(&self) -> Rect {
        self.into()
    }

    pub fn damage(&mut self) {
        if self.breakable && self.hitpoints > 0 {
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
            .fill_rect(SDLRect::from_center(
                context.translate_point(self.center),
                context.scale(self.width),
                context.scale(self.height),
            ))
            .map_err(err_msg)?;
        Ok(())
    }
}
