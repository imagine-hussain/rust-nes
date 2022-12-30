use std::{vec::Vec, fs::File, num::FpCategory};


#[derive(Debug)]
pub struct Cartridge {
    // TODO: Fields
    virtual_program_memory: Vec<u8>,
    virtual_character_memory: Vec<u8>,
    mapper_id: u8,
    program_banks_count: u8,
    character_banks_count: u8,
}

impl Cartridge {
    pub fn new(path: &str) -> Self {
        Cartridge {
            virtual_program_memory: Vec::new(),
            virtual_character_memory: Vec::new(),
            mapper_id: 0,
            program_banks_count: 0,
            character_banks_count: 0,
        }
    }

    pub fn ppu_read() {}
    pub fn ppu_write() {}
    pub fn cpu_read() {}
    pub fn cpu_write() {}
}

