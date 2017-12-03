pub mod flag;

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub f: flag::Flag,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
    pub ime: bool,
    pub cycles: u32
}

impl Registers {
    pub fn get_af(&self) -> u16 { self.f.bits() << 8 | self.a }
    pub fn set_af(&mut self, af: u16) {
        self.a = af as u8;
        self.f = flag::Flag::from_bits_truncate((af >> 8) as u8);
    }
    pub fn get_bc(&self) -> u16 { self.c << 8 | self.b }
    pub fn set_bc(&mut self, bc: u16) {
        self.b = bc as u8;
        self.c = (bc >> 8) as u8;
    }
    pub fn get_de(&self) -> u16 { self.e << 8 | self.d }
    pub fn set_de(&mut self, de: u16) {
        self.d = de as u8;
        self.e = (de >> 8) as u8;
    }
    pub fn get_hl(&self) -> u16 { self.l << 8 | self.h }
    pub fn set_hl(&mut self, hl: u16) {
        self.h = hl as u8;
        self.l = (hl >> 8) as u8;
    }
}