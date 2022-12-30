use std::vec::Vec;

use super::{header::HeaderParseError, Header};

/// Program is in 16Kb Chunks
const PRG_CHUNK_SIZE: usize = 0x4000;
/// Characters chunks are 8Kb
const CHR_CHUNK_SIZE: usize = 0x2000;

/// # Cartridge
///
/// ## Description
/// Created from a `iNes` file, this struct contains all the information
/// needed to emulate the cartridge including metadata, game rom and, mappers.
pub struct Cartridge {
    // TODO: Fields
    pub virtual_program_memory: Vec<u8>,
    pub virtual_character_memory: Vec<u8>,
    pub mapper_id: u8,
    pub program_banks_count: u8,
    pub character_banks_count: u8,
    pub header: Header,
}

impl Cartridge {
    pub fn ppu_read() {}
    pub fn ppu_write() {}
    pub fn cpu_read() {}
    pub fn cpu_write() {}
}

///////////////////////////////////////////////////////////////////////////////

pub enum CartridgeParseError {
    InvalidHeader(HeaderParseError),
    ProgramRomCutsOff,
    CharacterRomCutsOff,
}

impl TryFrom<&[u8]> for Cartridge {
    type Error = CartridgeParseError;

    /// An iNES file consists of the following sections, in order:
    ///
    /// 1. Header (16 bytes)
    /// 2. Trainer, if present (0 or 512 bytes)
    /// 3. PRG ROM data (16384 * x bytes)
    /// 4. CHR ROM data, if present (8192 * y bytes)
    /// 5. PlayChoice INST-ROM, if present (0 or 8192 bytes)
    /// 6. PlayChoice PROM, if present (16 bytes Data, 16 bytes CounterOut) (this is often missing, see PC10 ROM-Images for details)
    /// 7. Some ROM-Images additionally contain a 128-byte (or sometimes 127-byte) title at the end of the file.
    fn try_from(bytestream: &[u8]) -> Result<Self, Self::Error> {
        use CartridgeParseError::*;

        let header: Header = match Header::try_from(bytestream) {
            Ok(header) => header,
            Err(e) => return Err(InvalidHeader(e)),
        };

        // TODO: Add support for doing something with the trainer
        let has_trainer = header.has_trainer();

        // Program Memory
        let bytestream = match has_trainer {
            true => &bytestream[16 + 512..],
            false => &bytestream[16..],
        };
        let prg_size = header.prg_rom_size as usize * PRG_CHUNK_SIZE;
        if bytestream.len() < prg_size {
            return Err(ProgramRomCutsOff);
        }
        let virtual_program_memory = bytestream[..prg_size].to_vec();

        // Character Memory
        let bytestream = &bytestream[prg_size..];
        let chr_size = header.prg_chr_size as usize * CHR_CHUNK_SIZE;
        if bytestream.len() < 8192 {
            return Err(CharacterRomCutsOff);
        }
        let virtual_character_memory = bytestream[..chr_size].to_vec();

        // TODO: Program banks, chracter banks,


        // TODO: Mapper
        let mapper_id = header.mapper_id();

        // TODO:
        Ok(Self {
            header,
            virtual_program_memory,
            virtual_character_memory,
            mapper_id,
            program_banks_count: 0,
            character_banks_count: 0,
        })
    }
}
