use riscv_isa_sim::Cpu;

#[test]
fn reg0_is_always_zero() {
    let mut cpu = Cpu::new();

    cpu.set_reg(0, 42);
    assert_eq!(cpu.get_reg(0), 0);
}
