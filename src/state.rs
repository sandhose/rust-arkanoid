use rand::Rng;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, RenderTarget};
use std::collections::HashMap;

use ball::Ball;
use bonus::{ActiveBonus, FallingBonus};
use brick::Brick;
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
    bonuses: Vec<FallingBonus>,
    active_bonuses: Vec<ActiveBonus>,
    pub player: Player,
    pub ball: Ball,
}

pub const BALL_SPEED: f64 = 500.;

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

    pub fn queue_bonus(&mut self, b: ActiveBonus) {
        self.active_bonuses.push(b);
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
                        norm: depth,
                    });
                brick.damage();

                if rand::thread_rng().gen_bool(1. / 10.) {
                    self.bonuses.push(FallingBonus::random(self.ball.position));
                }
            }
        }

        self.bricks.retain(Brick::alive);

        if let Some((normal, depth)) = self.player.shape().collide(&self.ball.shape()) {
            self.ball.velocity = self.ball.velocity | normal;
            self.ball.position = self.ball.position
                + Point::from(Vector {
                    angle: normal,
                    norm: depth,
                });
        }

        // Make the bonuses fall
        for ref mut bonus in &mut self.bonuses {
            bonus.update(dt);
        }

        // Check for collisions on bonuses
        let pit = self.pit.shape.clone();
        let player = self.player.shape();
        let mut to_activate = Vec::new();
        self.bonuses.retain(|b| {
            // …with the pit (just destroy them)
            if pit.collide(&b.shape()).is_some() {
                false
            } else if player.collide(&b.shape()).is_some() {
                // …with the player (activate them)
                to_activate.push(b.bonus_type);
                false
            } else {
                true
            }
        });

        for b in to_activate {
            b.activate(self);
        }

        // Update the timer on the active bonuses
        for ref mut bonus in &mut self.active_bonuses {
            bonus.update(dt);
        }

        // Build a hashmap to count the number of active bonus for each type
        let active = {
            let mut m = HashMap::new();
            for bonus in &self.active_bonuses {
                let e = m.entry(bonus.bonus_type).or_insert(0);
                if bonus.active() {
                    *e += 1;
                }
            }
            m
        };

        for (bonus_type, &count) in active.iter() {
            bonus_type.stack(self, count);
        }

        for wall in &self.walls {
            // Check for collisions between walls and the ball
            if let Some((normal, depth)) = wall.shape.collide(&self.ball.shape()) {
                self.ball.velocity = self.ball.velocity | normal;
                self.ball.position = self.ball.position
                    + Point::from(Vector {
                        angle: normal,
                        norm: depth,
                    });
            }

            // …and between the walls and the player
            if let Some((normal, depth)) = wall.shape.collide(&self.player.shape()) {
                self.player.velocity = -self.player.velocity;
                self.player.position.x = self.player.position.x
                    + Vector {
                        angle: normal,
                        norm: depth,
                    }
                    .x();
            }
        }
    }
}
