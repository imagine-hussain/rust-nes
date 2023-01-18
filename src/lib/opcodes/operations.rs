/// Functions for each instruction
///
/// Documentation from:
/// https://www.nesdev.org/obelisk-6502-guide/reference.html
use crate::Cpu;
use crate::cpu::CpuFlag;
use crate::cpu::AddressingMode;

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
pub fn adc_fn(cpu: &mut Cpu) -> u8 {
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
pub fn sbc_fn(cpu: &mut Cpu) -> u8 {
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
pub fn and_fn(cpu: &mut Cpu) -> u8 {
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
pub fn asl_fn(cpu: &mut Cpu) -> u8 {
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
pub fn relative_branch(cpu: &mut Cpu) -> u8 {
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
pub fn bcc_fn(cpu: &mut Cpu) -> u8 {
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
pub fn bcs_fn(cpu: &mut Cpu) -> u8 {
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
pub fn beq_fn(cpu: &mut Cpu) -> u8 {
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
pub fn bne_fn(cpu: &mut Cpu) -> u8 {
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
pub fn bmi_fn(cpu: &mut Cpu) -> u8 {
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
pub fn bpl_fn(cpu: &mut Cpu) -> u8 {
    match cpu.get_flag(&CpuFlag::Negative) {
        true => 0,
        false => relative_branch(cpu),
    }
}

/// # BRK - Force Interrupt
/// The BRK instruction forces the generation of an interrupt request. The program counter and
/// processor status are pushed on the stack then the IRQ interrupt vector at $FFFE/F is loaded
/// into the PC and the break flag in the status set to one.
pub fn brk_fn(cpu: &mut Cpu) -> u8 {
    // shouldn't ever need to wrap on a real program but, for completeness,
    // have it overflow
    cpu.program_counter = cpu.program_counter.wrapping_add(1);
    cpu.set_flag(&CpuFlag::Interrupt);

    // Push PC to stack (2 bytes)
    let hi = (cpu.program_counter >> 8) as u8;
    let lo = cpu.program_counter as u8;
    cpu.push_stack(hi);
    cpu.push_stack(lo);

    // Push status register but, with the break flag set
    cpu.set_flag(&CpuFlag::Break);
    cpu.push_stack(cpu.status_register);
    cpu.clear_flag(&CpuFlag::Break);

    // Load PC from interrupt vector (Last 2 bytes of address space)



    0
}

/// # Branch if Overflow Clear
/// If the overflow flag is clear, then add the relative displacement to the program
/// counter to cause a branch to a new location.
/// ## Processor Status after use:
/// No Changes
/// ## Cycles:
/// +1 if branch occurs
/// +2 if branch occurs to a new page
pub fn bvc_fn(cpu: &mut Cpu) -> u8 {
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
pub fn bvs_fn(cpu: &mut Cpu) -> u8 {
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
pub fn bit_fn(cpu: &mut Cpu) -> u8 {
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
pub fn clc_fn(cpu: &mut Cpu) -> u8 {
    cpu.clear_flag(&CpuFlag::Carry);
    0
}

/// # Clear Decimal Flag
/// Set the decimal flag to 0
/// ## Processor Status after use:
/// - D - Decimal Flag      - Set to 0
pub fn cld_fn(cpu: &mut Cpu) -> u8 {
    cpu.clear_flag(&CpuFlag::Decimal);
    0
}

/// # Clear Interrupt Disable
/// Set the interrupt disable flag to 0
/// ## Processor Status after use:
/// - I - Interrupt Disable - Set to 0
pub fn cli_fn(cpu: &mut Cpu) -> u8 {
    cpu.clear_flag(&CpuFlag::Interrupt);
    0
}

/// # Clear Overflow Flag
/// Set the overflow flag to 0
/// ## Processor Status after use:
/// - V - Overflow Flag     - Set to 0
pub fn clv_fn(cpu: &mut Cpu) -> u8 {
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
pub fn cmp_fn(cpu: &mut Cpu) -> u8 {
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
pub fn cpx_fn(cpu: &mut Cpu) -> u8 {
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
pub fn cpy_fn(cpu: &mut Cpu) -> u8 {
    compare_values(cpu, cpu.y_register);
    0
}

/// Helper for compare instructions
/// ## Processor Status after use:
/// - C - Carry Flag        - Set if Register >= M
/// - Z - Zero Flag         - Set if Register == M
/// - N - Negative Flag     - Set if bit 7 of Register - M is set
pub fn compare_values(cpu: &mut Cpu, register_val: u8) {
    let rhs = cpu.fetch();
    // let result = ((register_val as i16 - rhs as u16) & 0x00FF) as u8;
    cpu.set_or_clear_flag(&CpuFlag::Carry, register_val >= rhs);
    cpu.set_or_clear_flag(&CpuFlag::Carry, register_val == rhs);
    // TODO: fix this to prevent underflow
    // cpu.set_or_clear_flag(&CpuFlag::Negative, result & 0x80 != 0);
}

/// # Decrement Memory
/// Decrement the value at a memory location by 1.
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if result is zero
/// - N - Negative Flag     - Set if bit 7 of result is set
pub fn dec_fn(cpu: &mut Cpu) -> u8 {
    let res = cpu.fetch().wrapping_sub(1);
    cpu.set_or_clear_flag(&CpuFlag::Zero, res == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, res & 0x80 != 0);

    0
}

/// # Decrement X Register
/// Subtracts one from the X register setting the zero and negative flags as appropriate.
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if X == 0
/// - N - Negative Flag     - Set if bit 7 of X is set
pub fn dex_fn(cpu: &mut Cpu) -> u8 {
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
pub fn dey_fn(cpu: &mut Cpu) -> u8 {
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
pub fn eor_fn(cpu: &mut Cpu) -> u8 {
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
pub fn inc_fn(cpu: &mut Cpu) -> u8 {
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
pub fn inx_fn(cpu: &mut Cpu) -> u8 {
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
pub fn iny_fn(cpu: &mut Cpu) -> u8 {
    cpu.y_register = cpu.x_register.wrapping_add(1);
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.y_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.y_register & 0x80 != 0);
    0
}

/// # Jump to Address
/// Sets the program counter to the address specified.
pub fn jmp_fn(cpu: &mut Cpu) -> u8 {
    cpu.program_counter = cpu.absolute_addr;
    0
}

/// # Jump to Subroutine
/// Pushes the program_counter to the stack and then sets the program_counter
/// to the address specified.
pub fn jsr_fn(cpu: &mut Cpu) -> u8 {
    cpu.program_counter -= 1;

    // Push Hi and Lo Seperately
    cpu.push_stack((cpu.program_counter >> 8 & 0x00FF) as u8);
    cpu.push_stack((cpu.program_counter & 0x00FF) as u8);

    cpu.program_counter = cpu.absolute_addr;

    0
}

/// # Load Accumulator
/// Loads a byte into the accumulator from Memory
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if result is zero
/// - N - Negative Flag     - Set if bit 7 of result is set
pub fn lda_fn(cpu: &mut Cpu) -> u8 {
    cpu.a_register = cpu.fetch();
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.a_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.a_register & 0x80 != 0);

    1
}

/// # Load X Register
/// Loads a byte into the X register from Memory
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if result is zero
/// - N - Negative Flag     - Set if bit 7 of result is set
pub fn ldx_fn(cpu: &mut Cpu) -> u8 {
    cpu.x_register = cpu.fetch();
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.x_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.x_register & 0x80 != 0);

    1
}

/// # Load Y Register
/// Loads a byte into the Y register from Memory
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if result is zero
/// - N - Negative Flag     - Set if bit 7 of result is set
pub fn ldy_fn(cpu: &mut Cpu) -> u8 {
    cpu.y_register = cpu.fetch();
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.y_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.y_register & 0x80 != 0);

    1
}

/// # Logical Shift Right
/// Shift the bits in A or M (depending on addressing mode) to the right
/// by one bit.
/// ## Processor Status after use:
/// - C - Carry Flag        - Set to the value of bit 0 before the shift
/// - Z - Zero Flag         - Set if result is zero
/// - N - Negative Flag     - Set if bit 7 of result is set
pub fn lsr_fn(cpu: &mut Cpu) -> u8 {
    let fetched = cpu.fetch();
    cpu.set_or_clear_flag(&CpuFlag::Carry, fetched & 0x01 != 0);

    let res = fetched >> 1;
    cpu.set_or_clear_flag(&CpuFlag::Zero, res == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, res & 0x80 != 0);

    match cpu.addressing_mode {
        AddressingMode::IMP => cpu.a_register = res,
        _ => cpu.write(cpu.absolute_addr, res),
    }

    0
}

/// # No Operation
/// No Operation. Do nothing
pub fn nop_fn(_cpu: &mut Cpu) -> u8 {
    // TODO: verify different no-op opcodes have different cycles
    // https://wiki.nesdev.com/w/index.php/CPU_unofficial_opcodes
    // The use of unofficial opcodes is rare in NES games. It appears to occur mostly in late or
    // unlicensed titles:
    // - Beauty and the Beast (E) (1994) uses $80 (a 2-byte NOP).[2]
    // - Disney's Aladdin (E) (December 1994) uses $07 (SLO). This is Virgin's port of the Game Boy
    // game, itself a port of the Genesis game, not any of the pirate originals.
    // - Dynowarz: Destruction of Spondylus (April 1990) uses 1-byte NOPs $DA and $FA on the first
    // level when your dino throws his fist.
    // - F-117A Stealth Fighter uses $89 (a 2-byte NOP).
    // - Gaau Hok Gwong Cheung (Ch) uses $8B (XAA immediate) as a 2-byte NOP. The game malfunctions
    // after selecting Left from the main menu if that instruction is not emulated.
    // - Infiltrator uses $89 (a 2-byte NOP).
    // - Puzznic (all regions) (US release November 1990) uses $89 (a 2-byte NOP).
    // - Super Cars (U) (February 1991) uses $B3 (LAX).
    // ----------------------------------------------
    0
}

/// # Bitwise Or
/// Performs a bitwise OR on the accumulator and the value in memory; A | M
/// Stores result back in accumulator
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if result is zero
/// - N - Negative Flag     - Set if bit 7 of result is set
pub fn ora_fn(cpu: &mut Cpu) -> u8 {
    cpu.a_register |= cpu.fetch();
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.a_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.a_register & 0x80 != 0);

    1
}

/// # Push Accumulator
/// Pushes the accumulator onto the stack
pub fn pha_fn(cpu: &mut Cpu) -> u8 {
    cpu.push_stack(cpu.a_register);
    0
}

/// # Push Status Register
/// Pushes the status register onto the stack
/// ## Processor Status after use:
/// - B - Break Flag        - Set to 1 before pushing. Unset after
/// - U - Unused Flag       - Set to 1 before pushing. Unset after TODO: verify
pub fn php_fn(cpu: &mut Cpu) -> u8 {
    cpu.push_stack(cpu.status_register | CpuFlag::Break as u8 | CpuFlag::Unused as u8);
    cpu.clear_flag(&CpuFlag::Break);
    cpu.clear_flag(&CpuFlag::Unused);
    0
}

/// # Pull Accumulator
/// Pulls top value from the stack, into the accumulator
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if result is zero
/// - N - Negative Flag     - Set if bit 7 of result is set
pub fn pla_fn(cpu: &mut Cpu) -> u8 {
    cpu.a_register = cpu.pop_stack();
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.a_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.a_register & 0x80 != 0);
    0
}


/// # Pull Status Register
/// Pulls top value from the stack, into the status register
/// ## Processor Status after use:
/// - U - Unused Flag       - Set to 1 after pulling. TODO: verify
pub fn plp_fn(cpu: &mut Cpu) -> u8 {
    cpu.a_register = cpu.pop_stack();
    cpu.set_flag(&CpuFlag::Unused);
    0
}

/// # Rotate Left
/// Shift the bits in A or M (depending on addressing mode) to the left by one
/// place. Result stored back in A or M
/// Bit 0 is filled with the current value of the carry flag whilst the old bit 7
/// becomes the new carry flag value.
/// ## Processor Status after use:
/// - C - Carry Flag        - Set to the value of bit 7 before the shift
/// - Z - Zero Flag         - Set if result is zero
/// - N - Negative Flag     - Set if bit 7 of result is set
pub fn rol_fn(cpu: &mut Cpu) -> u8 {
    let res: u16 = (cpu.fetch() as u16) << 1 | cpu.get_flag(&CpuFlag::Carry) as u16;

    cpu.set_or_clear_flag(&CpuFlag::Zero, res == 0);
    cpu.set_or_clear_flag(&CpuFlag::Carry, res & 0xFF00 != 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, res & 0x80 != 0);

    match cpu.addressing_mode {
        AddressingMode::IMP => cpu.a_register = res as u8,
        _ => cpu.write(cpu.absolute_addr, (res & 0x00FF) as u8),
    };

    0
}

/// # Rotate Right
/// Shift the bits in A or M (depending on addressing mode) to the right by one
/// place. Result Stored back in A or M
/// ## Processor Status after use:
/// - C - Carry Flag        - Set to the value of bit 0 before the shift
/// - Z - Zero Flag         - Set if result is zero
/// - N - Negative Flag     - Set if bit 7 of result is set
pub fn ror_fn(cpu: &mut Cpu) -> u8 {
    let fetched = cpu.fetch();
    cpu.set_or_clear_flag(&CpuFlag::Carry, fetched & 0x01 != 0);

    let res = (fetched >> 1) | (cpu.get_flag(&CpuFlag::Carry) as u8) << 7;
    cpu.set_or_clear_flag(&CpuFlag::Zero, res == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, res & 0x80 != 0);

    match cpu.addressing_mode {
        AddressingMode::IMP => cpu.a_register = res,
        _ => cpu.write(cpu.absolute_addr, res),
    };

    0
}

/// # Return from Interrupt
/// Used at the end of a interrupt routine to return to the main program.
/// Pulls status and program counter from the stac
/// ## Processor Status after use:
/// - C - Carry Flag        - Set from stack
/// - Z - Zero Flag         - Set from stack
/// - I - Interrupt Flag    - Set from stack
/// - D - Decimal Flag      - Set from stack
/// - B - Break Flag        - Set from stack
/// - V - Overflow Flag     - Set from stack
/// - N - Negative Flag     - Set from stack
pub fn rti_fn(cpu: &mut Cpu) -> u8 {

    cpu.status_register = cpu.pop_stack();

    // Unset Break and Unused since out of interrupt
    cpu.clear_flag(&CpuFlag::Break);
    cpu.clear_flag(&CpuFlag::Unused);

    let lo = cpu.pop_stack() as u16;
    let hi = cpu.pop_stack() as u16;
    cpu.program_counter = (hi << 8) | lo;

    0
}

/// # Return from Subroutine
/// Used at the end of a subroutine to return to the calling routine.
/// Pulls (program counter - 1) from the stack.
pub fn rts_fn(cpu: &mut Cpu) -> u8 {
    let lo = cpu.pop_stack() as u16;
    let hi = cpu.pop_stack() as u16;
    cpu.program_counter = (hi << 8 | lo) + 1;
    0
}

/// # Set Carry Flag
/// Sets the carry flag to 1
/// ## Processor Status after use:
/// - C - Carry Flag        - Set to 1
pub fn sec_fn(cpu: &mut Cpu) -> u8 {
    cpu.set_flag(&CpuFlag::Carry);
    0
}

/// # Set Decimal Flag
/// Sets the decimal flag to 1
/// ## Processor Status after use:
/// - D - Decimal Flag      - Set to 1
pub fn sed_fn(cpu: &mut Cpu) -> u8 {
    cpu.set_flag(&CpuFlag::Decimal);
    0
}

/// # Set Interrupt Disable
/// Sets the interrupt disable flag to 1
/// ## Processor Status after use:
/// - I - Interrupt Flag    - Set to 1
pub fn sei_fn(cpu: &mut Cpu) -> u8 {
    cpu.set_flag(&CpuFlag::Interrupt);
    0
}

/// # Store Accumulator
/// Stores the contents of the accumulator into memory
pub fn sta_fn(cpu: &mut Cpu) -> u8 {
    cpu.write(cpu.absolute_addr, cpu.a_register);
    0
}


/// # Store X Register
/// Stores the contents of X into memory
pub fn stx_fn(cpu: &mut Cpu) -> u8 {
    cpu.write(cpu.absolute_addr, cpu.x_register);
    0
}

/// # Store Y Register
/// Stores the contents of Y into memory
pub fn sty_fn(cpu: &mut Cpu) -> u8 {
    cpu.write(cpu.absolute_addr, cpu.y_register);
    0
}

/// # Transfer Accumulator to X
/// Copies the contents of the accumulator into the X register
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if X is zero
/// - N - Negative Flag     - Set if bit 7 of X is set
pub fn tax_fn(cpu: &mut Cpu) -> u8 {
    cpu.x_register = cpu.a_register;
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.x_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.x_register & 0x80 != 0);
    0
}

/// # Transfer Accumulator to Y
/// Copies the contents of the accumulator into the Y register
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if Y is zero
/// - N - Negative Flag     - Set if bit 7 of Y is set
pub fn tay_fn(cpu: &mut Cpu) -> u8 {
    cpu.y_register = cpu.a_register;
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.y_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.y_register & 0x80 != 0);
    0
}

/// # Transfer Stack Pointer to X
/// Copies the contents of the stack pointer into the X register
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if X is zero
/// - N - Negative Flag     - Set if bit 7 of X is set
pub fn tsx_fn(cpu: &mut Cpu) -> u8 {
    cpu.x_register = cpu.stack_pointer;
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.x_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.x_register & 0x80 != 0);
    0
}

/// # Transfer X to Stack Pointer
/// Copies the contents of the X register into the stack pointer
pub fn txs_fn(cpu: &mut Cpu) -> u8 {
    cpu.stack_pointer = cpu.x_register;
    0
}

/// # Transfer X to Accumulator
/// Copies the contents of the X register into the accumulator
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if A is zero
/// - N - Negative Flag     - Set if bit 7 of A is set
pub fn txa_fn(cpu: &mut Cpu) -> u8 {
    cpu.a_register = cpu.x_register;
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.a_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.a_register & 0x80 != 0);
    0
}

/// # Transfer Y to Accumulator
/// Copies the contents of the Y register into the accumulator
/// ## Processor Status after use:
/// - Z - Zero Flag         - Set if A is zero
/// - N - Negative Flag     - Set if bit 7 of A is set
pub fn tya_fn(cpu: &mut Cpu) -> u8 {
    cpu.a_register = cpu.y_register;
    cpu.set_or_clear_flag(&CpuFlag::Zero, cpu.a_register == 0);
    cpu.set_or_clear_flag(&CpuFlag::Negative, cpu.a_register & 0x80 != 0);
    0
}


/// # Unofficial / Illegal Instructions
/// Undefined. Should do nothing until implemented later.
/// Currently, acts equivalently to NOP
pub fn xxx_fn(_cpu: &mut Cpu) -> u8 {
    0
}

