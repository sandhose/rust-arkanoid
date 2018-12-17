use rand::Rng;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, RenderTarget};

use ball::Ball;
use brick::Brick;
use bonus::{Bonus, ActiveBonus, BonusType};
use level::Level;
use player::Player;
use resize::RenderContext;
use traits::{Collide, Renderable, Updatable};
use utils::{Point, Vector};
use wall::Wall;

pub struct State {
    bricks: Vec<Brick>,
    walls: Vec<Wall>,
    pit: Wall,
    bonuses: Vec<Bonus>,
    active_bonuses: Vec<ActiveBonus>,
    pub player: Player,
    pub ball: Ball,
}

const BALL_SPEED: f64 = 500.;

impl State {
    pub fn new(level: Level) -> State {
        State {
            bricks: level.bricks.clone(),
            walls: Wall::make_walls(level.height() as f64, level.width() as f64),
            pit: Wall::pit(level.height() as f64, level.width() as f64),
            bonuses: Vec::new(),
            active_bonuses: Vec::new(),
            player: Player {
                position: Point {
                    x: level.width() as f64 * 0.5,
                    y: level.height() as f64 - 30.0,
                },
                velocity: 0.,
                acceleration: 0.,
                color: Color::RGBA(255, 0, 0, 255),
            },
            ball: Ball {
                position: Point { x: 100.0, y: 350.0 },
                velocity: Vector {
                    angle: std::f64::consts::PI / 4.0,
                    norm: BALL_SPEED,
                },
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
        for bonus in &self.bonuses {
            bonus.render(canvas, context)?;
        }
        self.player.render(canvas, context)?;
        self.ball.render(canvas, context)?;
        Ok(())
    }
}

impl Updatable for State {
    fn update(&mut self, dt: f64) {
        self.ball.update(dt);
        self.player.update(dt);

        for brick in &mut self.bricks {
            if let Some((normal, depth)) = brick.shape().collide(&self.ball.shape()) {
                self.ball.velocity = self.ball.velocity | normal;
                self.ball.position = self.ball.position
                    + Point::from(Vector {
                        angle: normal,
                        norm: depth * 2.,
                    });
                brick.damage();

                if rand::thread_rng().gen_bool(1. / 10.) {
                    self.bonuses.push(Bonus::random(self.ball.position));
                }
            }
        }
        self.bricks.retain(|brick| {
            let delete = brick.breakable && brick.hitpoints == 0;
            !delete
        });

        if let Some((normal, depth)) = self.player.shape().collide(&self.ball.shape()) {
            self.ball.velocity = self.ball.velocity | normal;
            self.ball.position = self.ball.position
                + Point::from(Vector {
                    angle: normal,
                    norm: depth * 2.,
                });
        }

        for ref mut bonus in &mut self.bonuses {
            bonus.update(dt);
        }

        let pit = self.pit.shape.clone();
        let player = self.player.shape();
        let mut to_add = Vec::new();
        self.bonuses.retain(|b| {
            if pit.collide(&b.shape()).is_some() { return false }

            if player.collide(&b.shape()).is_some() {
                to_add.push(ActiveBonus::from(b));
                return false;
            }

            true
        });

        self.active_bonuses.extend(to_add);

        for ref mut bonus in &mut self.active_bonuses {
            bonus.update(dt);
        }

        self.active_bonuses.retain(ActiveBonus::active);

        let active_slow = self.active_bonuses.iter().filter(|b| b.bonus_type == BonusType::Slow).count();

        self.ball.velocity.norm = BALL_SPEED / (active_slow + 1) as f64;

        for wall in &self.walls {
            if let Some((normal, depth)) = wall.shape.collide(&self.ball.shape()) {
                self.ball.velocity = self.ball.velocity | normal;
                self.ball.position = self.ball.position
                    + Point::from(Vector {
                        angle: normal,
                        norm: depth * 2.,
                    });
            }

            if let Some((normal, depth)) = wall.shape.collide(&self.player.shape()) {
                self.player.velocity = -self.player.velocity;
                self.player.position.x = self.player.position.x
                    + Vector {
                        angle: normal,
                        norm: depth * 2.,
                    }
                    .x();
            }
        }
    }
}
