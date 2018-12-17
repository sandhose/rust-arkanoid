use rand::Rng;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, RenderTarget};
use std::collections::HashMap;

use ball::Ball;
use bonus::{ActiveBonus, BonusType, FallingBonus};
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
    player: Player,
    ball: Ball,
}

const BALL_SPEED: f64 = 400.;

impl State {
    pub fn new(level: Level) -> State {
        State {
            bricks: level.bricks.clone(),
            walls: Wall::make_walls(level.height() as f64, level.width() as f64),
            pit: Wall::pit(level.height() as f64, level.width() as f64),
            bonuses: Vec::new(),
            active_bonuses: Vec::new(),
            player: Player::new(Point::new(
                level.width() as f64 * 0.5,
                level.height() as f64 - 30.0,
            )),
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

    pub fn input(&mut self, input: f64) {
        self.player.input(input);
    }

    fn queue_bonus(&mut self, b: ActiveBonus) {
        self.active_bonuses.push(b);
    }

    fn activate_bonus(&mut self, bonus: BonusType) {
        match bonus {
            BonusType::Slow => self.queue_bonus(ActiveBonus::from(bonus)),
            BonusType::Expand => self.player.grow(),
        }
    }

    fn bonus_stack(&mut self, bonus: BonusType, count: usize) {
        match bonus {
            BonusType::Slow => {
                self.ball.velocity.norm = BALL_SPEED / (count + 1) as f64;
            }
            _ => {}
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
            if let Some(collision) = brick.shape().collide(&self.ball.shape()) {
                self.ball.bounce(collision);
                brick.damage();

                if !brick.alive() && rand::thread_rng().gen_bool(1. / 10.) {
                    self.bonuses.push(FallingBonus::random(self.ball.position));
                }
            }
        }

        self.bricks.retain(Brick::alive);

        if let Some(collision) = self.player.shape().collide(&self.ball.shape()) {
            self.ball.bounce(collision);
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
            self.activate_bonus(b);
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

        for (&bonus, &count) in active.iter() {
            self.bonus_stack(bonus, count);
        }

        for wall in &self.walls {
            // Check for collisions between walls and the ball
            if let Some(collision) = wall.shape.collide(&self.ball.shape()) {
                self.ball.bounce(collision);
            }

            // …and between the walls and the player
            if let Some(collision) = wall.shape.collide(&self.player.shape()) {
                self.player.bounce(collision);
            }
        }
    }
}
