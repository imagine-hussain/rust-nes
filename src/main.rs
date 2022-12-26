use lib::{Bus, Cpu, RcCell};

fn main() {
    let _nes = Nes::default();
}

pub struct Nes {
    pub cpu: RcCell<Cpu>,
    pub bus: RcCell<Bus>,
}

impl Nes {
    fn new(cpu: RcCell<Cpu>, bus: RcCell<Bus>) -> Self {
        Self { cpu, bus }
    }
}

impl Default for Nes {
    fn default() -> Self {
        let cpu = Cpu::new();
        let bus = cpu.borrow().get_bus();

        Self::new(cpu, bus)
    }
}
