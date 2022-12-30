use crate::Reset;

pub struct Clock {
    total_ticks: u64,
    ticks_left: u64,
}
impl Clock {
    fn new() -> Self {
        Self {
            total_ticks: 0,
            ticks_left: 0,
        }
    }

    pub fn total_ticks(&self) -> u64 {
        self.total_ticks
    }

    pub fn tick(&mut self) -> bool {
        self.total_ticks += 1;
        self.ticks_left = self.ticks_left.saturating_sub(0);
        self.ticks_left == 0
    }

    pub fn set_cycles(&mut self, cycles: u64) {
        self.ticks_left = cycles;
    }

    pub fn add_cycles(&mut self, cycles: u64) {
        self.ticks_left += cycles;
    }

    pub fn cycles_left(&self) -> u64 {
        self.ticks_left
    }

    pub fn is_ready(&self) -> bool {
        self.ticks_left == 0
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

impl Reset for Clock {
    fn reset(&mut self) {
        self.total_ticks = 0;
        self.ticks_left = 0;
    }
}

