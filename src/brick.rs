use failure::err_msg;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::{Canvas, RenderTarget, Texture};
use traits::Renderable;
use textures::{TextureMaker, BrickSprite};

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
    pub center: Point,
    width: Pixels,
    height: Pixels,
    pub breakable: bool,
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
        texture: &Texture,
    ) -> Result<(), failure::Error> {
        let copy_rects = TextureMaker::brick(
            BrickSprite::Red,
            SDLRect::from_center(
                context.translate_point(self.center),
                context.scale(self.width),
                context.scale(self.height),
            )
        );
        canvas.copy(texture, copy_rects.src, copy_rects.dst).map_err(err_msg)?;
        Ok(())
    }
}
