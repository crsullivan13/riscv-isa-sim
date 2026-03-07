// Default trait used when it makes sense to default to all zeros
#[derive(Default)]
pub struct Cpu {
    reg_file: [u32; 32],
    pc: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, value: u32) {
        self.pc = value;
    }

    pub fn set_reg(&mut self, reg: usize, value: u32) {
        // reg0 is always 0
        if reg != 0 {
            self.reg_file[reg] = value;
        }
    }

    pub fn get_reg(&self, reg: usize) -> u32 {
        assert!(reg < 32);
        self.reg_file[reg]
    }
}
