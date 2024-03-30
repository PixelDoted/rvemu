mod instruction;
mod ui;

use rv_f::RV32F;
use rv_m::RV32M;
use std::error::Error;
use ui::UserInterface;

use rv32i::RV32I;
use rvcore::{
    bus::{Bus, DRAM_ADDR},
    Base, DRam, EResult, Extension,
};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // ---- Setup Emulator ----
    let bus = Bus::new(DRam::new(1024 * 1024));
    let mut rv_base = RV32I::new(bus);

    let mut rv_m = rv_m::RV32M;
    let mut rv_f = rv_f::RV32F::default();

    {
        let bus = rv_base.bus();
        bus.store(DRAM_ADDR, 32, 0x00130293u32); // addi x5, x6, 1
        bus.store(4 + DRAM_ADDR, 32, 0x00128313u32); // addi x6, x5, 1
        bus.store(8 + DRAM_ADDR, 32, 0x00100073u32); // ebreak
        bus.store(12 + DRAM_ADDR, 32, 0xff5ff0efu32); // jal x0, -12
    }

    // ---- Setup Ratatui ----
    let mut interface = UserInterface::init()?;

    loop {
        interface.render(&rv_base)?;

        match interface.event(&mut rv_base)? {
            ui::UIEvent::Nothing => (),
            ui::UIEvent::Tick => {
                interface.tick_event(tick(&mut rv_base, &mut rv_m, &mut rv_f));
            }
            ui::UIEvent::Exit => {
                drop(interface);
                break;
            }
        }
    }

    Ok(())
}

fn tick(base: &mut RV32I, rv_m: &mut RV32M, rv_f: &mut RV32F) -> TickResult {
    //println!("---- Debug ----\n{:?}\n---- Debug ----", base);

    //println!("fetch");
    let instruction = base.fetch() as u32;

    //println!("execute");

    let (mut ecall, mut ebreak) = (false, false);

    // RV_I
    if !(ecall || ebreak) {
        let result = base.execute(instruction);
        match result {
            EResult::ECall => ecall = true,
            EResult::EBreak => ebreak = true,
            EResult::Found => return TickResult::Nothing,
            EResult::NotFound => (),
        }
    }

    // RV_M
    if !(ecall || ebreak) {
        let result = rv_m.execute(instruction, base);
        match result {
            EResult::ECall => ecall = true,
            EResult::EBreak => ebreak = true,
            EResult::Found => return TickResult::Nothing,
            EResult::NotFound => (),
        }
    }

    // RV_F
    if !(ecall || ebreak) {
        let result = rv_f.execute(instruction, base);
        match result {
            EResult::ECall => ecall = true,
            EResult::EBreak => ebreak = true,
            EResult::Found => return TickResult::Nothing,
            EResult::NotFound => (),
        }
    }

    // Execution Environment
    match (ecall, ebreak) {
        (true, false) => TickResult::ECall,
        (false, true) => TickResult::EBreak,
        _ => {
            //eprintln!("instruction not supported");
            TickResult::Nothing
        }
    }
}

pub enum TickResult {
    Nothing,
    ECall,
    EBreak,
}
