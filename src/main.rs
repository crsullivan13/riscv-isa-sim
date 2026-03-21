use std::fs;

pub use riscv_isa_sim::elf_parser::*;
pub use riscv_isa_sim::Cpu;
pub use riscv_isa_sim::Memory;
pub use riscv_isa_sim::step;

fn main() {
    let data: Vec<u8> = fs::read("rv32i-tests/a.out").unwrap();
    let header = ElfHeader::parse(&data);
    let prog_header = ProgramHeader::parse(&data, header.phoff() as usize, header.phentsize() as usize);

    println!("{:?}", header);
    println!("{:?}", prog_header);

    let mut memory = Memory::new(1024, 0x10000);
    let mut cpu = Cpu::new();
    cpu.set_pc(header.entry());

    for i in 0..prog_header.memsz() {
        memory.store_u8(prog_header.vaddr() + i, data[(prog_header.offset() as u32 + i) as usize]).unwrap();
    }
    println!("{:?}", memory);

    cpu.set_reg(2, 0x10000 + 1024);
    loop {
        println!("step! {}", cpu.pc());
        step(&mut cpu, &mut memory).unwrap();
    }
}
