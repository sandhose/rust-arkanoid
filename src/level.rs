use std::fs::File;
use std::io::Read;

use brick::*;
use failure::{err_msg, Error};
use utils::{Pixels, Point};
use wall::WALL_THICKNESS;

#[derive(Deserialize, Serialize)]
pub struct Level {
    pub bricks: Vec<Brick>,
    height: Pixels,
    width: Pixels,
}

impl Level {
    pub fn height(&self) -> u32 {
        self.height as u32
    }

    pub fn width(&self) -> u32 {
        self.width as u32
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
        const BRICK_WIDTH: Pixels = 32.0;
        const BRICK_HEIGHT: Pixels = 16.0;
        const BRICK_V_PAD: Pixels = 2.0;
        const BRICK_H_PAD: Pixels = 2.0;
        const N: usize = 25;
        const M: usize = 13;

        let mut bricks = vec![];
        let offset = Point {
            x: BRICK_WIDTH / 2. + BRICK_V_PAD + WALL_THICKNESS,
            y: BRICK_HEIGHT / 2. + BRICK_H_PAD + WALL_THICKNESS,
        };

        for i in 0..N {
            for j in 0..M {
                let center = Point {
                    x: i as Pixels * (BRICK_WIDTH + BRICK_V_PAD),
                    y: j as Pixels * (BRICK_HEIGHT + BRICK_H_PAD),
                };

                bricks.push(Brick::new(
                    if i % 2 == 0 {
                        BrickType::Simple
                    } else {
                        BrickType::Hard
                    },
                    center + offset,
                    BRICK_WIDTH,
                    BRICK_HEIGHT,
                ));
            }
        }

        Level {
            bricks,
            width: (BRICK_WIDTH + BRICK_V_PAD) * N as f64 + WALL_THICKNESS * 2. + BRICK_V_PAD,
            height: ((BRICK_HEIGHT + BRICK_H_PAD) * M as f64 + WALL_THICKNESS) * 3.,
        }
    }
}
