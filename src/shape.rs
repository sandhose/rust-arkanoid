use traits::{Collide, Collisionable};
use brick::Brick;
use ball::{Ball, BALL_RADIUS};
use utils::{Pixels, Point, Rad, Vector, PI};

#[derive(Debug)]
pub struct Rect {
    center: Point,
    width: Pixels,
    height: Pixels,
}

impl From<&Brick> for Rect {
    fn from(brick: &Brick) -> Self {
        let (xg, xd) = brick.get_x();
        let (yh, yb) = brick.get_y();
        Rect {
            center: Point { x: (xg + xd) / 2., y: (yh + yb) / 2. },
            height: yb - yh,
            width: xd - xg,
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
            WallOrientation::Left if other.center.x - other.radius < self.position => Some(PI),
            WallOrientation::Bottom if other.center.y + other.radius > self.position => Some(0.),
            WallOrientation::Right if other.center.x + other.radius > self.position => Some(PI),
            _ => None,
        }
    }
}

impl Collide<Circle> for Circle {
    fn collide(&self, other: &Self) -> Option<Rad> {
        let distance = Vector::from(self.center - other.center);

        if distance.norm < self.radius + other.radius {
            Some(distance.angle + PI)
        } else {
            None
        }
    }
}

impl Collide<Point> for Circle {
    fn collide(&self, other: &Point) -> Option<Rad> {
        let distance = Vector::from(self.center - *other);

        if distance.norm < self.radius {
            Some(distance.angle + PI)
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
                println!("horizontal");
                Some(0.)
            } else {
                println!("vertical");
                Some(PI)
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
            width: self.width - other.radius * 2.,
            height: self.height + other.radius * 2.,
            center: self.center,
        };

        if let Some(_) = rect.collide(&other.center) {
            println!("first");
            return Some(0.);
        }

        let rect = Rect {
            width: self.width + other.radius * 2.,
            height: self.height - other.radius * 2.,
            center: self.center,
        };

        if let Some(_) = rect.collide(&other.center) {
            println!("second");
            return Some(PI);
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
                println!("corner");
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
