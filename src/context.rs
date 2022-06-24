use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::borrow::BorrowMut;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::image_canvas::Canvas;

pub fn display_image_on_screen(image: Canvas) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (w, h) = image.get_size();

    let window = video_subsystem
        .window("rust-sdl2 demo", w as u32, h as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let image_canvas = image.get_map();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        for (i, x_vec) in image_canvas.iter().enumerate() {
            for (j, c) in x_vec.into_iter().enumerate() {
                canvas.set_draw_color(Color::RGB(*c, *c, *c));

                canvas.draw_point(Point::new(w as i32 - j as i32, h as i32 - i as i32));
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn display_threaded_image_on_screen(image_arc_mutex: Arc<Mutex<Canvas>>) {
    let mut image = image_arc_mutex.deref().lock().unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (w, h) = image.get_size();

    let window = video_subsystem
        .window("rust-sdl2 demo", w as u32, h as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let image_canvas = image.get_map();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        for (i, x_vec) in image_canvas.iter().enumerate() {
            for (j, c) in x_vec.into_iter().enumerate() {
                canvas.set_draw_color(Color::RGB(*c, *c, *c));

                canvas.draw_point(Point::new(w as i32 - j as i32, h as i32 - i as i32));
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
