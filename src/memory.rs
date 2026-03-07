use crate::trap::Trap;

pub struct Memory {
    data_array: Vec<u8>,
    base: u32,
}

impl Memory {
    pub fn new(init_size: usize, base: u32) -> Self {
        Memory {
            data_array: vec![0; init_size],
            base,
        }
    }

    fn address_to_index(&self, addr: u32) -> Result<usize, Trap> {
        if addr < self.base {
            return Err(Trap::OutOfBounds(addr));
        }

        let index = addr - self.base;

        if index >= self.data_array.len() as u32 {
            return Err(Trap::OutOfBounds(addr));
        }

        Ok(index as usize)
    }

    pub fn load_u8(&self, addr: u32) -> Result<u8, Trap> {
        let index = self.address_to_index(addr)?;
        Ok(self.data_array[index])
    }

    pub fn store_u8(&mut self, addr: u32, value: u8) -> Result<(), Trap> {
        let index = self.address_to_index(addr)?;
        self.data_array[index] = value;
        Ok(())
    }

    pub fn load_u16(&self, addr: u32) -> Result<u16, Trap> {
        if addr % 2 != 0 {
            return Err(Trap::MisalignedLoad(addr));
        }

        let b1 = self.load_u8(addr)? as u16;
        let b2 = self.load_u8(addr + 1)? as u16;

        Ok((b2 << 8) | b1)
    }

    pub fn store_u16(&mut self, addr: u32, value: u16) -> Result<(), Trap> {
        if addr % 2 != 0 {
            return Err(Trap::MisalignedStore(addr));
        }

        self.store_u8(addr, (value & 0xff) as u8)?;
        self.store_u8(addr + 1, ((value >> 8) & 0xff) as u8)?;

        Ok(())
    }

    pub fn load_u32(&self, addr: u32) -> Result<u32, Trap> {
        if addr % 4 != 0 {
            return Err(Trap::MisalignedLoad(addr));
        }

        let b1 = self.load_u8(addr)? as u32;
        let b2 = self.load_u8(addr + 1)? as u32;
        let b3 = self.load_u8(addr + 2)? as u32;
        let b4 = self.load_u8(addr + 3)? as u32;

        Ok((b4 << 24) | (b3 << 16) | (b2 << 8) | b1)
    }

    pub fn store_u32(&mut self, addr: u32, value: u32) -> Result<(), Trap> {
        if addr % 4 != 0 {
            return Err(Trap::MisalignedStore(addr));
        }

        self.store_u8(addr, (value & 0xff) as u8)?;
        self.store_u8(addr + 1, ((value >> 8) & 0xff) as u8)?;
        self.store_u8(addr + 2, ((value >> 16) & 0xff) as u8)?;
        self.store_u8(addr + 3, ((value >> 24) & 0xff) as u8)?;

        Ok(())
    }
}
