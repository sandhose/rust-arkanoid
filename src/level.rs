use brick::{Brick, BrickType};
use utils::Pixels;

pub struct Level {
    pub bricks: Vec<Brick>,
}

impl Level {
    pub fn height(&self) -> Pixels {
        600.0
    }

    pub fn width(&self) -> Pixels {
        800.0
    }
}

impl Default for Level {
    fn default() -> Self {
        let mut bricks = vec![];
        for i in 0..10 {
            for j in 0..6 {
                bricks.push(Brick::new(BrickType::Simple, i as f32, j as f32));
            }
        }

        Level { bricks }
    }
}
