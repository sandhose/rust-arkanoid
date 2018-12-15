extern crate failure;
extern crate sdl2;

pub mod ball;
pub mod brick;
pub mod traits;
pub mod utils;

use traits::*;

use failure::{err_msg, Error};
use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

const BRICK_COL: u32 = 10;
const BRICK_ROW: u32 = 6;

fn init() -> Result<(Sdl, Canvas<Window>, EventPump, Vec<brick::Brick>), Error> {
    let sdl_context = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl_context.video().map_err(err_msg)?;

    let window = video_subsystem
        .window(
            "Arkanoid",
            (brick::BRICK_WIDTH * (BRICK_COL as f32) +
                brick::BRICK_H_PAD * ((BRICK_COL as f32) + 1.0))
            as u32,
            (brick::BRICK_HEIGHT * (BRICK_ROW as f32) +
                brick::BRICK_V_PAD * ((BRICK_ROW as f32) + 1.0) + 300.0)
            as u32,
        )
        .position_centered()
        //.resizable()
        .allow_highdpi()
        .build()?;

    let mut canvas = window.into_canvas().accelerated().present_vsync().build()?;
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().map_err(err_msg)?;

    let mut bricks: Vec<brick::Brick> = vec![];
    for x in 0..BRICK_COL {
        for y in 0..BRICK_ROW {
            bricks.push(
                brick::BrickFactory::simple_brick(
                    x as utils::Pixels, y as utils::Pixels
                )
            );
        }
    }
    let bricks = bricks;
    
    Ok((sdl_context, canvas, event_pump, bricks))
}

fn main() {
    let (_sdl_context, mut canvas, mut event_pump, mut bricks) = init().unwrap();

    let mut ball: ball::Ball = ball::Ball {
        position: utils::Point {x: 100.0, y: 100.0},
        speed: utils::Point {x: 1.0, y: 1.0},
        color: Color::RGBA(120, 120, 200, 230)
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => break 'running,

                _ => {}
            }
        }

        let mut remove: i64 = -1;
        for (i, brick) in bricks.iter().enumerate() {
            let collision_result: (bool, (f32, f32)) = brick.collides(&ball);
            if collision_result.0 {
                ball.bounce(
                    utils::Point {
                        x: (collision_result.1).0,
                        y: (collision_result.1).1,
                    }
                );
                remove = i as i64;
            }
        }
        if remove > 0 && remove < (bricks.len() as i64) {
            bricks.remove(remove as usize);
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for brick in &bricks {
            let result = brick.render(&mut canvas);
        }
        ball.update();
        let result = ball.render(&mut canvas);
        canvas.present();
    }
}
