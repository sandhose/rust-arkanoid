use ball::{Ball, BALL_RADIUS};
use brick::{Brick, BRICK_HEIGHT, BRICK_WIDTH};
use player::{Player, PLAYER_THICKNESS, PLAYER_WIDTH};
use traits::Collide;
use utils::{Pixels, Point, Rad, Vector, PI};

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
            height: BRICK_HEIGHT,
            width: BRICK_WIDTH,
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

pub enum WallOrientation {
    Top,
    Bottom,
    Right,
    Left,
}

pub struct InfiniteWall {
    pub orientation: WallOrientation,
    pub position: Pixels,
}

impl Collide<Circle> for InfiniteWall {
    fn collide(&self, other: &Circle) -> Option<Rad> {
        match self.orientation {
            WallOrientation::Top if other.center.y - other.radius < self.position => Some(0.),
            WallOrientation::Left if other.center.x - other.radius < self.position => Some(PI / 2.),
            WallOrientation::Bottom if other.center.y + other.radius > self.position => Some(0.),
            WallOrientation::Right if other.center.x + other.radius > self.position => {
                Some(PI / 2.)
            }
            _ => None,
        }
    }
}

impl Collide<Rect> for InfiniteWall {
    fn collide(&self, other: &Rect) -> Option<Rad> {
        match self.orientation {
            WallOrientation::Top if other.center.y - other.height / 2. < self.position => Some(0.),
            WallOrientation::Left if other.center.x - other.width / 2. < self.position => {
                Some(PI / 2.)
            }
            WallOrientation::Bottom if other.center.y + other.height / 2. > self.position => {
                Some(0.)
            }
            WallOrientation::Right if other.center.x + other.width / 2. > self.position => {
                Some(PI / 2.)
            }
            _ => None,
        }
    }
}

impl Collide<Circle> for Circle {
    fn collide(&self, other: &Self) -> Option<Rad> {
        let distance = Vector::from(self.center - other.center);

        if distance.norm < self.radius + other.radius {
            Some(distance.angle)
        } else {
            None
        }
    }
}

impl Collide<Point> for Circle {
    fn collide(&self, other: &Point) -> Option<Rad> {
        let distance = Vector::from(self.center - *other);

        if distance.norm < self.radius {
            Some(distance.angle)
        } else {
            None
        }
    }
}

impl Collide<Point> for Rect {
    fn collide(&self, other: &Point) -> Option<Rad> {
        let diff = *other - self.center;
        let d_top = self.height / 2. + diff.y;
        let d_bottom = self.height / 2. - diff.y;
        let d_right = self.width / 2. + diff.x;
        let d_left = self.width / 2. - diff.x;

        if d_top > 0. && d_bottom > 0. && d_right > 0. && d_left > 0. {
            let l = [d_top, d_bottom, d_right, d_left];
            let max = l.iter().fold(100., |a, &b| f64::min(a, b));
            if d_top == max || d_bottom == max {
                Some(0.)
            } else {
                Some(PI / 2.)
            }
        } else {
            None
        }
    }
}

impl Collide<Circle> for Rect {
    fn collide(&self, other: &Circle) -> Option<Rad> {
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

        if let Some(_) = rect.collide(&other.center) {
            return Some(0.);
        }

        let rect = Rect {
            width: self.width + other.radius * 2.,
            height: self.height,
            center: self.center,
        };

        if let Some(_) = rect.collide(&other.center) {
            return Some(PI / 2.);
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
                // Bottom left
                center: Point {
                    x: self.center.x - self.width / 2.,
                    y: self.center.y + self.height / 2.,
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
    fn collide(&self, other: &Rect) -> Option<Rad> {
        other.collide(self)
    }
}
