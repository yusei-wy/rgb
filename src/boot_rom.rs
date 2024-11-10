pub struct BootROM {
    rom: Box<[u8]>,
}

impl BootROM {
    pub fn new(rom: Box<[u8]>) -> Self {
        Self { rom }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
}
