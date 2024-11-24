#[derive(Clone, Copy, Debug, Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    pub fn write_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xF0) as u8; // F レジスタは下位 4 bit は常に 0
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    pub fn write_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = (val & 0xFF) as u8;
    }

    pub fn de(&self) -> u16 {
        ((self.e as u16) << 8) | (self.e as u16)
    }
    pub fn write_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = (val & 0xFF) as u8;
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    pub fn write_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0xFF) as u8;
    }

    pub fn zf(&self) -> bool {
        (self.f & 0b_1000_0000) > 0
    }
    pub fn write_zf(&mut self, zf: bool) {
        match zf {
            true => self.f |= 0b_1000_0000,
            false => self.f &= 0b_0111_1111,
        }
    }

    pub fn nf(&self) -> bool {
        (self.f & 0b_0100_0000) > 0
    }
    pub fn write_nf(&mut self, nf: bool) {
        match nf {
            true => self.f |= 0b_0100_0000,
            false => self.f &= 0b_1011_1111,
        }
    }

    pub fn hf(&self) -> bool {
        (self.f & 0b_0010_0000) > 0
    }
    pub fn write_hf(&mut self, hf: bool) {
        match hf {
            true => self.f |= 0b_0010_0000,
            false => self.f &= 0b_1101_1111,
        }
    }

    pub fn cf(&self) -> bool {
        (self.f & 0b_0001_0000) > 0
    }
    pub fn write_cf(&mut self, cf: bool) {
        match cf {
            true => self.f |= 0b_0001_0000,
            false => self.f &= 0b_1110_1111,
        }
    }
}
