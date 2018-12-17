use resize;
use utils;

pub trait Renderable<T>
where
    T: sdl2::render::RenderTarget,
{
    fn render(
        &self,
        &mut sdl2::render::Canvas<T>,
        &resize::RenderContext,
        &sdl2::render::Texture,
    ) -> Result<(), failure::Error>;
}

pub trait Updatable {
    fn update(&mut self, dt: f64);
}

pub type Collision = (utils::Rad, utils::Pixels);

pub trait Collide<T> {
    fn collide(&self, &T) -> Option<Collision>;
}
