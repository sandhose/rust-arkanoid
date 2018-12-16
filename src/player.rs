use failure::err_msg;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use traits::Renderable;

use utils;

pub const PLAYER_WIDTH: utils::Pixels = 80.0;
pub const PLAYER_THICKNESS: utils::Pixels = 16.0;
pub const PLAYER_END_RADIUS: utils::Pixels = PLAYER_THICKNESS * 0.5;

pub struct Player {
    pub position: utils::Point,
    pub color: sdl2::pixels::Color,
}

impl<T> Renderable<T> for Player
where
    T: RenderTarget,
{
    fn render(&self, canvas: &mut Canvas<T>) -> Result<(), failure::Error> {
        canvas.set_draw_color(self.color);
        canvas
            .fill_rect(Rect::from_center(
                self.position,
                PLAYER_WIDTH as u32,
                PLAYER_THICKNESS as u32,
            ))
            .map_err(err_msg)?;
        Ok(())
    }
}

// TODO :
// impl Updatable for Player
