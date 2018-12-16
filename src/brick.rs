use failure::err_msg;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use traits::{Collisionable, Renderable, Updatable};

use ball;
use utils::{collision, CollisionResult, Pixels, Point};
use shape;
use wall;

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
    pub position: Point, // TODO: use real coordinates
    pub breakable: bool,
    pub hitpoints: u8,
}

impl Brick {
    pub fn new(brick_type: BrickType, x: Pixels, y: Pixels) -> Brick {
        let position = Point { x, y };

        match brick_type {
            BrickType::Simple => Brick {
                brick_type,
                position,
                breakable: true,
                hitpoints: 1,
            },
            BrickType::Hard => Brick {
                brick_type,
                position,
                breakable: true,
                hitpoints: 2,
            },
            BrickType::Super => Brick {
                brick_type,
                position,
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
        let (xg, xd) = self.get_x();
        let (yh, yb) = self.get_y();
        canvas
            .fill_rect(Rect::new(
                xg as i32,
                yh as i32,
                (xd - xg) as u32,
                (yb - yh) as u32,
            ))
            .map_err(err_msg)?;
        Ok(())
    }
}

impl Collisionable for Brick {
    fn get_x(&self) -> (Pixels, Pixels) {
        (
            (self.position.x * BRICK_WIDTH
                + (self.position.x + 1.0) * BRICK_H_PAD
                + wall::WALL_THICKNESS),
            ((self.position.x + 1.0) * BRICK_WIDTH + (self.position.x + 1.0) * BRICK_H_PAD)
                + wall::WALL_THICKNESS,
        )
    }
    fn get_y(&self) -> (Pixels, Pixels) {
        (
            (self.position.y * BRICK_HEIGHT + (self.position.y + 1.0) * BRICK_V_PAD)
                + wall::WALL_THICKNESS,
            ((self.position.y + 1.0) * BRICK_HEIGHT + (self.position.y + 1.0) * BRICK_V_PAD)
                + wall::WALL_THICKNESS,
        )
    }

    fn collides(&self, ball: &ball::Ball) -> Option<CollisionResult> {
        collision::<Brick>(self, ball)
    }
}
