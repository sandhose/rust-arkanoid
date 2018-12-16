use std::fs::File;
use std::io::Read;

use brick::{Brick, BrickType};
use failure::{err_msg, Error};
use utils::Pixels;

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
        for i in 0..10 {
            for j in 0..6 {
                bricks.push(Brick::new(BrickType::Simple, i as Pixels, j as Pixels));
            }
        }

        Level { bricks }
    }
}
