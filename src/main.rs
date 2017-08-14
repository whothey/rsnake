extern crate sdl2;
extern crate rand;

mod snake;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{ Rect, Point };
use std::thread;
use std::time::Duration;
use snake::*;
use rand::distributions::IndependentSample;

// Tiles are 32x32
const TILE: u8 = 32;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    // TODO: This look awkward
    let BLACK: Color = Color::RGB(0, 0, 0);
    let WHITE: Color = Color::RGB(0xff, 0xff, 0xff);
    let RED:   Color = Color::RGB(0xff, 0x00, 0x00);
    let GREEN: Color = Color::RGB(0x00, 0xff, 0x00);

    // Random number generators
    let mut rng = rand::thread_rng();
    let rand_range_w = rand::distributions::Range::new(0u32, WINDOW_WIDTH / TILE as u32 - 1u32);
    let rand_range_h = rand::distributions::Range::new(0u32, WINDOW_HEIGHT / TILE as u32 - 1u32);

    let ctx   = sdl2::init().expect("Could not create a SDL2 context");
    let video = ctx.video().expect("Could not create a video subsystem");

    let window = video.window("RSnake", WINDOW_WIDTH, WINDOW_HEIGHT)
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

    let mut timer  = ctx.timer().expect("Could not create a timer!");
    let mut events = ctx.event_pump().expect("Event pump was not created!");
    // TODO: Calc framerate based on thread::sleep
    let mut framerate = 0u16;

    // Game Objects
    let mut snake = Snake::new(1, GREEN);
    let mut apple = Rect::new(
        rand_range_w.ind_sample(&mut rng) as i32 * TILE as i32,
        rand_range_h.ind_sample(&mut rng) as i32 * TILE as i32,
        TILE as u32, TILE as u32
    );

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

        canvas.set_draw_color(BLACK);
        canvas.clear();

        canvas.set_draw_color(RED);
        canvas.fill_rect(apple);

        snake.draw(&mut canvas);

        canvas.present();

        // TODO: snake must teleport when move beyond the window
        snake.move_forward();

        // Nhack!
        if apple.has_intersection(*snake.head()) {
            let newx = rand_range_w.ind_sample(&mut rng) as i32 * TILE as i32;
            let newy = rand_range_h.ind_sample(&mut rng) as i32 * TILE as i32;

            snake.tail_grow();

            apple.set_x(newx);
            apple.set_y(newy);
        }

        thread::sleep(Duration::from_millis(100));
    }
}
