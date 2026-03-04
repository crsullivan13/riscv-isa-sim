#[derive(Debug)]
enum Trap {
    MisalignedFetch(u32),
    MisalignedLoad(u32),
    MisalignedStore(u32),
    OutOfBounds(u32),
    InvalidInstruction(u32),
}

fn main() {
}

struct Cpu {
    reg_file: [u32; 32],
    pc: u32,
}

impl Cpu {
    fn new() -> Self {
        Cpu { reg_file: [0; 32], pc: 0 }
    }

    fn set_reg(&mut self, reg: usize, value: u32) {
        // reg0 is always 0
        if reg != 0 {
            self.reg_file[reg] = value;
        }
    }

    fn get_reg(&self, reg: usize) -> u32 {
        self.reg_file[reg]
    }
}

struct Memory {
    data_array: Vec<u8>,
    base: u32,
}

impl Memory {
    fn new(init_size: usize, base: u32) -> Self {
        Memory { data_array: vec![0; init_size], base }
    }

    fn address_to_index(&self, addr: u32) -> Result<u32, Trap> {
        if addr < self.base {
            return Err(Trap::OutOfBounds(addr));
        }

        let index = addr - self.base;

        if index >= self.data_array.len() as u32 {
            return Err(Trap::OutOfBounds(addr));
        }

        Ok(index)
    }

    fn load_u8(&self, addr: u32) -> Result<u8, Trap> {
        let index = self.address_to_index(addr)?;
        Ok(self.data_array[index as usize])
    }

    fn store_u8(&mut self, addr: u32, value: u8) -> Result<(), Trap> {
        let index = self.address_to_index(addr)?;
        self.data_array[index as usize] = value;
        Ok(())
    }

    fn load_u32(&self, addr: u32) -> Result<u32, Trap> {
        if addr % 4 != 0 {
            return Err(Trap::MisalignedLoad(addr));
        }

        let b1 = self.load_u8(addr + 0)?;
        let b2 = self.load_u8(addr + 1)?;
        let b3 = self.load_u8(addr + 2)?;
        let b4 = self.load_u8(addr + 3)?;

        Ok((b4 as u32) << 24 | (b3 as u32) << 16 | (b2 as u32) << 8 | b1 as u32)
    }

    fn store_u32(&mut self, addr: u32, value: u32) -> Result<(), Trap> {
        if addr % 4 != 0 {
            return Err(Trap::MisalignedStore(addr));
        }

        self.store_u8(addr + 0, (value & 0xff) as u8)?;
        self.store_u8(addr + 1, (value >> 8 & 0xff) as u8)?;
        self.store_u8(addr + 2, (value >> 16 & 0xff) as u8)?;
        self.store_u8(addr + 3, (value >> 24 & 0xff) as u8)?;

        Ok(())
    }
}

fn step(cpu: &mut Cpu, mem: &mut Memory) -> Result<(), Trap> {
    if cpu.pc % 4 != 0 {
        return Err(Trap::MisalignedFetch(cpu.pc));
    }

    let instr = mem.load_u32(cpu.pc)?;
    let pc_next = cpu.pc + 4;

    let opcode = instr & 0x7F;
    let rd = ( instr >> 7 ) & 0x1F;
    let funct3 = ( instr >> 12 ) & 0x7;
    let rs1 = ( instr >> 15 ) & 0x1F;
    let rs2 = ( instr >> 20 ) & 0x1F;
    let funct7 = ( instr >> 25 ) & 0x7F;

    match opcode {
        0b0110011 => {
            let a = cpu.get_reg(rs1 as usize);
            let b = cpu.get_reg(rs2 as usize);
            let result = match (funct3, funct7) {
                (0b000, 0b000_0000) => a + b, // ADD
                (0b000, 0b010_0000) => a - b, // SUB
                (0b111, 0b000_0000) => a & b, // AND
                (0b110, 0b000_0000) => a | b, // OR
                (0b100, 0b000_0000) => a ^ b, // XOR
                (0b001, 0b000_0000) => a << ( b & 0x1F) , // Shift Left Logical
                (0b101, 0b000_0000) => a >> ( b & 0x1F ), // Shift Right Logical
                (0b101, 0b010_0000) => ( ( a as i32 ) >> ( b & 0x1F ) ) as u32, // Shift Right Arithmetic
                (0b010, 0b000_0000) => ( ( a as i32 ) < ( b as i32 ) ) as u32, // Set Less Than
                (0b011, 0b000_0000) => ( a < b ) as u32, // Set Less Than Unsigned
                _ => return  Err(Trap::InvalidInstruction(instr))
            };
            cpu.set_reg(rd as usize, result);
            cpu.pc = pc_next;
        }
        _ => return  Err(Trap::InvalidInstruction(instr))
    }


    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_cpu() -> Cpu {
        Cpu { reg_file: [0;32], pc: 0x100 }
    }

    fn mk_mem() -> Memory {
        Memory { data_array: vec![0;20], base: 0x1000 }
    }

    // Memory tests
    #[test]
    fn u8_store_load_roundtrip() {
        let mut mem = mk_mem();
        let addr = 0x1000;
        mem.store_u8(addr, 0xFF).unwrap();
        assert_eq!(mem.load_u8(addr).unwrap(), 0xFF);
    }

    #[test]
    fn u32_rountrip() {
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
    fn outofbounds_errors() {
        let mut mem = mk_mem();

        let below = 0x999;
        match mem.load_u8(below) {
            Err(Trap::OutOfBounds(a)) => assert_eq!(a, 0x999),
            _ => panic!("expected OutOfBounds"),
        }

        let above = 0x10000;
        match mem.store_u8(above, 0xFF) {
            Err(Trap::OutOfBounds(a)) => assert_eq!(a, 0x10000),
            _ => panic!("expected OutOfBounds"),
        }
    }
    // End memory tests

    // Cpu tests
    #[test]
    fn reg0_is_always_zero() {
        let mut cpu = mk_cpu();

        cpu.set_reg(0, 42);
        assert_eq!(cpu.get_reg(0), 0);
    }
}
