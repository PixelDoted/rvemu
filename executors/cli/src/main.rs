use rv32i::RV32I;
use rv_f::RV32FData;
use rvcore::{Base, Extension, Memory};

fn main() {
    let mut base = RV32I::default();
    let mut memory = Memory::new(1024 * 1024);

    let mut rv_m = rv_m::RV32M;
    let mut rv_f = rv_f::RV32F::default();

    memory.store_word(0, 0x00130293u32 as i32); // addi x5, x6, 1
    memory.store_word(4, 0x00128313u32 as i32); // addi x6, x5, 1
    memory.store_word(8, 0xff1ff06fu32 as i32); // jal x0, -8

    loop {
        println!("fetch");
        let pc = base.fetch();
        let instruction = memory.load_word(pc as usize) as u32;

        println!("execute");
        let result = base.execute(instruction, &mut memory).is_some()
            || rv_m.execute(instruction, &mut base).is_some()
            || rv_f
                .execute(
                    instruction,
                    &mut RV32FData {
                        base: &mut base,
                        memory: &mut memory,
                    },
                )
                .is_some();

        if !result {
            eprintln!("instruction not supported");
        }

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
