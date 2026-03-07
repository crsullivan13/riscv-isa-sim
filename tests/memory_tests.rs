use riscv_isa_sim::{Memory, Trap};

fn mk_mem() -> Memory {
    Memory::new(20, 0x1000)
}

#[test]
fn u8_store_load_roundtrip() {
    let mut mem = mk_mem();
    let addr = 0x1000;

    mem.store_u8(addr, 0xFF).unwrap();
    assert_eq!(mem.load_u8(addr).unwrap(), 0xFF);
}

#[test]
fn u16_store_load_roundtrip() {
    let mut mem = mk_mem();
    let addr = 0x1000;

    mem.store_u16(addr, 0xFFFF).unwrap();
    assert_eq!(mem.load_u16(addr).unwrap(), 0xFFFF);
}

#[test]
fn u32_roundtrip() {
    let mut mem = mk_mem();
    let addr = 0x1000;

    mem.store_u32(addr, 0xDEADBEEF).unwrap();
    assert_eq!(mem.load_u8(addr).unwrap(), 0xEF);
    assert_eq!(mem.load_u8(addr + 1).unwrap(), 0xBE);
    assert_eq!(mem.load_u8(addr + 2).unwrap(), 0xAD);
    assert_eq!(mem.load_u8(addr + 3).unwrap(), 0xDE);
    assert_eq!(mem.load_u32(addr).unwrap(), 0xDEADBEEF);
}

#[test]
fn misaligned_store_errors() {
    let mut mem = mk_mem();
    let addr = 0x1001;

    match mem.store_u32(addr, 0xDEADBEEF) {
        Err(Trap::MisalignedStore(a)) => assert_eq!(a, 0x1001),
        _ => panic!("expected MisalignedStore"),
    }
}

#[test]
fn misaligned_load_errors() {
    let mem = mk_mem();
    let addr = 0x1001;

    match mem.load_u32(addr) {
        Err(Trap::MisalignedLoad(a)) => assert_eq!(a, 0x1001),
        _ => panic!("expected MisalignedLoad"),
    }
}

#[test]
fn out_of_bounds_errors() {
    let mut mem = mk_mem();

    let below = 0x0999;
    match mem.load_u8(below) {
        Err(Trap::OutOfBounds(a)) => assert_eq!(a, 0x0999),
        _ => panic!("expected OutOfBounds"),
    }

    let above = 0x1100;
    match mem.store_u8(above, 0xFF) {
        Err(Trap::OutOfBounds(a)) => assert_eq!(a, 0x1100),
        _ => panic!("expected OutOfBounds"),
    }
}
