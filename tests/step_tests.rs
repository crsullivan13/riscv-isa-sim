use riscv_isa_sim::{encode_branch, encode_itype, encode_itype_shift, encode_load, encode_rtype, encode_store, step, Cpu, Branch, IType, ITypeShift, Load, Memory, RType, Store, Trap};
use riscv_isa_sim::encode::{encode_jump, encode_itype_jump, ITypeJump, Jump};

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

// -- Loads --

#[test]
fn lb_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    mem.store_u32(0x0, encode_load(Load::LB, 1, 4, 2)).unwrap();
    mem.store_u32(0x4, 0xDEAD_BEFF).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 0xFFFF_FFFF);
}

#[test]
fn lh_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    mem.store_u32(0x0, encode_load(Load::LH, 1, 4, 2)).unwrap();
    mem.store_u32(0x4, 0xDEAD_FFFF).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 0xFFFF_FFFF);
}

#[test]
fn lw_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    mem.store_u32(0x0, encode_load(Load::LW, 1, 4, 2)).unwrap();
    mem.store_u32(0x4, 0xDEAD_BEEF).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 0xDEAD_BEEF);
}

#[test]
fn lbu_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    mem.store_u32(0x0, encode_load(Load::LBU, 1, 4, 2)).unwrap();
    mem.store_u32(0x4, 0xDEAD_BEFF).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 0xFF);
}

#[test]
fn lhu_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    mem.store_u32(0x0, encode_load(Load::LHU, 1, 4, 2)).unwrap();
    mem.store_u32(0x4, 0xDEAD_FFFF).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(2), 0xFFFF);
}

// --- Stores ---

#[test]
fn sb_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(2, 0xDEAD_BEEF);
    mem.store_u32(0x0, encode_store(Store::SB, 1, 2, 4)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(mem.load_u32(0x4).unwrap(), cpu.get_reg(2) & 0xFF);
}

#[test]
fn sh_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(2, 0xDEAD_BEEF);
    mem.store_u32(0x0, encode_store(Store::SH, 1, 2, 4)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(mem.load_u32(0x4).unwrap(), cpu.get_reg(2) & 0xFFFF);
}

#[test]
fn sw_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(2, 0xDEAD_BEEF);
    cpu.set_reg(1, 8);
    mem.store_u32(0x0, encode_store(Store::SW, 1, 2, -4)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(mem.load_u32(0x4).unwrap(), cpu.get_reg(2));
}

// --- Branches ---

#[test]
fn beq_taken() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 5);
    cpu.set_reg(2, 5);
    mem.store_u32(0x0, encode_branch(Branch::BEQ, 1, 2, 8)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.pc(), 8);
}

#[test]
fn beq_not_taken() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 5);
    cpu.set_reg(2, 6);
    mem.store_u32(0x0, encode_branch(Branch::BEQ, 1, 2, 8)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.pc(), 4);
}

#[test]
fn bne_taken() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 5);
    cpu.set_reg(2, 6);
    mem.store_u32(0x0, encode_branch(Branch::BNE, 1, 2, 8)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.pc(), 8);
}

#[test]
fn bne_not_taken() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 5);
    cpu.set_reg(2, 5);
    mem.store_u32(0x0, encode_branch(Branch::BNE, 1, 2, 8)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.pc(), 4);
}

#[test]
fn blt_taken() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, (-1i32) as u32);
    cpu.set_reg(2, 0);
    mem.store_u32(0x0, encode_branch(Branch::BLT, 1, 2, 8)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.pc(), 8);
}

#[test]
fn blt_not_taken() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 1);
    cpu.set_reg(2, 0);
    mem.store_u32(0x0, encode_branch(Branch::BLT, 1, 2, 8)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.pc(), 4);
}

#[test]
fn bge_taken() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 5);
    cpu.set_reg(2, 5);
    mem.store_u32(0x0, encode_branch(Branch::BGE, 1, 2, 8)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.pc(), 8);
}

#[test]
fn bge_not_taken() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, (-1i32) as u32);
    cpu.set_reg(2, 0);
    mem.store_u32(0x0, encode_branch(Branch::BGE, 1, 2, 8)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.pc(), 4);
}

#[test]
fn bltu_taken() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 1);
    cpu.set_reg(2, 2);
    mem.store_u32(0x0, encode_branch(Branch::BLTU, 1, 2, 8)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.pc(), 8);
}

#[test]
fn bltu_not_taken() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 2);
    cpu.set_reg(2, 1);
    mem.store_u32(0x0, encode_branch(Branch::BLTU, 1, 2, 8)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.pc(), 4);
}

#[test]
fn bgeu_taken() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 5);
    cpu.set_reg(2, 5);
    mem.store_u32(0x0, encode_branch(Branch::BGEU, 1, 2, 8)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.pc(), 8);
}

#[test]
fn bgeu_not_taken() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(1, 1);
    cpu.set_reg(2, 2);
    mem.store_u32(0x0, encode_branch(Branch::BGEU, 1, 2, 8)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.pc(), 4);
}

// --- Jumps ---

#[test]
fn jal_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    mem.store_u32(0x0, encode_jump(Jump::JAL, 1, 8)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(1), 4);
    assert_eq!(cpu.pc(), 8);
}

#[test]
fn jal_neg_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x4);
    cpu.set_pc(0x4);
    mem.store_u32(0x4, encode_jump(Jump::JAL, 1, -4)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(1), 8);
    assert_eq!(cpu.pc(), 0);
}

#[test]
fn jalr_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);
    cpu.set_reg(5, 12);
    mem.store_u32(0x0, encode_itype_jump(ITypeJump::JALR, 1, 5, 4)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(1), 4);
    assert_eq!(cpu.pc(), 16);
}

#[test]
fn jalr_neg_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x4);
    cpu.set_pc(0x4);
    cpu.set_reg(5, 12);
    mem.store_u32(0x4, encode_itype_jump(ITypeJump::JALR, 1, 5, -3)).unwrap();
    step(&mut cpu, &mut mem).unwrap();
    assert_eq!(cpu.get_reg(1), 8);
    assert_eq!(cpu.pc(), 8);
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
