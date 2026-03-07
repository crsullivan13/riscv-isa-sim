use riscv_isa_sim::{encode_itype, encode_itype_shift, encode_rtype, step, Cpu, IType, ITypeShift, Memory, RType, Trap};

// --- R-type ---

#[test]
fn add_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 42);
    cpu.set_reg(2, 42);
    mem.store_u32(0, encode_rtype(RType::ADD, 1, 2, 3)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(3), 84);
    assert_eq!(cpu.pc(), 4);
}

#[test]
fn sub_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 42);
    cpu.set_reg(2, 41);
    mem.store_u32(0, encode_rtype(RType::SUB, 1, 2, 3)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(3), 1);
    assert_eq!(cpu.pc(), 4);
}

#[test]
fn and_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 0b1100);
    cpu.set_reg(2, 0b1010);
    mem.store_u32(0, encode_rtype(RType::AND, 1, 2, 3)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(3), 0b1000);
}

#[test]
fn or_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 0b1100);
    cpu.set_reg(2, 0b1010);
    mem.store_u32(0, encode_rtype(RType::OR, 1, 2, 3)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(3), 0b1110);
}

#[test]
fn xor_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 0b1100);
    cpu.set_reg(2, 0b1010);
    mem.store_u32(0, encode_rtype(RType::XOR, 1, 2, 3)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(3), 0b0110);
}

#[test]
fn sll_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 1);
    cpu.set_reg(2, 3);
    mem.store_u32(0, encode_rtype(RType::SLL, 1, 2, 3)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(3), 8);
}

#[test]
fn srl_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 8);
    cpu.set_reg(2, 1);
    mem.store_u32(0, encode_rtype(RType::SRL, 1, 2, 3)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(3), 4);
}

#[test]
fn sra_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, (-8i32) as u32);
    cpu.set_reg(2, 1);
    mem.store_u32(0, encode_rtype(RType::SRA, 1, 2, 3)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(3) as i32, -4);
}

#[test]
fn slt_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, (-1i32) as u32);
    cpu.set_reg(2, 0);
    mem.store_u32(0, encode_rtype(RType::SLT, 1, 2, 3)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(3), 1);
}

#[test]
fn sltu_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 1);
    cpu.set_reg(2, 2);
    mem.store_u32(0, encode_rtype(RType::SLTU, 1, 2, 3)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(3), 1);
}

// --- I-type ---

#[test]
fn addi_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 10);
    mem.store_u32(0, encode_itype(IType::ADDI, 1, 5, 2)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 15);
}

#[test]
fn addi_negative_imm_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 10);
    mem.store_u32(0, encode_itype(IType::ADDI, 1, -1, 2)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 9);
}

#[test]
fn xori_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 0b1100);
    mem.store_u32(0, encode_itype(IType::XORI, 1, 0b1010, 2)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 0b0110);
}

#[test]
fn ori_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 0b1100);
    mem.store_u32(0, encode_itype(IType::ORI, 1, 0b1010, 2)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 0b1110);
}

#[test]
fn andi_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 0b1100);
    mem.store_u32(0, encode_itype(IType::ANDI, 1, 0b1010, 2)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 0b1000);
}

#[test]
fn slli_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 1);
    mem.store_u32(0, encode_itype_shift(ITypeShift::SLLI, 1, 3, 2)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 8);
}

#[test]
fn srli_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 8);
    mem.store_u32(0, encode_itype_shift(ITypeShift::SRLI, 1, 1, 2)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 4);
}

#[test]
fn srai_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, (-8i32) as u32);
    mem.store_u32(0, encode_itype_shift(ITypeShift::SRAI, 1, 1, 2)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2) as i32, -4);
}

#[test]
fn slti_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, (-1i32) as u32);
    mem.store_u32(0, encode_itype(IType::SLTI, 1, 0, 2)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 1);
}

#[test]
fn sltiu_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 1);
    mem.store_u32(0, encode_itype(IType::SLTIU, 1, 2, 2)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 1);
}

// --- Traps ---

#[test]
fn aligned_fetch_error() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_pc(1);
    match step(&mut cpu, &mut mem) {
        Err(Trap::MisalignedFetch(pc)) => assert_eq!(pc, 1),
        _ => panic!("expected MisalignedFetch"),
    }
}
