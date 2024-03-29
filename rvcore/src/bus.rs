use std::fmt::Debug;

use crate::DRam;

pub const DRAM_ADDR: usize = 0x0; //0x8000_0000;

pub struct Bus {
    pub dram: DRam,
}

impl Bus {
    pub fn new(dram: DRam) -> Self {
        Self { dram }
    }

    pub fn load(&self, addr: usize, size: u8) -> u32 {
        if addr < DRAM_ADDR + self.dram.size() {
            self.dram.load(addr - DRAM_ADDR, size)
        } else {
            0
        }
    }

    pub fn store(&mut self, addr: usize, size: u8, value: u32) {
        if addr < DRAM_ADDR + self.dram.size() {
            self.dram.store(addr - DRAM_ADDR, size, value);
        }
    }
}

impl Debug for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bus")
            .field("dram", &self.dram.size())
            .finish()
    }
}
