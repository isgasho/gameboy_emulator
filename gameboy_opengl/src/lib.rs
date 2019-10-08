#[macro_use]
extern crate c_str_macro;
extern crate gameboy_core;
extern crate gl;
extern crate glutin;

mod screen;
mod shader;

use gameboy_core::Emulator;
use screen::Screen;
use std::thread;
use std::time::{Duration, SystemTime};

pub fn start(rom: Vec<u8>) {
    let mut screen = Screen::new();
    let joypad = screen.get_input();
    let mut emulator = Emulator::from_rom(rom, joypad);

    let frame_rate = 60f64;
    let frame_duration = Duration::from_millis((1000f64 * (1f64 / frame_rate)) as u64);

    while screen.should_run() {
        let start_time = SystemTime::now();

        loop {
            let vblank = emulator.emulate(&mut screen);
            screen.poll_input();
            if vblank {
                break;
            }
        }

        screen.render();

        let end_time = SystemTime::now();

        let last_frame_duration = end_time.duration_since(start_time).unwrap();

        if frame_duration >= last_frame_duration {
            let sleep_duration = frame_duration - last_frame_duration;
            thread::sleep(sleep_duration);
        }
    }
}