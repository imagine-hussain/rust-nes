#[derive(Debug)]
pub struct Cartridge {
    // TODO: Fields
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge { }
    }

    pub fn ppu_read() { }
    pub fn ppu_write() { }
    pub fn cpu_read() { }
    pub fn cpu_write() { }

}

impl Default for Cartridge {
    fn default() -> Self {
        Self::new()
    }
}


