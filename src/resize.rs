use utils::{Pixels, Point};

#[derive(Copy, Clone)]
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
    base: Size,
}

impl RenderContext {
    pub fn new(base: Size, size: Size) -> Self {
        let scale = Self::min_scale(base, size);
        RenderContext {
            scale: scale,
            offset: Size {
                width: 0,
                height: 0,
            },
            base: base,
        }
    }
    pub fn fit(&mut self, size: Size) {
        let scale = Self::min_scale(self.base, size);
        self.scale = scale as f64;
        self.offset = Size {
            width: ((size.width as f64 - self.base.width as f64 * scale) / 2.) as i32,
            height: ((size.height as f64 - self.base.height as f64 * scale) / 2.) as i32,
        };
    }
    fn min_scale(base: Size, size: Size) -> f64 {
        let prop = (
            size.width as f64 / base.width as f64,
            size.height as f64 / base.height as f64,
        );
        if prop.0 < prop.1 {
            return prop.0;
        } else {
            return prop.1;
        };
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
