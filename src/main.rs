extern crate failure;
extern crate sdl2;

pub mod ball;
pub mod brick;
pub mod traits;

use std::f64::consts::PI;
use std::time::Instant;

use traits::{UpdateFrame, Renderable, Updatable};

use failure::{err_msg, Error};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};
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
        .window("Arkanoid",
                (brick::BRICK_WIDTH*BRICK_COL + brick::BRICK_H_PAD*(BRICK_COL+1)),
                (brick::BRICK_HEIGHT*BRICK_ROW + brick::BRICK_V_PAD*(BRICK_ROW+1) + 300))
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
//            bricks.push(brick::Brick {x: x, y: y});
            bricks.push(brick::BrickFactory::simple_brick(x, y));
        }
    }
    let bricks = bricks;
    
    Ok((sdl_context, canvas, event_pump, bricks))
}

fn main() {
    let (_sdl_context, mut canvas, mut event_pump, mut bricks) = init().unwrap();

    let mut ball: ball::Ball = ball::Ball {
        position: (100, 100),
        speed: (1, 1),
        color: Color::RGBA(120, 120, 200, 230)
    };

    for brick in &bricks {
        print!("{}, {}\t", brick.x, brick.y);
    }
    println!();

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
            //match ball.collides(brick) {
            //    ball::Side::Right => println!("RIGHT"),
            //    ball::Side::Left => println!("LEFT"),
            //    ball::Side::Up => println!("UP"),
            //    ball::Side::Down => println!("DOWN"),
            //    ball::Side::No => println!("NONE"),
            //}
//            if ball.collides(brick) != Side::none {
//                ball.bounce(brick);
//                println!("{}, {}", brick.x, brick.y);
//                remove = i as i64;
//            }
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
