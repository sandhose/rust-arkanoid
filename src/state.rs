use ball::Ball;
use brick::Brick;
use level::Level;
use player::Player;
use resize::RenderContext;
use traits::{Collide, Renderable, Updatable};
use utils::{Point, Vector};
use wall::Wall;

use sdl2::pixels::Color;
use sdl2::render::{Canvas, RenderTarget};

pub struct State {
    bricks: Vec<Brick>,
    walls: Vec<Wall>,
    pub player: Player,
    pub ball: Ball,
}

impl State {
    pub fn new(level: Level) -> State {
        State {
            bricks: level.bricks.clone(),
            walls: Wall::make_walls(level.height(), level.width()),
            player: Player {
                position: Point {
                    x: level.width() * 0.5,
                    y: level.height() - 30.0,
                },
                velocity: 0.,
                acceleration: 0.,
                color: Color::RGBA(255, 0, 0, 255),
            },
            ball: Ball {
                position: Point { x: 100.0, y: 300.0 },
                velocity: Vector {
                    angle: std::f64::consts::PI / 4.0,
                    norm: 2.0,
                },
                acceleration: 0.0,
                color: Color::RGBA(120, 120, 200, 230),
            },
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new(Level::default())
    }
}

impl<T> Renderable<T> for State
where
    T: RenderTarget,
{
    fn render(
        &self,
        canvas: &mut Canvas<T>,
        context: &RenderContext,
    ) -> Result<(), failure::Error> {
        for brick in &self.bricks {
            brick.render(canvas, context)?;
        }
        for wall in &self.walls {
            wall.render(canvas, context)?;
        }
        self.player.render(canvas, context)?;
        self.ball.render(canvas, context)?;
        Ok(())
    }
}

impl Updatable for State {
    fn update(&mut self) {
        self.ball.update();
        self.player.update();

        let mut remove: i64 = -1;
        for (i, brick) in self.bricks.iter().enumerate() {
            if let Some(tangent) = brick.shape().collide(&self.ball.shape()) {
                self.ball.velocity = self.ball.velocity | tangent;
                remove = i as i64;
                break;
            }
        }
        if remove >= 0 && remove < (self.bricks.len() as i64) {
            self.bricks.remove(remove as usize);
        }

        if let Some(tangent) = self.player.shape().collide(&self.ball.shape()) {
            self.ball.velocity = self.ball.velocity | tangent;
        }

        for wall in &self.walls {
            if let Some(tangent) = wall.shape.collide(&self.ball.shape()) {
                self.ball.velocity = self.ball.velocity | tangent;
            }

            if let Some(_) = wall.shape.collide(&self.player.shape()) {
                self.player.velocity = -self.player.velocity;
            }
        }
    }
}
