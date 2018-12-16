use ball;
use utils;

pub struct UpdateFrame {
    pub dt: f64,
    pub player_input: f64,
}

pub trait Renderable<T>
where
    T: sdl2::render::RenderTarget,
{
    fn render(&self, &mut sdl2::render::Canvas<T>) -> Result<(), failure::Error>;
}

pub trait Updatable {
    fn update(&mut self);
}

pub trait Collisionable {
    fn get_x(&self) -> (utils::Pixels, utils::Pixels);
    fn get_y(&self) -> (utils::Pixels, utils::Pixels);
    fn collides(&self, &ball::Ball) -> Option<utils::CollisionResult>;
}

pub trait Collide<T> {
    fn collide(&self, &T) -> Option<utils::Rad>;
}

// impl<T, U: Collide<T>> Collide<U> for T {
//     fn collide(&self, other: &U) -> Option<utils::Rad> {
//         other.collide(self)
//     }
// }
