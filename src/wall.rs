use ball;
use traits;
use utils::Point;

type Pixels = usize;
const WALL_THICKNESS: Pixels = 10;

pub struct Wall {
    origin: Point,
    limits: Point,
}

impl Wall {
    // Admettant un rectangle :
    // p1 - p2
    // |    |
    // p3 - p4
    pub fn get_boundaries(&self) -> (Point, Point, Point, Point) {
        let p1 = (self.origin.0, self.origin.1);
        let p2 = (self.origin.0, self.limits.1);
        let p3 = (self.limits.0, self.origin.1);
        let p4 = (self.limits.0, self.limits.1);
        return (p1, p2, p3, p4);
    }
}

// TODO :
// impl Updatable for Wall
// impl Renderable for Wall
// impl Collisionable for Wall

mod WallFactory {
    use super::*;

    fn top_wall(width: Pixels) -> Wall {
        return Wall {
            origin: (0, 0),
            limits: (width, WALL_THICKNESS),
        };
    }
    fn left_wall(height: Pixels) -> Wall {
        return Wall {
            origin: (0, 0),
            limits: (WALL_THICKNESS, height),
        };
    }
    fn right_wall(height: Pixels, width: Pixels) -> Wall {
        return Wall {
            origin: (width - WALL_THICKNESS, 0),
            limits: (width, height),
        };
    }
    fn pit(height: Pixels, width: Pixels) -> Wall {
        return Wall {
            origin: (0, height - WALL_THICKNESS),
            limits: (height, width),
        }
    }
    pub fn make_walls(height: Pixels, width: Pixels)
        -> (Wall, Wall, Wall, Wall)
    {
        (
            top_wall(),
            left_wall(height),
            right_wall(height, width),
            pit(height, width),
        )
    }
}
