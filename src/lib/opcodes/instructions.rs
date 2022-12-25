/// Functions for each instruction
///
/// Documentation from:
/// https://www.nesdev.org/obelisk-6502-guide/reference.html
use crate::Cpu;


/// # Add with carry
///
/// This instruction adds the contents of a memory location to the accumulator together with the
/// carry bit. If overflow occurs the carry bit is set, this enables multiple byte addition to be
/// performed.
fn adc_fn(cpu: &mut Cpu) -> u8 {
    cpu.fetch();

    1
}


