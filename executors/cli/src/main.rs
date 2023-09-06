use rv32i::{Memory, RV32I};
use rvcore::Base;

fn main() {
    let mut base = RV32I::default();
    let mut memory = Memory::new(1024 * 1024);

    memory.store_word(0, 0x00130293u32 as i32); // addi x5, x6, 1
    memory.store_word(4, 0x00128313u32 as i32); // addi x6, x5, 1
    memory.store_word(8, 0xff1ff06fu32 as i32); // jal x0, -81

    loop {
        println!("fetch");
        let pc = base.fetch();
        let ins = memory.load_word(pc as usize) as u32;

        println!("execute");
        if base.execute(ins, &mut memory).is_none() {
            println!("instruction not supported");
        }

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
