use joypad::Joypad;
use gpu::color::Color;

pub trait Render {
    fn render(&mut self);
}

pub trait Input {
    fn get_input(&mut self) -> &mut Joypad;
}

pub trait Running {
    fn should_run(&self) -> bool;
}

pub trait PixelMapper {
    fn map_pixel(&mut self, x: u8, y: u8, color: Color);
    fn get_pixel(&self, x: u8, y: u8) -> Color;
}