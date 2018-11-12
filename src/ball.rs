use brick::Brick;

type Pixels = u32;
pub const BALL_RADIUS: Pixels = 60;

pub struct Ball {
    pub x: u32,
    pub y: u32,
}

impl Ball {
    pub fn collides(&self, brick: &Brick) -> bool {
        let (xg, xd) = brick.get_x();
        let (yh, yb) = brick.get_y();
        if self.x > xg && self.x < xd &&
           self.y > yh && self.y < yb {
            return true;
        }
        return false;
    }
}
