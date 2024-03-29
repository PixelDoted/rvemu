use rv32i::RV32I;
use rvcore::{
    bus::{Bus, DRAM_ADDR},
    Base, DRam, EResult, Extension,
};

fn main() {
    let bus = Bus::new(DRam::new(1024 * 1024));
    let mut base = RV32I::new(bus);

    let mut rv_m = rv_m::RV32M;
    let mut rv_f = rv_f::RV32F::default();

    {
        let bus = base.bus();
        bus.store(DRAM_ADDR, 32, 0x00130293u32); // addi x5, x6, 1
        bus.store(4 + DRAM_ADDR, 32, 0x00128313u32); // addi x6, x5, 1
        bus.store(8 + DRAM_ADDR, 32, 0xff1ff06fu32); // jal x0, -8
    }

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        println!("---- Debug ----\n{:?}\n---- Debug ----", base);

        println!("fetch");
        let instruction = base.fetch() as u32;

        println!("execute");

        // RV_I
        let result = base.execute(instruction);
        match result {
            EResult::Environment(_) => todo!(),
            EResult::Found => continue,
            EResult::NotFound => (),
        }

        // RV_M
        let result = rv_m.execute(instruction, &mut base);
        match result {
            EResult::Environment(_) => todo!(),
            EResult::Found => continue,
            EResult::NotFound => (),
        }

        // RV_F
        let result = rv_f.execute(instruction, &mut base);
        match result {
            EResult::Environment(_) => todo!(),
            EResult::Found => continue,
            EResult::NotFound => (),
        }

        eprintln!("instruction not supported");
    }
}
