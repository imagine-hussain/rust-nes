/// # Header of the iNES file format
///
/// ## Format
/// [Wiki Source](https://www.nesdev.org/wiki/INES#iNES_file_format>)
///
///
pub struct Header {
    pub name: [u8; 4],
    pub prg_rom_size: u8,
    pub prg_chr_size: u8,
    pub mapper_1: u8,
    pub mapper_2: u8,
    pub prg_ram_size: u8,
    pub tv_system_1: u8,
    pub tv_system_2: u8,
}

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

        Self {
            name: [bytestream[0], bytestream[1], bytestream[2], bytestream[3]],
            prg_rom_size: bytestream[4],
            prg_chr_size: bytestream[5],
            mapper_1: bytestream[6],
            mapper_2: bytestream[7],
            prg_ram_size: bytestream[8],
            tv_system_1: bytestream[9],
            tv_system_2: bytestream[10],
        };

    }
}

impl TryFrom<&str> for Header {
    type Error = HeaderParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() < 16 {
            return Err(HeaderParseError::InvalidStreamLength);
        }

        // let mut header_bytestream: [u8; 16] = s.as_bytes().try_into().unwrap();
        let header_bytestream: &[u8; 16] = &s.as_bytes()[0..16]
            .try_into()
            .expect("Checked stream length > 16");

        Header::try_from(header_bytestream)
    }
}

impl TryFrom<String> for Header {
    type Error = HeaderParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Header::try_from(value.as_str())
    }
}
