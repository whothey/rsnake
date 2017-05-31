extern crate sdl2;

mod snake;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{ Rect, Point };
use std::thread;
use std::time::Duration;
use snake::*;

// Tiles are 32x32
const TILE: u8 = 32;

fn main() {
    let BLACK: Color = Color::RGB(0, 0, 0);
    let WHITE: Color = Color::RGB(0xff, 0xff, 0xff);
    let RED: Color   = Color::RGB(0xff, 0x00, 0x00);
    let GREEN: Color = Color::RGB(0x00, 0xff, 0x00);

    let ctx   = sdl2::init().expect("Could not create a SDL2 context");
    let video = ctx.video().expect("Could not create a video subsystem");

    let window = video.window("RSnake", 800, 600)
        .position_centered()
        .build()
        .expect("Could not create a window");

    let mut canvas = window.into_canvas()
        .accelerated()
        .build()
        .expect("Could not create a canvas!");

    canvas.set_draw_color(BLACK);
    canvas.clear();
    canvas.present();

    let mut timer = ctx.timer().expect("Could not create a timer!");
    let mut events = ctx.event_pump().expect("Event pump was not created!");
    // TODO: Calc framerate based on thread::sleep
    let mut framerate = 0u16;
    let mut snake = Snake::new(1, GREEN);

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    snake.look_to(Facing::Top);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    snake.look_to(Facing::Bottom);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    snake.look_to(Facing::Left);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    snake.look_to(Facing::Right);
                },
                Event::KeyDown { keycode: Some(_), .. } => {
                    break 'running;
                },
                _ => ()
            }
        }

        let ticks = timer.ticks();

        canvas.set_draw_color(BLACK);
        canvas.clear();

        snake.draw(&mut canvas);

        canvas.present();

        snake.move_forward();

        thread::sleep(Duration::from_millis(100));
    }
}
