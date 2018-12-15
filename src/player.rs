use failure::err_msg;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use traits::{Collisionable, Renderable};

use ball;
use utils;

pub const PLAYER_WIDTH: utils::Pixels = 80.0;
pub const PLAYER_THICKNESS: utils::Pixels = 16.0;
pub const PLAYER_END_RADIUS: utils::Pixels = PLAYER_THICKNESS * 0.5;

pub struct Player {
    pub position: utils::Point,
    pub color: sdl2::pixels::Color,
}

impl Collisionable for Player {
    fn get_x(&self) -> (utils::Pixels, utils::Pixels) {
        return (
            self.position.x - (PLAYER_WIDTH * 0.5),
            self.position.x + (PLAYER_WIDTH * 0.5),
        );
    }
    fn get_y(&self) -> (utils::Pixels, utils::Pixels) {
        return (
            self.position.y - (PLAYER_THICKNESS * 0.5),
            self.position.y + (PLAYER_THICKNESS * 0.5),
        );
    }

    fn collides(&self, ball: &ball::Ball) -> utils::CollisionResult {
        return utils::collision::<Player>(&self, &ball);
    }
}

impl<T> Renderable<T> for Player
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
        canvas
            .filled_circle(
                xg as i16,
                (yh + (PLAYER_THICKNESS * 0.5)) as i16,
                PLAYER_END_RADIUS as i16,
                self.color,
            )
            .map_err(err_msg)?;
        canvas
            .filled_circle(
                xd as i16,
                (yb - (PLAYER_THICKNESS * 0.5)) as i16,
                PLAYER_END_RADIUS as i16,
                self.color,
            )
            .map_err(err_msg)?;
        Ok(())
    }
}

// TODO :
// impl Updatable for Player
