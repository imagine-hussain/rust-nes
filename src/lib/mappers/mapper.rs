use crate::cartridge::Header;


/// # Mapper
/// Trait to emulate the address mapper.
///
/// https://www.nesdev.org/wiki/Mapper
pub trait Mapper {
    /// Construct a new maper, given the header metadata for the ROM
    /// Most of this information will not be used by most mappers, however
    /// it is there if needed.
    fn new(header: &Header) -> Self;

    // TODO: These should return a Option<u16> instead of a boolean flag

    /// Transform address from cpu to an address indexable in the ROM
    fn map_cpu_read(&mut self, addr: u16, new_addr: &mut u16) -> bool;
    /// Transform a
    fn map_cpu_write(&mut self, addr: u16, new_addr: &mut u16) -> bool;

    fn map_ppu_read(&mut self, addr: u16, new_addr: &mut u16) -> bool;
    fn map_ppu_write(&mut self, addr: u16, new_addr: &mut u16) -> bool;
}

// /// Represents the state of an address after it has been mapped
// /// TODO: use this places
// pub enum MappedAddressStatus {
//     Changed,
//     Unchanged,
// }

