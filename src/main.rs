extern crate failure;
extern crate rand;
extern crate sdl2;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use failure::{err_msg, Error};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};
use std::time::Instant;

mod ball;
mod bonus;
mod brick;
mod level;
mod player;
mod resize;
mod shape;
mod state;
mod traits;
mod utils;
mod wall;

use level::Level;
use resize::{RenderContext, Size};
use state::State;
use traits::*;

fn init(width: u32, height: u32) -> Result<(Sdl, Canvas<Window>, EventPump), Error> {
    let sdl_context = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl_context.video().map_err(err_msg)?;

    let window = video_subsystem
        .window("Arkanoid", height, width)
        .position_centered()
        .resizable()
        .allow_highdpi()
        .build()?;

    let mut canvas = window.into_canvas().accelerated().present_vsync().build()?;
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().map_err(err_msg)?;
    Ok((sdl_context, canvas, event_pump))
}

fn main() {
    // println!("{}", serde_json::to_string(&Level::default()).unwrap());

    let level = Level::load_file("levels/default.json").expect("Could not load level file");
    let (_sdl_context, mut canvas, mut event_pump) = init(level.height(), level.width()).unwrap();
    let mut context = RenderContext::new(
        Size::new((level.width(), level.height())),
        Size::new(canvas.window().drawable_size()),
    );
    let mut state = State::new(level);

    let mut last_update = Instant::now();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        state.render(&mut canvas, &context).unwrap();
        canvas.present();

        let now = Instant::now();
        let dt = now.duration_since(last_update);
        let dt: f64 = dt.as_secs() as f64 + dt.subsec_nanos() as f64 * 1e-9;
        last_update = now;

        let player_input = {
            let keyboard_state = KeyboardState::new(&event_pump);
            let mut input = 0;
            if keyboard_state.is_scancode_pressed(Scancode::Left) {
                input -= 1;
            }
            if keyboard_state.is_scancode_pressed(Scancode::Right) {
                input += 1;
            }
            input as f64
        };

        state.input(player_input);
        state.update(dt);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::Window {
                    win_event: WindowEvent::SizeChanged(_, _),
                    ..
                }
                | Event::Window {
                    win_event: WindowEvent::Resized(_, _),
                    ..
                } => {
                    context.fit(Size::new(canvas.window().drawable_size()));
                }

                _ => {}
            }
        }
    }
}
