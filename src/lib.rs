pub mod cpu;
pub mod encode;
pub mod memory;
pub mod trap;

pub use cpu::Cpu;
pub use encode::{encode_branch, encode_itype, encode_itype_jump, encode_itype_shift, encode_jump, encode_load, encode_rtype, encode_store, Branch, IType, ITypeJump, ITypeShift, Jump, Load, RType, Store};
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
        0b011_0011 => {
            let a = cpu.get_reg(rs1 as usize);
            let b = cpu.get_reg(rs2 as usize);
            let shift = b & 0x1F;
            let result = match (funct3, funct7) {
                (0x0, 0b000_0000) => a.wrapping_add(b),
                (0x0, 0b010_0000) => a.wrapping_sub(b),
                (0x7, 0b000_0000) => a & b,
                (0x6, 0b000_0000) => a | b,
                (0x4, 0b000_0000) => a ^ b,
                (0x1, 0b000_0000) => a << shift,
                (0x5, 0b000_0000) => a >> shift,
                (0x5, 0b010_0000) => ((a as i32) >> shift) as u32,
                (0x2, 0b000_0000) => ((a as i32) < (b as i32)) as u32,
                (0x3, 0b000_0000) => (a < b) as u32,
                _ => return Err(Trap::InvalidInstruction(instr)),
            };
            cpu.set_reg(rd as usize, result);
            cpu.set_pc(pc_next);
        }
        0b001_0011 => {
            let a = cpu.get_reg(rs1 as usize);
            let b = i_imm(instr);
            let shift = b & 0x1F;
            let result = match (funct3, funct7) {
                (0x0, _) => a.wrapping_add(b),
                (0x4, _) => a ^ b,
                (0x6, _) => a | b,
                (0x7, _) => a & b,
                (0x1, 0b000_0000) => a << shift,
                (0x5, 0b000_0000) => a >> shift,
                (0x5, 0b010_0000) => ((a as i32) >> shift) as u32,
                (0x2, _) => ((a as i32) < (b as i32)) as u32,
                (0x3, _) => (a < b) as u32,
                _ => return Err(Trap::InvalidInstruction(instr)),
            };
            cpu.set_reg(rd as usize, result);
            cpu.set_pc(pc_next);
        }
        0b000_0011 => {
            let a = cpu.get_reg(rs1 as usize);
            let b = i_imm(instr);
            let result = match funct3 {
                // rust sign extends when casting from signed to wider type
                0x0 => mem.load_u8(a.wrapping_add(b))? as i8 as u32,
                0x1 => mem.load_u16(a.wrapping_add(b))? as i16 as u32,
                0x2 => mem.load_u32(a.wrapping_add(b))?,
                0x4 => mem.load_u8(a.wrapping_add(b))? as u32,
                0x5 => mem.load_u16(a.wrapping_add(b))? as u32,
                _ => return Err(Trap::InvalidInstruction(instr)),
            };
            cpu.set_reg(rd as usize, result);
            cpu.set_pc(pc_next);
        }
        0b010_0011 => {
            let a = cpu.get_reg(rs1 as usize);
            let b = cpu.get_reg(rs2 as usize);
            let imm = s_imm(instr);
            match funct3 {
                0x0 => mem.store_u8(a.wrapping_add(imm), (b & 0xFF) as u8)?,
                0x1 => mem.store_u16(a.wrapping_add(imm), (b & 0xFFFF) as u16)?,
                0x2 => mem.store_u32(a.wrapping_add(imm), b)?,
                _ => return Err(Trap::InvalidInstruction(instr)),
            };
            cpu.set_pc(pc_next);
        }
        0b110_0011 => {
            let a = cpu.get_reg(rs1 as usize);
            let b = cpu.get_reg(rs2 as usize);
            let imm = b_imm(instr);
            let result = match funct3 {
                0x0 => a == b,
                0x1 => a != b,
                0x4 => (a as i32) < (b as i32),
                0x5 => (a as i32) >= (b as i32),
                0x6 => a < b,
                0x7 => a >= b,
                _ => return Err(Trap::InvalidInstruction(instr)),
            };
            if result { cpu.set_pc(cpu.pc().wrapping_add(imm)); } else { cpu.set_pc( pc_next ); }
        }
        0b110_1111 => {
            let imm = j_imm(instr);
            println!("{}", imm as i32);
            cpu.set_reg(rd as usize, pc_next);
            cpu.set_pc(cpu.pc().wrapping_add(imm));
        }
        0b110_0111 => {
            let a = cpu.get_reg(rs1 as usize);
            let imm = i_imm(instr);
            match funct3 {
                0x0 => {
                    cpu.set_reg(rd as usize, pc_next);
                    cpu.set_pc((a.wrapping_add(imm)) & 0xFFFF_FFFE);
                },
                _ => return Err(Trap::InvalidInstruction(instr)),
            }
        }
        _ => return Err(Trap::InvalidInstruction(instr)),
    }

    Ok(())
}

fn i_imm(instr: u32) -> u32 {
    ((instr as i32) >> 20) as u32
}

fn s_imm(instr: u32) -> u32 {
    let imm = (instr & 0xFE00_0000) | ((instr & 0x0000_0F80) << 13);
    ((imm as i32) >> 20) as u32
}

fn b_imm(instr: u32) -> u32 {
    let imm = (instr & 0x8000_0000) | ((instr & 0x0000_0080) << 23) | ((instr & 0x7E00_0000) >> 1) | ((instr & 0x0000_0F00) << 12);
    ((imm as i32) >> 19) as u32
}

fn j_imm(instr: u32) -> u32 {
    let imm = (instr & 0x8000_0000) | ((instr & 0x000F_F000) << 11) | ((instr & 0x0010_0000) << 2) | ((instr & 0x7FE0_0000) >> 9);
    ((imm as i32) >> 11) as u32
}
