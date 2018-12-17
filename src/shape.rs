use ball::{Ball, BALL_RADIUS};
use bonus::FallingBonus;
use brick::Brick;
use player::{Player, PLAYER_THICKNESS, PLAYER_WIDTH};
use traits::{Collide, Collision};
use utils::{Pixels, Point, Rad, Vector, PI};

const UP: Rad = -PI / 2.;
const DOWN: Rad = PI / 2.;
const LEFT: Rad = PI;
const RIGHT: Rad = 0.;

#[derive(Debug)]
pub struct Rect {
    center: Point,
    width: Pixels,
    height: Pixels,
}

impl From<&Brick> for Rect {
    fn from(brick: &Brick) -> Self {
        Rect {
            center: brick.center,
            height: brick.height,
            width: brick.width,
        }
    }
}

impl From<&Player> for Rect {
    fn from(player: &Player) -> Self {
        Rect {
            center: player.position,
            height: PLAYER_THICKNESS,
            width: PLAYER_WIDTH,
        }
    }
}

#[derive(Debug)]
pub struct Circle {
    center: Point,
    radius: Pixels,
}

impl From<&Ball> for Circle {
    fn from(ball: &Ball) -> Self {
        Circle {
            center: ball.position,
            radius: BALL_RADIUS,
        }
    }
}

impl From<&FallingBonus> for Circle {
    fn from(bonus: &FallingBonus) -> Self {
        Circle {
            center: bonus.position,
            radius: 12.,
        }
    }
}

#[derive(Clone)]
pub enum WallOrientation {
    Top,
    Bottom,
    Right,
    Left,
}

#[derive(Clone)]
pub struct InfiniteWall {
    pub orientation: WallOrientation,
    pub center: Point,
}

impl Collide<Circle> for InfiniteWall {
    fn collide(&self, other: &Circle) -> Option<Collision> {
        match self.orientation {
            WallOrientation::Top if other.center.y - other.radius < self.center.y => Some((
                DOWN,
                (-(other.center.y - other.radius) + self.center.y).abs(),
            )),
            WallOrientation::Left if other.center.x - other.radius < self.center.x => Some((
                RIGHT,
                (-(other.center.x - other.radius) + self.center.x).abs(),
            )),
            WallOrientation::Bottom if other.center.y + other.radius > self.center.y => {
                Some((UP, ((other.center.y + other.radius) - self.center.y).abs()))
            }
            WallOrientation::Right if other.center.x + other.radius > self.center.x => Some((
                LEFT,
                ((other.center.x + other.radius) - self.center.x).abs(),
            )),
            _ => None,
        }
    }
}

impl Collide<Rect> for InfiniteWall {
    fn collide(&self, other: &Rect) -> Option<Collision> {
        match self.orientation {
            WallOrientation::Top if other.center.y - other.height / 2. < self.center.y => {
                Some((DOWN, -(other.center.y - other.height / 2.) + self.center.y))
            }
            WallOrientation::Left if other.center.x - other.width / 2. < self.center.x => {
                Some((RIGHT, -(other.center.x - other.width / 2.) + self.center.x))
            }
            WallOrientation::Bottom if other.center.y + other.height / 2. > self.center.y => {
                Some((UP, (other.center.y + other.height / 2.) - self.center.y))
            }
            WallOrientation::Right if other.center.x + other.width / 2. > self.center.x => {
                Some((LEFT, (other.center.x + other.width / 2.) - self.center.x))
            }
            _ => None,
        }
    }
}

impl Collide<Circle> for Circle {
    fn collide(&self, other: &Self) -> Option<Collision> {
        let distance = Vector::from(self.center - other.center);

        if distance.norm < self.radius + other.radius {
            Some((distance.angle, (self.radius + other.radius) - distance.norm))
        } else {
            None
        }
    }
}

impl Collide<Point> for Circle {
    fn collide(&self, other: &Point) -> Option<Collision> {
        let distance = Vector::from(self.center - *other);

        if distance.norm < self.radius {
            Some((distance.angle, self.radius - distance.norm))
        } else {
            None
        }
    }
}

impl Collide<Point> for Rect {
    fn collide(&self, other: &Point) -> Option<Collision> {
        let diff = *other - self.center;
        let d_top = self.height / 2. + diff.y;
        let d_bottom = self.height / 2. - diff.y;
        let d_right = self.width / 2. + diff.x;
        let d_left = self.width / 2. - diff.x;

        if d_top > 0. && d_bottom > 0. && d_right > 0. && d_left > 0. {
            let l = [d_top, d_bottom, d_right, d_left];
            let min = l.iter().fold(100., |a, &b| f64::min(a, b));
            if min == d_top {
                Some((UP, d_top))
            } else if d_bottom == min {
                Some((DOWN, d_bottom))
            } else if d_right == min {
                Some((RIGHT, d_right))
            } else if d_left == min {
                Some((LEFT, d_left))
            } else {
                panic!()
            }
        } else {
            None
        }
    }
}

impl Collide<Circle> for Rect {
    fn collide(&self, other: &Circle) -> Option<Collision> {
        let distance = Vector::from(self.center - other.center);
        let outer_radius = Point {
            x: self.width / 2.,
            y: self.height / 2.,
        }
        .norm();

        // Fast check
        if distance.norm > outer_radius + other.radius {
            return None;
        }

        let rect = Rect {
            width: self.width,
            height: self.height + other.radius * 2.,
            center: self.center,
        };

        // TODO: this might break
        if let Some(r) = rect.collide(&other.center) {
            return Some(r);
        }

        let rect = Rect {
            width: self.width + other.radius * 2.,
            height: self.height,
            center: self.center,
        };

        if let Some(r) = rect.collide(&other.center) {
            return Some(r);
        }

        let corners = [
            Circle {
                // Top left
                center: Point {
                    x: self.center.x - self.width / 2.,
                    y: self.center.y - self.height / 2.,
                },
                radius: other.radius,
            },
            Circle {
                // Top right
                center: Point {
                    x: self.center.x + self.width / 2.,
                    y: self.center.y - self.height / 2.,
                },
                radius: other.radius,
            },
            Circle {
                // Bottom left
                center: Point {
                    x: self.center.x - self.width / 2.,
                    y: self.center.y + self.height / 2.,
                },
                radius: other.radius,
            },
            Circle {
                // Bottom right
                center: Point {
                    x: self.center.x + self.width / 2.,
                    y: self.center.y + self.height / 2.,
                },
                radius: other.radius,
            },
        ];

        for corner in &corners {
            if let Some(res) = corner.collide(&other.center) {
                return Some(res);
            }
        }

        None
    }
}

impl Collide<Rect> for Circle {
    fn collide(&self, other: &Rect) -> Option<Collision> {
        other.collide(self)
    }
}
