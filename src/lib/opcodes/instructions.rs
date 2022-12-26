/// Functions for each instruction
///
/// Documentation from:
/// https://www.nesdev.org/obelisk-6502-guide/reference.html
use crate::{cpu::CpuFlag, Cpu};

/// # Add with carry
///
/// This instruction adds the contents of a memory location to the accumulator together with the
/// carry bit. If overflow occurs the carry bit is set, this enables multiple byte addition to be
/// performed.
/// # Status Register Flags
/// C    Carry Flag     - Set if overflow in bit 7
/// Z    Zero Flag      - Set if A = 0
/// V    Overflow Flag  - Set if sign bit is incorrect
/// N    Negative Flag  - Set if bit 7 set
fn adc_fn(cpu: &mut Cpu) -> u8 {
    let fetched = cpu.fetch();

    // add as u16 for overflow detection
    let raw_add = cpu.a_register as u16 + fetched as u16 + cpu.get_flag(&CpuFlag::Carry) as u16;
    let final_add = (raw_add & 0xFF) as u8;

    //

    cpu.set_or_clear_flag(&CpuFlag::Carry, raw_add > u8::MAX as u16);
    cpu.set_or_clear_flag(&CpuFlag::Zero, final_add == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, (final_add & 0x80) != 0);
    // TODO: double check this logic
    cpu.set_or_clear_flag(&CpuFlag::Overflow, {
            !(cpu.a_register as u16 ^ fetched as u16)
            & ((cpu.a_register as u16 ^ raw_add) * 0x0080)
        } == 0,
    );

    cpu.a_register = final_add;

    1 // Can require extra cycle
}


/// # Subtract with Carry
/// This instruction subtracts the contents of a memory location to the accumulator together with
/// the not of the carry bit. If overflow occurs the carry bit is clear, this enables multiple byte
/// subtraction to be performed.
/// ## Processor Status after use:
/// - C - Carry Flag        - Clear if overflow in bit 7
/// - Z - Zero Flag         - Set if A = 0
/// - V - Overflow Flag     - Set if sign bit is incorrect
/// - N - Negative Flag     - Set if bit 7 set
fn sbc_fn(cpu: &mut Cpu) -> u8 {
    // Take compliment and treat as addition
    let fetched = cpu.fetch() ^ 0xFF ;

    // add as u16 for overflow detection
    let raw_add = cpu.a_register as u16 + fetched as u16 + cpu.get_flag(&CpuFlag::Carry) as u16;
    let final_add = (raw_add & 0xFF) as u8;

    //

    cpu.set_or_clear_flag(&CpuFlag::Carry, raw_add > u8::MAX as u16);
    cpu.set_or_clear_flag(&CpuFlag::Zero, final_add == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, (final_add & 0x80) != 0);
    // TODO: double check this logic
    cpu.set_or_clear_flag(&CpuFlag::Overflow, {
            !(cpu.a_register as u16 ^ fetched as u16)
            & ((cpu.a_register as u16 ^ raw_add) * 0x0080)
        } == 0,
    );

    cpu.a_register = final_add;

    1 // Can require extra cycle
}


