#[derive(Debug)]
pub struct ElfHeader {
    e_entry: u32,     // entry point
    e_phoff: u32,     // prog header offest
    e_phentsize: u16, // prog header entry size
    e_phnum: u16,     // num entries in prog header
}

impl ElfHeader {
    pub fn parse(bytes: &[u8]) -> Self {
        Self {
            e_entry: u32::from_le_bytes(bytes[24..28].try_into().unwrap()),
            e_phoff: u32::from_le_bytes(bytes[28..32].try_into().unwrap()),
            e_phentsize: u16::from_le_bytes(bytes[42..44].try_into().unwrap()),
            e_phnum: u16::from_le_bytes(bytes[44..46].try_into().unwrap()),
        }
    }

    pub fn entry(&self) -> u32 {
        self.e_entry
    }

    pub fn phoff(&self) -> u32 {
        self.e_phoff
    }

    pub fn phentsize(&self) -> u16 {
        self.e_phentsize
    }

    pub fn e_phnum(&self) -> u16 {
        self.e_phnum
    }
}

#[derive(Debug)]
pub struct ProgramHeader {
    p_offset: u32,
    p_vaddr: u32,
    p_memsz: u32,
}

impl ProgramHeader {
    pub fn parse(bytes: &[u8], phoff: usize, phentsize: usize) -> Self {
        let bytes_adjusted = &bytes[phoff + phentsize..phentsize * 2 + phoff];
        Self {
            p_offset: u32::from_le_bytes(bytes_adjusted[4..8].try_into().unwrap()),
            p_vaddr: u32::from_le_bytes(bytes_adjusted[8..12].try_into().unwrap()),
            p_memsz: u32::from_le_bytes(bytes_adjusted[20..24].try_into().unwrap())
        }
    }

    pub fn offset(&self) -> u32 {
        self.p_offset
    }

    pub fn vaddr(&self) -> u32 {
        self.p_vaddr
    }

    pub fn memsz(&self) -> u32 {
        self.p_memsz
    }
}
