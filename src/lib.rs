pub mod cpu;
pub mod encode;
pub mod memory;
pub mod trap;

pub use cpu::Cpu;
pub use encode::{encode_rtype, RType};
pub use memory::Memory;
pub use trap::Trap;

pub fn step(cpu: &mut Cpu, mem: &mut Memory) -> Result<(), Trap> {
    if cpu.pc() % 4 != 0 {
        return Err(Trap::MisalignedFetch(cpu.pc()));
    }

    let instr = mem.load_u32(cpu.pc())?;
    let pc_next = cpu.pc().wrapping_add(4);

    let opcode = instr & 0x7F;
    let rd = (instr >> 7) & 0x1F;
    let funct3 = (instr >> 12) & 0x7;
    let rs1 = (instr >> 15) & 0x1F;
    let rs2 = (instr >> 20) & 0x1F;
    let funct7 = (instr >> 25) & 0x7F;

    match opcode {
        0b0110011 => {
            let a = cpu.get_reg(rs1 as usize);
            let b = cpu.get_reg(rs2 as usize);
            let shift = b & 0x1F;
            let result = match (funct3, funct7) {
                (0b000, 0b000_0000) => a.wrapping_add(b),
                (0b000, 0b010_0000) => a.wrapping_sub(b),
                (0b111, 0b000_0000) => a & b,
                (0b110, 0b000_0000) => a | b,
                (0b100, 0b000_0000) => a ^ b,
                (0b001, 0b000_0000) => a << shift,
                (0b101, 0b000_0000) => a >> shift,
                (0b101, 0b010_0000) => ((a as i32) >> shift) as u32,
                (0b010, 0b000_0000) => ((a as i32) < (b as i32)) as u32,
                (0b011, 0b000_0000) => (a < b) as u32,
                _ => return Err(Trap::InvalidInstruction(instr)),
            };
            cpu.set_reg(rd as usize, result);
            cpu.set_pc(pc_next);
        }
        _ => return Err(Trap::InvalidInstruction(instr)),
    }

    Ok(())
}
