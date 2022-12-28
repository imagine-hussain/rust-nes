/// # Emulation Structure for the Picture Processing Unit (PPU)
///
/// <https://www.nesdev.org/wiki/PPU>
pub struct Ppu {
    memory: [u8; 10 * 1024],
    // pattern: [u8; 8 * 1024],
    // name_table: [u8; 2 * 1024],
    // palette: [u8; 32],
}

impl Ppu {

    pub fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

}

