extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{ Rect, Point };
use std::thread;

struct Snake {
    velocity: u32,
    tail_len: u8,
    parts: Vec<SnakeParts>
}

// Snake (x, y)
enum SnakeParts {
    Head(u32, u32),
    Tail(u32, u32)
}

enum Facing {
    Top,
    Bottom,
    Left,
    Right
}

fn main() {
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

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut timer = ctx.timer().expect("Could not create a timer!");
    let mut events = ctx.event_pump().expect("Event pump was not created!");
    let mut framerate = 0u16;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let mut rect = Rect::new(0, 0, 32, 32);

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(_), .. } => {
                    break 'running;
                },
                _ => ()
            }
        }

        canvas.fill_rect(rect);
        canvas.present();

        thread::sleep_ms(100);
    }
}
