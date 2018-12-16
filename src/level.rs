use std::fs::File;
use std::io::Read;

use brick::*;
use failure::{err_msg, Error};
use utils::{Pixels, Point};
use wall::WALL_THICKNESS;

#[derive(Deserialize, Serialize)]
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

    fn load(body: &str) -> Result<Self, Error> {
        serde_json::from_str(&body).map_err(err_msg)
    }

    pub fn load_file(filename: &str) -> Result<Self, Error> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Level::load(&contents)
    }
}

impl Default for Level {
    fn default() -> Self {
        let mut bricks = vec![];
        let offset = Point {
            x: BRICK_WIDTH / 2. + BRICK_V_PAD + WALL_THICKNESS,
            y: BRICK_HEIGHT / 2. + BRICK_H_PAD + WALL_THICKNESS,
        };

        for i in 0..10 {
            for j in 0..6 {
                let center = Point {
                    x: i as Pixels * (BRICK_WIDTH + BRICK_V_PAD),
                    y: j as Pixels * (BRICK_HEIGHT + BRICK_H_PAD),
                };

                bricks.push(Brick::new(BrickType::Simple, center + offset));
            }
        }

        Level { bricks }
    }
}
