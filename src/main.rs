type Pixels = u32;
const BRICK_WIDTH: Pixels = 150;
const BRICK_HEIGHT: Pixels = 75;
const BRICK_V_PAD: Pixels = 10;
const BRICK_H_PAD: Pixels = 10;

const BALL_RADIUS: Pixels = 100;

struct Brick {
    x: u32,
    y: u32, 
}

impl Brick {
    fn get_x(&self) -> (u32, u32) {
        let xg: u32 = (self.x * BRICK_WIDTH + (self.x + 1) * BRICK_H_PAD) as u32;
        let xd: u32 = ((self.x + 1) * BRICK_WIDTH + (self.x + 1) * BRICK_H_PAD) as u32;
        (xg, xd)
    }
    fn get_y(&self) -> (u32, u32) {
        let yh: u32 = (self.y * BRICK_HEIGHT + (self.y + 1) * BRICK_V_PAD) as u32;
        let yb: u32 = ((self.y + 1) * BRICK_HEIGHT + (self.y + 1) * BRICK_V_PAD) as u32;
        (yh, yb)
    }
}

struct Ball {
    x: u32,
    y: u32,
}

impl Ball {
    fn collides(&self, brick: &Brick) -> bool {
        let (xg, xd) = brick.get_x();
        let (yh, yb) = brick.get_y();
        if self.x > xg && self.x < xd &&
           self.y > yh && self.y < yb {
            return true;
        }
        return false;
    }
}

fn main() {
    let w: u32 = 10;
    let h: u32 = 6;

    let mut bricks: Vec<Brick> = vec![];
    for x in 0..h {
        for y in 0..w {
            bricks.push(Brick {x: x, y: y});
        }
    }
    let ball: Ball = Ball{x: 100, y: 100};

    for brick in &bricks {
        print!("{}, {}\t", brick.x, brick.y);
    }
    println!();

    let mut remove: i64 = -1;
    for (i, brick) in bricks.iter().enumerate() {
        if ball.collides(brick) {
            println!("{}, {}", brick.x, brick.y);
            remove = i as i64;
        }
    }
    bricks.remove(remove as usize);

    for brick in &bricks {
        print!("{}, {}\t", brick.x, brick.y);
    }
    println!();
}

fn indices(index: u32, height: u8, width: u8) -> (u8, u8) {
    assert_eq!(height, 6);
    assert_eq!(width, 10);
    let x: u8 = (index / width as u32) as u8;
    let y: u8 = (index % width as u32) as u8;
    (x, y)
}
