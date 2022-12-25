use lib::{Bus, Cpu, RcCell};

fn main() {
    let nes = Nes::default();
}

struct Nes {
    cpu: RcCell<Cpu>,
    bus: RcCell<Bus>,
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

        Self { cpu, bus }
    }
}

