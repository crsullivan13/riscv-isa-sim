use riscv_isa_sim::{encode_rtype, step, Cpu, Memory, RType, Trap};

#[test]
fn add_instruction_executes() {
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
fn sub_instruction_executes() {
    let mut cpu = Cpu::new();
    let mut mem = Memory::new(64, 0x0);

    cpu.set_reg(1, 42);
    cpu.set_reg(2, 41);
    mem.store_u32(0, encode_rtype(RType::SUB, 1, 2, 3)).unwrap();

    step(&mut cpu, &mut mem).unwrap();

    assert_eq!(cpu.get_reg(3), 1);
    assert_eq!(cpu.pc(), 4);
}

// TODO: Make one test for all RType to see that they execute properly

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
