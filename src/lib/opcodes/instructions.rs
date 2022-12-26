use std::cmp::Ordering;

/// Functions for each instruction
///
/// Documentation from:
/// https://www.nesdev.org/obelisk-6502-guide/reference.html
use crate::{cpu::CpuFlag, AddressingMode, Cpu};

/// # Add with carry
///
/// This instruction adds the contents of a memory location to the accumulator together with the
/// carry bit. If overflow occurs the carry bit is set, this enables multiple byte addition to be
/// performed.
/// ## Status Register Flags
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
    cpu.set_or_clear_flag(
        &CpuFlag::Overflow,
        {
            !(cpu.a_register as u16 ^ fetched as u16) & ((cpu.a_register as u16 ^ raw_add) * 0x0080)
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
    let fetched = cpu.fetch() ^ 0xFF;

    // add as u16 for overflow detection
    let raw_add = cpu.a_register as u16 + fetched as u16 + cpu.get_flag(&CpuFlag::Carry) as u16;
    let final_add = (raw_add & 0xFF) as u8;

    //

    cpu.set_or_clear_flag(&CpuFlag::Carry, raw_add > u8::MAX as u16);
    cpu.set_or_clear_flag(&CpuFlag::Zero, final_add == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, (final_add & 0x80) != 0);
    // TODO: double check this logic
    cpu.set_or_clear_flag(
        &CpuFlag::Overflow,
        {
            !(cpu.a_register as u16 ^ fetched as u16) & ((cpu.a_register as u16 ^ raw_add) * 0x0080)
        } == 0,
    );

    cpu.a_register = final_add;

    1 // Can require extra cycle
}

/// # Logical AND
/// A logical AND is performed, bit by bit, on the accumulator contents using the contents of a
/// byte of memory.
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if A = 0
/// - N - Negative Flag     - Set if bit 7 set
fn and_fn(cpu: &mut Cpu) -> u8 {
    let fetched = cpu.fetch();

    let res = cpu.a_register & fetched;
    cpu.set_or_clear_flag(&CpuFlag::Zero, res == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, (res & 0x80) != 0);

    1
}

/// # Arithmetic Shift Left
/// Shift bits of accumulator left by one position. Equivalent to acc << 1
/// or, a multiplication by 2.
/// ## Processor Status after use:
/// - C - Carry Flag        - Multiplying result overflows 8bit register
/// - Z - Zero Flag         - Set if A = 0
/// - N - Negative Flag     - Set if bit 7 of result set
fn asl_fn(cpu: &mut Cpu) -> u8 {
    let raw_res: u16 = (cpu.fetch() as u16) << 0x01;
    let result: u8 = (raw_res & 0xFF) as u8;

    cpu.set_or_clear_flag(&CpuFlag::Carry, raw_res > u8::MAX as u16);
    cpu.set_or_clear_flag(&CpuFlag::Zero, result == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, (result & 0x80) != 0);

    // Store in either memory or accumulator
    match cpu.addressing_mode {
        AddressingMode::IMP => cpu.a_register = result,
        _ => cpu.write(cpu.absolute_addr, result),
    };

    0
}

/// Helper for branching.
/// Returns 1 extra cycle if branch occurs to the same page.
/// 2 Extra cycles if branch is onto a different bage
fn relative_branch(cpu: &mut Cpu) -> u8 {
    let pc_old = cpu.program_counter;
    cpu.program_counter = (cpu.program_counter as u16) + (cpu.relative_addr as i16) as u16;

    match (pc_old & 0xFF00) == (cpu.program_counter & 0xFF00) {
        true => 1,  // Same page -> 1
        false => 2, // New page -> 2
    }
}

/// # Branch if Carry Clear
/// If carry flag is clear, then add relative displacement to the program
/// counter, causing a branch.
///
/// ## Processor Status after use:
/// No Changes
///
/// ## Cycles:
/// +1 if branch occurs
/// +2 if branch occurs to a new page
fn bcc_fn(cpu: &mut Cpu) -> u8 {
    match cpu.get_flag(&CpuFlag::Carry) {
        true => 0,
        false => relative_branch(cpu),
    }
}

/// # Branch if Carry Set
/// If carry flag is set, then add relative displacement to the program
/// counter, causing a branch.
///
/// ## Processor Status after use:
/// No Changes
///
/// ## Cycles:
/// +1 if branch occurs
/// +2 if branch occurs to a new page
fn bcs_fn(cpu: &mut Cpu) -> u8 {
    match cpu.get_flag(&CpuFlag::Carry) {
        true => relative_branch(cpu),
        false => 0,
    }
}

/// # Branch if Equal
/// If the zero flag is set, then add the relative displacement to the program
/// counter to cause a branch to a new location.
/// ## Processor Status after use:
/// No Changes
/// ## Cycles:
/// +1 if branch occurs
/// +2 if branch occurs to a new page
fn beq_fn(cpu: &mut Cpu) -> u8 {
    match cpu.get_flag(&CpuFlag::Zero) {
        true => relative_branch(cpu),
        false => 0,
    }
}

/// # Branch if Not Equal
/// If the zero flag is clear, then add the relative displacement to the program
/// counter to cause a branch to a new location.
/// ## Processor Status after use:
/// No Changes
/// ## Cycles:
/// +1 if branch occurs
/// +2 if branch occurs to a new page
fn bne_fn(cpu: &mut Cpu) -> u8 {
    match cpu.get_flag(&CpuFlag::Zero) {
        true => 0,
        false => relative_branch(cpu),
    }
}

/// # Branch if Minus
/// If the negative flag is set, then add the relative displacement to the program
/// counter to cause a branch to a new location.
/// ## Processor Status after use:
/// No Changes
/// ## Cycles:
/// +1 if branch occurs
/// +2 if branch occurs to a new page
fn bmi_fn(cpu: &mut Cpu) -> u8 {
    match cpu.get_flag(&CpuFlag::Negative) {
        true => relative_branch(cpu),
        false => 0,
    }
}

/// # Branch if Positive
/// If the negative flag is clear, then add the relative displacement to the program
/// counter to cause a branch to a new location.
/// ## Processor Status after use:
/// No Changes
/// ## Cycles:
/// +1 if branch occurs
/// +2 if branch occurs to a new page
fn bpl_fn(cpu: &mut Cpu) -> u8 {
    match cpu.get_flag(&CpuFlag::Negative) {
        true => 0,
        false => relative_branch(cpu),
    }
}

/// # Branch if Overflow Clear
/// If the overflow flag is clear, then add the relative displacement to the program
/// counter to cause a branch to a new location.
/// ## Processor Status after use:
/// No Changes
/// ## Cycles:
/// +1 if branch occurs
/// +2 if branch occurs to a new page
fn bvc_fn(cpu: &mut Cpu) -> u8 {
    match cpu.get_flag(&CpuFlag::Overflow) {
        true => 0,
        false => relative_branch(cpu),
    }
}

/// # Branch if Overflow Set
/// If the overflow flag is set, then add the relative displacement to the program
/// counter to cause a branch to a new location.
/// ## Processor Status after use:
/// No Changes
/// ## Cycles:
/// +1 if branch occurs
/// +2 if branch occurs to a new page
fn bvs_fn(cpu: &mut Cpu) -> u8 {
    match cpu.get_flag(&CpuFlag::Overflow) {
        false => 0,
        true => relative_branch(cpu),
    }
}

/// # Bit Test
/// Perform a bitwise And between the accumulator and the value in memory.
/// Tests if any of the bits are set (the result of the BitAnd is non-zero).
/// The result is not stored back in memory or, the accumulator.
/// Instead, is used to set processor status flags.
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if result is zero
/// - N - Negative Flag     - Set if bit 7 of result is set
/// - V - Overflow Flag     - Set if bit 6 of result is set
fn bit_fn(cpu: &mut Cpu) -> u8 {
    let fetched = cpu.fetch();
    let result = cpu.a_register & fetched;

    cpu.set_or_clear_flag(&CpuFlag::Zero, result == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, (fetched & (1 << 7)) != 0);
    cpu.set_or_clear_flag(&CpuFlag::Overflow, (fetched & (1 << 6)) != 0);

    0
}

/// # Clear Carry Flag
/// Set the carry flag to 0
/// ## Processor Status after use:
/// - C - Carry Flag        - Set to 0
fn clc_fn(cpu: &mut Cpu) -> u8 {
    cpu.clear_flag(&CpuFlag::Carry);
    0
}

/// # Clear Decimal Flag
/// Set the decimal flag to 0
/// ## Processor Status after use:
/// - D - Decimal Flag      - Set to 0
fn cld_fn(cpu: &mut Cpu) -> u8 {
    cpu.clear_flag(&CpuFlag::Decimal);
    0
}

/// # Clear Interrupt Disable
/// Set the interrupt disable flag to 0
/// ## Processor Status after use:
/// - I - Interrupt Disable - Set to 0
fn cli_fn(cpu: &mut Cpu) -> u8 {
    cpu.clear_flag(&CpuFlag::Interrupt);
    0
}

/// # Clear Overflow Flag
/// Set the overflow flag to 0
/// ## Processor Status after use:
/// - V - Overflow Flag     - Set to 0
fn clv_fn(cpu: &mut Cpu) -> u8 {
    cpu.clear_flag(&CpuFlag::Overflow);
    0
}


/// # Compare Accumultor
/// Compares the contents of the accumulator with another value in memory (M).
/// Result is returned by setting zero and carry flags.
/// ## Processor Status after use:
/// - C - Carry Flag        - Set if A >= M
/// - Z - Zero Flag         - Set if A == M
/// - N - Negative Flag     - Set if bit 7 of A - M is set
/// ## Cycles:
/// +1 if page crosses in certain addressing modes
fn cmp_fn(cpu: &mut Cpu) -> u8 {
    compare_values(cpu, cpu.a_register);
    1
}

/// # Compare X Register
/// Compares the contents of the X register with another value in memory (M).
/// Result is returned by setting zero and carry flags.
/// ## Processor Status after use:
/// - C - Carry Flag        - Set if X >= M
/// - Z - Zero Flag         - Set if X == M
/// - N - Negative Flag     - Set if bit 7 of X - M is set
fn cpx_fn(cpu: &mut Cpu) -> u8 {
    compare_values(cpu, cpu.x_register);
    0
}


/// # Compare Y Register
/// Compares the contents of the Y register with another value in memory (M).
/// Result is returned by setting zero and carry flags.
/// ## Processor Status after use:
/// - C - Carry Flag        - Set if Y >= M
/// - Z - Zero Flag         - Set if Y == M
/// - N - Negative Flag     - Set if bit 7 of Y - M is set
fn cpy_fn(cpu: &mut Cpu) -> u8 {
    compare_values(cpu, cpu.y_register);
    0
}

/// Helper for compare instructions
/// ## Processor Status after use:
/// - C - Carry Flag        - Set if Register >= M
/// - Z - Zero Flag         - Set if Register == M
/// - N - Negative Flag     - Set if bit 7 of Register - M is set
fn compare_values(cpu: &mut Cpu, register_val: u8) {
    let rhs_val = cpu.fetch();
    let result = ((register_val as u16 - rhs_val as u16) & 0x00FF) as u8;
    cpu.set_or_clear_flag(&CpuFlag::Carry, register_val >= rhs_val);
    cpu.set_or_clear_flag(&CpuFlag::Carry, result == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, result & 0x80 != 0);
}


/// # Decrement X Register
/// Subtracts one from the X register setting the zero and negative flags as appropriate.
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if X == 0
/// - N - Negative Flag     - Set if bit 7 of X is set
fn dex_fn(cpu: &mut Cpu) -> u8 {
    cpu.x_register = cpu.x_register.wrapping_sub(1);
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.x_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.x_register & 0x80 != 0);
    0
}

