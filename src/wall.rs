use failure::err_msg;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use resize::RenderContext;
use shape::{InfiniteWall, WallOrientation};
use traits::Renderable;
use utils::{Pixels, Point};

pub const WALL_THICKNESS: Pixels = 4.0;

// DONE: refactored this
pub struct Wall {
    width: Pixels,
    height: Pixels,
    pub shape: InfiniteWall,
}

impl Wall {
    fn top(width: Pixels) -> Self {
        Wall {
            width: width,
            height: WALL_THICKNESS,
            shape: InfiniteWall {
                orientation: WallOrientation::Top,
                center: Point::new(width / 2., WALL_THICKNESS / 2.),
            },
        }
    }

    fn left(height: Pixels) -> Self {
        Wall {
            width: WALL_THICKNESS,
            height: height,
            shape: InfiniteWall {
                orientation: WallOrientation::Left,
                center: Point::new(WALL_THICKNESS / 2., height / 2.),
            },
        }
    }

    fn right(height: Pixels, width: Pixels) -> Self {
        return Wall {
            width: WALL_THICKNESS,
            height: height,
            shape: InfiniteWall {
                orientation: WallOrientation::Right,
                center: Point::new(width - (WALL_THICKNESS / 2.), height / 2.),
            },
        };
    }

    fn pit(height: Pixels, width: Pixels) -> Self {
        Wall {
            width: width,
            height: WALL_THICKNESS,
            shape: InfiniteWall {
                orientation: WallOrientation::Bottom,
                center: Point::new(width / 2., height - (WALL_THICKNESS / 2.)),
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
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(127, 127, 127, 255));
        canvas
            .fill_rect(Rect::from_center(
                context.translate_point(self.shape.center),
                context.scale(self.width),
                context.scale(self.height),
            ))
            .map_err(err_msg)?;
        Ok(())
    }
}
