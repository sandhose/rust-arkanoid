use rand::Rng;
use sdl2::render::{Canvas, RenderTarget, Texture};
use std::collections::HashMap;

use ball::{Ball, BALL_RADIUS};
use bonus::{ActiveBonus, BonusType, FallingBonus};
use brick::Brick;
use level::Level;
use player::{Player, PLAYER_THICKNESS};
use resize::RenderContext;
use traits::{Collide, Renderable, Updatable};
use utils::{Point, PI};
use wall::{Wall, WALL_THICKNESS};

const MAX_BALLS: usize = 16;

pub struct State {
    bricks: Vec<Brick>,
    walls: Vec<Wall>,
    pit: Wall,
    bonuses: Vec<FallingBonus>,
    active_bonuses: Vec<ActiveBonus>,
    player: Player,
    lives: u8,
    balls: Vec<Ball>,
}

const PLAYER_OFFSET: f64 = WALL_THICKNESS + PLAYER_THICKNESS / 2. + 10.;
const BALL_OFFSET: f64 = PLAYER_OFFSET + PLAYER_THICKNESS / 2. + BALL_RADIUS;

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
                level.height() as f64 - PLAYER_OFFSET,
            )),
            lives: 3,
            balls: vec![Ball::new(
                Point::new(
                    level.width() as f64 * 0.5,
                    level.height() as f64 - BALL_OFFSET,
                ),
                -PI / 4.0,
            )],
        }
    }

    pub fn alive(&self) -> bool {
        self.lives > 0
    }

    pub fn won(&self) -> bool {
        !self.bricks.iter().any(|b| b.breakable)
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
            BonusType::Divide => {
                let mut to_add = Vec::new();
                for ball in &self.balls {
                    let mut new = ball.clone();
                    new.rotate(PI / 6.);
                    to_add.push(new);
                    let mut new = ball.clone();
                    new.rotate(-PI / 6.);
                    to_add.push(new);
                }
                self.balls.extend(to_add);
                self.balls.truncate(MAX_BALLS);
            }
            BonusType::Life => {
                self.lives += 1;
            }
        }
    }

    fn bonus_stack(&mut self, bonus: BonusType, count: usize) {
        match bonus {
            BonusType::Slow => {
                for ref mut ball in &mut self.balls {
                    ball.speed(count);
                }
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
        texture: &Texture,
    ) -> Result<(), failure::Error> {
        for brick in &self.bricks {
            brick.render(canvas, context, texture)?;
        }
        for wall in &self.walls {
            wall.render(canvas, context, texture)?;
        }
        for bonus in &self.bonuses {
            bonus.render(canvas, context, texture)?;
        }
        for ball in &self.balls {
            ball.render(canvas, context, texture)?;
        }
        self.player.render(canvas, context, texture)?;
        Ok(())
    }
}

impl Updatable for State {
    fn update(&mut self, dt: f64) {
        // First, update all the balls
        for ref mut ball in &mut self.balls {
            ball.update(dt);
            // Force the position of balls that are on hold
            if ball.on_hold() {
                ball.set_position(
                    self.player.position() + Point::new(0., -(PLAYER_THICKNESS / 2. + BALL_RADIUS)),
                );
            }
        }

        // Update the player
        self.player.update(dt);

        // Check for collisions between bricks and balls
        for brick in &mut self.bricks {
            for ref mut ball in &mut self.balls {
                if let Some(collision) = brick.shape().collide(&ball.shape()) {
                    ball.bounce(collision);
                    brick.damage();

                    // Randomly spawn a new bonus
                    if !brick.alive() && rand::thread_rng().gen_bool(1. / 4.) {
                        self.bonuses.push(FallingBonus::random(brick.center));
                    }
                }
            }
        }

        // Remove bricks that were destroyed
        self.bricks.retain(Brick::alive);

        // Check for collisions between balls and the player
        for ref mut ball in &mut self.balls {
            if let Some(collision) = self.player.shape().collide(&ball.shape()) {
                ball.bounce(collision);
            }
        }

        // Make the bonuses fall
        for ref mut bonus in &mut self.bonuses {
            bonus.update(dt);
        }

        // Check for collisions on bonuses
        let pit = self.pit.shape.clone();
        let player = self.player.shape();
        let mut to_activate: Vec<BonusType> = Vec::new(); // List of bonuses to activate
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
        let active: HashMap<BonusType, usize> = {
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

        // Remove balls that collided with the pit
        self.balls.retain(|b| pit.collide(&b.shape()).is_none());

        if self.balls.is_empty() {
            // Lost a life
            self.lives -= 1;
            self.balls.push(Ball::new(Point::new(0., 0.), -PI / 4.0));
        }

        for wall in &self.walls {
            // Check for collisions between walls and the balls
            for ref mut ball in &mut self.balls {
                if let Some(collision) = wall.shape.collide(&ball.shape()) {
                    ball.bounce(collision);
                }
            }

            // …and between the walls and the player
            if let Some(collision) = wall.shape.collide(&self.player.shape()) {
                self.player.bounce(collision);
            }
        }
    }
}
