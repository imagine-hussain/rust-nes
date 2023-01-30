use log::debug;

/// # Header of the iNES file format
///
/// ## Format
/// [Wiki Source](https://www.nesdev.org/wiki/INES#iNES_file_format>)
///
///
#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub name: [u8; 4],
    pub prg_rom_size: u8,
    pub prg_chr_size: u8,
    pub flag_6: u8,
    pub flag_7: u8,
    pub prg_ram_size: u8,
    pub tv_system_1: u8,
    pub tv_system_2: u8,
}

impl Header {
    /// Returns true if the header says the Cartridge contains
    /// trainer data.
    /// This information exists in the `3` byte of the `flag_6` field
    /// in the header.
    pub fn has_trainer(&self) -> bool {
        self.flag_6 & 0b0000_0100 != 0
    }

    /// Returns the mapper number by parsing flag 6 and 7
    pub fn mapper_id(&self) -> u8 {
        // The lower nybble of the mapper number is in flag 6: bytes [4-7]
        // The upper nybble of the mapper number is in flag 7: bytes [4-7]
        (self.flag_7 & 0b1111_0000) | (self.flag_6 >> 4)
    }
}

#[derive(Debug)]
pub enum HeaderParseError {
    InvalidStreamLength,
    NoNesConstant,
}

impl TryFrom<&[u8; 16]> for Header {
    type Error = HeaderParseError;

    /// # Parse the header of the iNes File
    ///
    /// ## Format
    ///
    /// 16 bytes, in the following order:
    ///
    /// | bytes:  | Description                                                                                 |
    /// |---------|---------------------------------------------------------------------------------------------|
    /// | 0-3:    | Constant $4E $45 $53 $1A ("NES" followed by MS-DOS end-of-file)                             |
    /// | 4:      | Size of PRG ROM in 16 KB units                                                              |
    /// | 5:      | Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)                        |
    /// | 6:      | Flags 6 - Mapper, mirroring, battery, trainer                                               |
    /// | 7:      | Flags 7 - Mapper, VS/Playchoice, NES 2.0                                                    |
    /// | 8:      | Flags 8 - PRG-RAM size (rarely used extension)                                              |
    /// | 9:      | Flags 9 - TV system (rarely used extension)                                                 |
    /// | 10:     | Flags 10 - TV system, PRG-RAM presence (unofficial, rarely used extension)                  |
    /// | 11-15:  | Unused padding (should be filled with zero-but some rippers put their name on bytes 7-15)   |
    fn try_from(bytestream: &[u8; 16]) -> Result<Self, Self::Error> {
        if bytestream.len() != 16 {
            return Err(HeaderParseError::InvalidStreamLength);
        }

        if bytestream[0..4] != [b'N', b'E', b'S', 0x1A] {
            return Err(HeaderParseError::NoNesConstant);
        }


        debug!("Header Bytes: ");
        for byte in bytestream.iter() {
            debug!("\t{}\t= 0x{:x}", byte, byte);
        }

        Ok(Self {
            name: [bytestream[0], bytestream[1], bytestream[2], bytestream[3]],
            prg_rom_size: bytestream[4],
            prg_chr_size: bytestream[5],
            flag_6: bytestream[6],
            flag_7: bytestream[7],
            prg_ram_size: bytestream[8],
            tv_system_1: bytestream[9],
            tv_system_2: bytestream[10],
        })
    }
}

impl TryFrom<&[u8]> for Header {
    type Error = HeaderParseError;

    fn try_from(bytestream: &[u8]) -> Result<Self, Self::Error> {
        if bytestream.len() != 16 {
            return Err(HeaderParseError::InvalidStreamLength);
        }
        let header_bytestream: &[u8; 16] = bytestream.try_into().expect("Checked Length");
        Self::try_from(header_bytestream)
    }
}

impl TryFrom<&str> for Header {
    type Error = HeaderParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Header::try_from(s.as_bytes())
    }
}

impl TryFrom<String> for Header {
    type Error = HeaderParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Header::try_from(value.as_str())
    }
}
