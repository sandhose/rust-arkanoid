use ball;

pub struct UpdateFrame {
    pub dt: f64,
    pub player_input: f64,
}

pub trait Renderable<T>
where
    T: sdl2::render::RenderTarget,
{
    fn render(&self, &mut sdl2::render::Canvas<T>)
        -> Result<(), failure::Error>;
}

pub trait Updatable {
    fn update(&mut self);
}

pub trait Collisionable {
    fn collides(&self, &ball::Ball) -> (bool, f32);
}
