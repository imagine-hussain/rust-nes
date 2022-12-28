/// # Emulation Structure for the Picture Processing Unit (PPU)
///
/// Can generate 240 lines of pixels.
///
/// <https://www.nesdev.org/wiki/PPU>
pub struct Ppu {
    pattern: [u8; 8 * 1024],
    name_table: [u8; 2 * 1024],
    palette: [u8; 32],
}

