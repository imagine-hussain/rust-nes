use lib::{Cpu, Bus, RcCell};



fn main() {
    let cpu = Cpu::new();
    let bus = cpu.borrow().get_bus();

    let nes = Nes::new(cpu, bus);
}

struct Nes {
    cpu: RcCell<Cpu>,
    bus: RcCell<Bus>,
}

impl Nes {
    fn new(cpu: RcCell<Cpu>, bus: RcCell<Bus>) -> Self {
        Self {
            cpu,
            bus
        }
    }
}


