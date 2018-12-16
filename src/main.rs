extern crate failure;
extern crate sdl2;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod ball;
pub mod brick;
// pub mod store; // Not used for now
pub mod level;
pub mod player;
pub mod shape;
pub mod state;
pub mod traits;
pub mod utils;
pub mod wall;

use level::Level;
use state::State;
use traits::*;
use utils::Pixels;

use failure::{err_msg, Error};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

fn init(width: Pixels, height: Pixels) -> Result<(Sdl, Canvas<Window>, EventPump), Error> {
    let sdl_context = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl_context.video().map_err(err_msg)?;

    let window = video_subsystem
        .window("Arkanoid", height as u32, width as u32)
        .position_centered()
        //.resizable()
        //.allow_highdpi()
        .build()?;

    let mut canvas = window.into_canvas().accelerated().present_vsync().build()?;
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().map_err(err_msg)?;
    Ok((sdl_context, canvas, event_pump))
}

fn main() {
    let level = Level::load_file("levels/default.json").expect("Could not load level file");
    let (_sdl_context, mut canvas, mut event_pump) = init(level.height(), level.width()).unwrap();
    let mut state = State::new(level);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    state.ball.velocity.norm *= 1.5;
                },

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    state.ball.velocity.norm /= 1.5;
                },

                _ => {}
            }
        }

        state.update();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        state.render(&mut canvas).unwrap();
        canvas.present();
    }
}
