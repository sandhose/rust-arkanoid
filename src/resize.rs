use utils::{Pixels, Point};

pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub fn new(size: (u32, u32)) -> Self {
        Size {
            width: size.0 as i32,
            height: size.1 as i32,
        }
    }
}

pub struct RenderContext {
    scale: f64,
    offset: Size,
}

impl RenderContext {
    pub fn fit(size: Size) -> Self {
        let scale: i32 = std::cmp::min(size.width, size.height) - 1;
        RenderContext {
            scale: scale as f64,
            offset: Size {
                width: (size.width - scale) / 2,
                height: (size.height - scale) / 2,
            },
        }
    }
    pub fn scale(&self, size: Pixels) -> u32 {
        (size * self.scale) as u32
    }
    pub fn translate_point(&self, position: Point) -> Point {
        Point {
           x: self.scale * position.x + self.offset.width as f64,
           y: self.scale * position.y + self.offset.height as f64,
        }
    }
}
