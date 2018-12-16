use failure::err_msg;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use resize::RenderContext;
use shape::{InfiniteWall, WallOrientation};
use traits::Renderable;
use utils::{Pixels, Point};

pub const WALL_THICKNESS: Pixels = 10.0;

// TODO: refactor this
pub struct Wall {
    pub origin: Point,
    pub limits: Point,
    pub bounce: Pixels,
    pub shape: InfiniteWall,
}

impl Wall {
    fn top(width: Pixels) -> Self {
        Wall {
            origin: Point { x: 0.0, y: 0.0 },
            limits: Point {
                x: width,
                y: WALL_THICKNESS,
            },
            bounce: 1.0,
            shape: InfiniteWall {
                orientation: WallOrientation::Top,
                position: WALL_THICKNESS,
            },
        }
    }

    fn left(height: Pixels) -> Self {
        Wall {
            origin: Point { x: 0.0, y: 0.0 },
            limits: Point {
                x: WALL_THICKNESS,
                y: height,
            },
            bounce: 1.0,
            shape: InfiniteWall {
                orientation: WallOrientation::Left,
                position: WALL_THICKNESS,
            },
        }
    }

    fn right(height: Pixels, width: Pixels) -> Self {
        return Wall {
            origin: Point {
                x: width - WALL_THICKNESS,
                y: 0.0,
            },
            limits: Point {
                x: width,
                y: height,
            },
            bounce: 1.0,
            shape: InfiniteWall {
                orientation: WallOrientation::Right,
                position: width - WALL_THICKNESS,
            },
        };
    }

    fn pit(height: Pixels, width: Pixels) -> Self {
        Wall {
            origin: Point {
                x: 0.0,
                y: height - WALL_THICKNESS,
            },
            limits: Point {
                x: width,
                y: height,
            },
            bounce: 0.0,
            shape: InfiniteWall {
                orientation: WallOrientation::Bottom,
                position: height - WALL_THICKNESS,
            },
        }
    }

    pub fn make_walls(h: Pixels, w: Pixels) -> Vec<Self> {
        return vec![
            Wall::top(w),
            Wall::left(h),
            Wall::right(h, w),
            Wall::pit(h, w),
        ];
    }
}

impl<T> Renderable<T> for Wall
where
    T: RenderTarget,
{
    fn render(
        &self,
        canvas: &mut Canvas<T>,
        context: &RenderContext,
    ) -> Result<(), failure::Error> {
        let t_origin = context.translate_point(self.origin);
        let t_limits = context.translate_point(self.limits);
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(127, 127, 127, 255));
        canvas
            .fill_rect(Rect::new(
                t_origin.x as i32,
                t_origin.y as i32,
                (t_limits.x - t_origin.x) as u32,
                (t_limits.y - t_origin.y) as u32,
            ))
            .map_err(err_msg)?;
        Ok(())
    }
}
