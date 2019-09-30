#![feature(nll)]
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

mod cpu;
mod mmu;
mod gpu;
mod timer;
pub mod joypad;
pub mod emulator;

pub use emulator::Emulator;
pub use emulator::traits;
pub use gpu::color::Color;