use resize;
use utils;

pub struct UpdateFrame {
    pub dt: f64,
    pub player_input: f64,
}

pub trait Renderable<T>
where
    T: sdl2::render::RenderTarget,
{
    fn render(
        &self,
        &mut sdl2::render::Canvas<T>,
        &resize::RenderContext,
    ) -> Result<(), failure::Error>;
}

pub trait Updatable {
    fn update(&mut self);
}

pub type Collision = (utils::Rad, utils::Pixels);

pub trait Collide<T> {
    fn collide(&self, &T) -> Option<Collision>;
}

// impl<T, U: Collide<T>> Collide<U> for T {
//     fn collide(&self, other: &U) -> Option<utils::Rad> {
//         other.collide(self)
//     }
// }
