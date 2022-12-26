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

}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}
