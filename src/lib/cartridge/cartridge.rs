use std::vec::Vec;

use log::debug;

use crate::{
    mappers::{select_mapper, Mapper000},
    Mapper,
};

use super::{header::HeaderParseError, Header};

/// Program is in 16Kb Chunks
// const PRG_CHUNK_SIZE: usize = 0x4000;
const PRG_CHUNK_SIZE: usize = 16384;
/// Characters chunks are 8Kb
// const CHR_CHUNK_SIZE: usize = 0x2000;
const CHR_CHUNK_SIZE: usize = 8192;

/// # Cartridge
///
/// ## Description
/// Created from a `iNes` file, this struct contains all the information
/// needed to emulate the cartridge including metadata, game rom and, mappers.
pub struct Cartridge {
    pub header: Header,
    pub virtual_program_memory: Vec<u8>,
    pub virtual_character_memory: Vec<u8>,
    pub mapper_id: u8,
    pub program_banks_count: u8,
    pub character_banks_count: u8,
    pub mapper: Mapper000,
}

impl Cartridge {
    pub fn ppu_read() {}
    pub fn ppu_write() {}

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        let mut new_address: u16 = 0;
        if self.mapper.map_cpu_read(address, &mut new_address) {
            self.virtual_program_memory[new_address as usize]
        } else {
            0
        }
    }

    pub fn cpu_write() {}
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum CartridgeParseError {
    InvalidHeader(HeaderParseError),
    ProgramRomCutsOff,
    CharacterRomCutsOff,
    FileError(std::io::Error),
}

impl TryFrom<Vec<u8>> for Cartridge {
    type Error = CartridgeParseError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let slice = &value[..];
        slice.try_into()
    }
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

        let cartridge_size = bytestream.len();
        let header: Header = match Header::try_from(&bytestream[..16]) {
            Ok(header) => header,
            Err(e) => return Err(InvalidHeader(e)),
        };

        debug!("Header: {:?}", header);

        let bytestream = &bytestream[16..];
        log_read_progress("Debug", bytestream, cartridge_size);

        // TODO: Do something other than just ignore the training data?
        let has_trainer = header.has_trainer();
        debug!("Has Trainer: {}", has_trainer);

        // Trainer
        let bytestream = match has_trainer {
            true => &bytestream[512..],
            false => &bytestream[..],
        };
        log_read_progress("Trainer", bytestream, cartridge_size);

        // PROGRAM_MEMORY
        let program_banks_count = header.prg_rom_size;
        let prg_size = program_banks_count as usize * PRG_CHUNK_SIZE;
        debug!("PRG Size: {}", prg_size);

        let virtual_program_memory = match bytestream.len() < prg_size {
            true => return Err(ProgramRomCutsOff),
            false => bytestream[..prg_size].to_vec(),
        };
        let bytestream = &bytestream[prg_size..];
        log_read_progress("PROG Memory", bytestream, cartridge_size);

        // Character Memory
        let character_banks_count = header.prg_chr_size;
        let chr_size = character_banks_count as usize * CHR_CHUNK_SIZE;
        let virtual_character_memory = match bytestream.len() < chr_size {
            true => Err(CharacterRomCutsOff),
            false => Ok(bytestream[..chr_size].to_vec()),
        }?;
        let bytestream = &bytestream[chr_size..];
        log_read_progress("Character Memory", bytestream, cartridge_size);

        // let bytestream = &bytestream[chr_size..];
        // debug!("at very end: remaining: {}", bytestream.len());

        let mapper_id = header.mapper_id();
        debug!("Mapper ID: {}", mapper_id);
        // Currently only supports Mapper 000
        let mapper = select_mapper(mapper_id, &header);

        // To Dos
        // TODO: Program banks, chracter banks,
        // TODO: Deal with the 3 diff file formats
        //      - Archaic iNES
        //      - iNES 0.7
        //      - iNES
        // TODO: Read rest of Flag6 and Flag7
        //

        Ok(Self {
            header,
            virtual_program_memory,
            virtual_character_memory,
            mapper_id,
            program_banks_count,
            character_banks_count,
            mapper,
        })
    }
}

fn log_read_progress(stage: &str, bytestream: &[u8], initial_size: usize) {
    let remaining = bytestream.len();
    let bytes_read = initial_size - remaining;
    debug!(
        "CartridgeDecode: Completed {} stage.\tRemaining bytes:{}. Read {} of {}",
        stage, remaining, bytes_read, initial_size
    );
    // debug!("{}: {}", stage, bytestream.len());
}
