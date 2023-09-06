use rv32i::{Memory, RV32I};
use rvcore::{Base, Extension};

fn main() {
    let mut base = RV32I::default();
    let mut memory = Memory::new(1024 * 1024);

    let mut rv_m = rv_m::RvM;

    memory.store_word(0, 0x00130293u32 as i32); // addi x5, x6, 1
    memory.store_word(4, 0x00128313u32 as i32); // addi x6, x5, 1
    memory.store_word(8, 0xff1ff06fu32 as i32); // jal x0, -81

    loop {
        println!("fetch");
        let pc = base.fetch();
        let word = memory.load_word(pc as usize);

        if let Some(instruction) = rvcore::Instruction::decode(word as u32) {
            println!("execute");

            let result = base.execute(&instruction, &mut memory).is_some()
                || rv_m.execute(&instruction, &mut base).is_some();

            if !result {
                eprintln!("instruction not supported");
            }
        } else {
            panic!("unsupported opcode");
        }

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
