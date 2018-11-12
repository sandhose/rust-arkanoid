type Pixels = u32;
pub const BRICK_WIDTH: Pixels = 80;
pub const BRICK_HEIGHT: Pixels = 40;
pub const BRICK_V_PAD: Pixels = 5;
pub const BRICK_H_PAD: Pixels = 5;

pub struct Brick {
    pub x: u32,
    pub y: u32, 
}

impl Brick {
    pub fn get_x(&self) -> (u32, u32) {
        let xg: u32 = (self.x * BRICK_WIDTH + (self.x + 1) * BRICK_H_PAD) as u32;
        let xd: u32 = ((self.x + 1) * BRICK_WIDTH + (self.x + 1) * BRICK_H_PAD) as u32;
        (xg, xd)
    }
    pub fn get_y(&self) -> (u32, u32) {
        let yh: u32 = (self.y * BRICK_HEIGHT + (self.y + 1) * BRICK_V_PAD) as u32;
        let yb: u32 = ((self.y + 1) * BRICK_HEIGHT + (self.y + 1) * BRICK_V_PAD) as u32;
        (yh, yb)
    }
}
