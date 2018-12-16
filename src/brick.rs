use failure::err_msg;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use traits::{Collisionable, Renderable, Updatable};

use ball;
use utils::{collision, CollisionResult, Pixels, Point};
use wall;

pub const BRICK_WIDTH: Pixels = 80.0;
pub const BRICK_HEIGHT: Pixels = 40.0;
pub const BRICK_V_PAD: Pixels = 5.0;
pub const BRICK_H_PAD: Pixels = 5.0;

pub enum BrickType {
    Simple,
    Hard,
    Super,
}

#[derive(Clone)]
pub struct Brick {
    pub position: Point,
    pub breakable: bool,
    pub hitpoints: u8,
    pub color: sdl2::pixels::Color,
}

impl Brick {
    pub fn new(brick_type: BrickType, x: Pixels, y: Pixels) -> Brick {
        let position = Point { x, y };

        match brick_type {
            BrickType::Simple => Brick {
                position,
                breakable: true,
                hitpoints: 1,
                color: sdl2::pixels::Color::RGBA(200, 0, 200, 200),
            },
            BrickType::Hard => Brick {
                position,
                breakable: true,
                hitpoints: 2,
                color: sdl2::pixels::Color::RGBA(0, 200, 200, 200),
            },
            BrickType::Super => Brick {
                position,
                breakable: false,
                hitpoints: 0,
                color: sdl2::pixels::Color::RGBA(200, 200, 0, 200),
            },
        }
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
        canvas.set_draw_color(self.color);
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

    fn collides(&self, ball: &ball::Ball) -> CollisionResult {
        collision::<Brick>(self, ball)
    }
}
