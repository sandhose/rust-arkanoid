use failure::err_msg;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use ball;
use traits::{Collisionable, Renderable};
use utils::{collision, CollisionResult, Pixels, Point};

pub const WALL_THICKNESS: Pixels = 10.0;

pub struct Wall {
    pub origin: Point,
    pub limits: Point,
    pub bounce_direction: Point,
}

impl Wall {
    // Admettant un rectangle :
    // p1 - p2
    // |    |
    // p3 - p4
    pub fn get_boundaries(&self) -> (Point, Point, Point, Point) {
        let p1 = Point {
            x: self.origin.x,
            y: self.origin.y,
        };
        let p2 = Point {
            x: self.origin.x,
            y: self.limits.y,
        };
        let p3 = Point {
            x: self.limits.x,
            y: self.origin.y,
        };
        let p4 = Point {
            x: self.limits.x,
            y: self.limits.y,
        };

        (p1, p2, p3, p4)
    }

    fn top(width: Pixels) -> Self {
        Wall {
            origin: Point { x: 0.0, y: 0.0 },
            limits: Point {
                x: width,
                y: WALL_THICKNESS,
            },
            bounce_direction: Point { x: 1.0, y: 1.0 },
        }
    }

    fn left(height: Pixels) -> Self {
        Wall {
            origin: Point { x: 0.0, y: 0.0 },
            limits: Point {
                x: WALL_THICKNESS,
                y: height,
            },
            bounce_direction: Point { x: 1.0, y: 1.0 },
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
            bounce_direction: Point { x: 1.0, y: 1.0 },
        };
    }

    fn pit(height: Pixels, width: Pixels) -> Self {
        Wall {
            origin: Point {
                x: 0.0,
                y: height - WALL_THICKNESS,
            },
            limits: Point {
                x: height,
                y: width,
            },
            bounce_direction: Point { x: 0.0, y: 0.0 },
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

impl Collisionable for Wall {
    fn get_x(&self) -> (Pixels, Pixels) {
        return (self.origin.x, self.limits.x);
    }
    fn get_y(&self) -> (Pixels, Pixels) {
        return (self.origin.y, self.limits.y);
    }

    fn collides(&self, ball: &ball::Ball) -> CollisionResult {
        collision::<Wall>(&self, &ball).map(|v| v * self.bounce_direction)
    }
}

impl<T> Renderable<T> for Wall
where
    T: RenderTarget,
{
    fn render(&self, canvas: &mut Canvas<T>) -> Result<(), failure::Error> {
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(127, 127, 127, 255));
        canvas
            .fill_rect(Rect::new(
                self.origin.x as i32,
                self.origin.y as i32,
                (self.limits.x - self.origin.x) as u32,
                (self.limits.y - self.origin.y) as u32,
            ))
            .map_err(err_msg)?;
        Ok(())
    }
}
