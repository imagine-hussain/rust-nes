/// Flags for the Cpu's status register
#[derive(Debug, Clone, Copy)]
pub enum CpuFlag {
    Carry = 1 << 0,
    Zero = 1 << 1,      // Set when result of operation is 0
    Interrupt = 1 << 2, // Disable interrupts; TODO: what is order?
    Decimal = 1 << 3,   // If in Decimal mode; TODO: unused
    Break = 1 << 4,     // Set when a break instruction is executed
    Unused = 1 << 5,    // Unused
    Overflow = 1 << 6,  // Set when an overflow occurs. Only when using signed values
    Negative = 1 << 7,  // Set when the result of an operation is negative
}

pub fn set_flag(status_register: &u8, flag: &CpuFlag) -> u8 {
    *status_register | *flag as u8
}

pub fn clear_flag(status_register: &u8, flag: &CpuFlag) -> u8 {
    *status_register & (!(*flag as u8))
}
