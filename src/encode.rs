pub enum RType {
    ADD,
}

impl RType {
    pub fn funct(self) -> (u32, u32) {
        match self {
            RType::ADD => (0b000_0000, 0b000),
        }
    }
}

pub fn encode_rtype(op: RType, rs1: u32, rs2: u32, rd: u32) -> u32 {
    let opcode = 0b011_0011;
    let (funct7, funct3) = op.funct();

    funct7 << 25
        | (rs2 & 0x1F) << 20
        | (rs1 & 0x1F) << 15
        | funct3 << 12
        | (rd & 0x1F) << 7
        | opcode
}
