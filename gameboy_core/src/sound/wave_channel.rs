use crate::bit_utils;

pub struct WaveChannel {
    dac_enabled: bool,
    length_load: u8,
    timer_load: u16,
    length_enable: bool,
    trigger_bit: bool,
    volume_code: u8,
    wave_table: [u8; 16],
    length_counter: u16,
    enabled: bool,
    timer: i32,
    position_counter: u8,
    output_vol: u8,
}

impl WaveChannel {
    pub fn new() -> WaveChannel {
        WaveChannel {
            dac_enabled: false,
            length_load: 0,
            timer_load: 0,
            length_enable: false,
            trigger_bit: false,
            volume_code: 0,
            wave_table: [0; 16],
            length_counter: 0,
            enabled: false,
            timer: 0,
            position_counter: 0,
            output_vol: 0,
        }
    }

    pub fn step(&mut self) {
        self.timer -= 1;
        if self.timer <= 0 {
            self.timer = (2048 - self.timer_load as i32) * 2;
            self.position_counter = (self.position_counter + 1) & 0x1F;
            if self.enabled && self.dac_enabled {
                let position = self.position_counter / 2;
                let mut output_byte = self.wave_table[position as usize];
                let high_bit = !bit_utils::is_set(self.position_counter, 0);
                if high_bit {
                    output_byte >>= 4;
                }
                output_byte &= 0xF;
                if self.volume_code > 0 {
                    output_byte >>= self.volume_code - 1;
                } else {
                    output_byte = 0;
                }
                self.output_vol = output_byte;
            } else {
                self.output_vol = 0;
            }
        }
    }

    pub fn length_click(&mut self) {
        if self.length_counter > 0 && self.length_enable {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF1A => {
                let dac_enabled = if self.dac_enabled { 1 } else { 0 };
                dac_enabled << 7
            }
            0xFF1B => self.length_load,
            0xFF1C => self.volume_code << 5,
            0xFF1D => (self.timer_load & 0xFF) as u8,
            0xFF1E => {
                let length_enable = if self.length_enable { 1 } else { 0 };
                let trigger_bit = if self.trigger_bit { 1 } else { 0 };
                ((self.timer_load >> 8) & 0x07) as u8 | (length_enable << 6) | (trigger_bit << 7)
            }
            0xFF30..=0xFF3F => self.wave_table[(address - 0xFF30) as usize],
            _ => panic!("unknown address: {:04X}", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF1A => self.dac_enabled = bit_utils::is_set(value, 7),
            0xFF1B => self.length_load = value,
            0xFF1C => self.volume_code = (value >> 5) & 0x03,
            0xFF1D => self.timer_load = (self.timer_load & 0x700) | value as u16,
            0xFF1E => {
                self.timer_load = (self.timer_load & 0xFF) | ((value as u16 & 0x7) << 8);
                self.length_enable = bit_utils::is_set(value, 6);
                self.trigger_bit = bit_utils::is_set(value, 7);
                if self.trigger_bit {
                    self.trigger();
                }
            }
            0xFF30..=0xFF3F => self.wave_table[(address - 0xFF30) as usize] = value,
            _ => panic!("unknown address: {:04X}", address),
        }
    }

    fn trigger(&mut self) {
        self.enabled = true;
        if self.length_counter == 0 {
            self.length_counter = 256;
        }
        self.timer = (2048 - self.timer_load as i32) * 2;
        self.position_counter = 0;
    }

    pub fn get_status(&self) -> bool {
        self.enabled && self.dac_enabled
    }

    pub fn reset_length_counter(&mut self) {
        self.length_counter = 0;
    }

    pub fn get_output_vol(&self) -> u8 {
        self.output_vol
    }
}