/// # Decrement y Register
/// Subtracts one from the y register setting the zero and negative flags as appropriate.
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if y == 0
/// - N - Negative Flag     - Set if bit 7 of y is set
fn dey_fn(cpu: &mut Cpu) -> u8 {
    cpu.y_register = cpu.x_register.wrapping_sub(1);
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.y_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.y_register & 0x80 != 0);
    0
}

/// # Exclusive Or (A ^ M)
/// Perform a bitwise exclusive or between the accumulator and the value in memory.
/// The result is stored back in the accumulator.
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if result is zero
/// - N - Negative Flag     - Set if bit 7 of result is set
fn eor_fn(cpu: &mut Cpu) -> u8 {
    cpu.a_register ^= cpu.fetch();
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.a_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.a_register & 0x80 != 0);
    1
}


/// # Increment Memory
/// Add one to the value at the memory address.
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if result is zero
/// - N - Negative Flag     - Set if bit 7 of result is set
fn inc_fn(cpu: &mut Cpu) -> u8 {
    let res = cpu.fetch() + 1;
    cpu.write(cpu.absolute_addr + 1, res);

    cpu.set_or_clear_flag(&CpuFlag::Zero, res == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, res & 0x80 != 0);
    0
}

/// # Increment X Register
/// Adds one to the X register setting the zero and negative flags as appropriate.
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if X == 0
/// - N - Negative Flag     - Set if bit 7 of X is set
fn inx_fn(cpu: &mut Cpu) -> u8 {
    cpu.x_register = cpu.x_register.wrapping_add(1);
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.x_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.x_register & 0x80 != 0);
    0
}

/// # Increment y Register
/// Adds one to the y register setting the zero and negative flags as appropriate.
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if y == 0
/// - N - Negative Flag     - Set if bit 7 of y is set
fn iny_fn(cpu: &mut Cpu) -> u8 {
    cpu.y_register = cpu.x_register.wrapping_add(1);
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.y_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.y_register & 0x80 != 0);
    0
}

/// # Jump to Address
/// Sets the program counter to the address specified.
fn jmp_fn(cpu: &mut Cpu) -> u8 {
    cpu.program_counter = cpu.absolute_addr;
    0
}

/// # Jump to Subroutine
/// Pushes the program_counter to the stack and then sets the program_counter
/// to the address specified.
fn jsr_fn(cpu: &mut Cpu) -> u8 {
    cpu.program_counter -= 1;

    // Push Hi and Lo Seperately
    cpu.push_stack((cpu.program_counter >> 8 & 0x00FF) as u8);
    cpu.push_stack((cpu.program_counter & 0x00FF) as u8);

    cpu.program_counter = cpu.absolute_addr;

    0
}


