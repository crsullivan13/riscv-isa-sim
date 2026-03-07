pub enum RType {
    ADD,
    SUB,
    AND,
    OR,
    XOR,
    SLL,
    SRL,
    SRA,
    SLT,
    SLTU,
}

impl RType {
    pub fn funct(self) -> (u32, u32) {
        match self {
            RType::ADD  => (0x0, 0b000_0000),
            RType::SUB  => (0x0, 0b010_0000),
            RType::AND  => (0x7, 0b000_0000),
            RType::OR   => (0x6, 0b000_0000),
            RType::XOR  => (0x4, 0b000_0000),
            RType::SLL  => (0x1, 0b000_0000),
            RType::SRL  => (0x5, 0b000_0000),
            RType::SRA  => (0x5, 0b010_0000),
            RType::SLT  => (0x2, 0b000_0000),
            RType::SLTU => (0x3, 0b000_0000),
        }
    }
}

pub fn encode_rtype(op: RType, rs1: u32, rs2: u32, rd: u32) -> u32 {
    let opcode = 0b011_0011;
    let (funct3, funct7) = op.funct();

    funct7 << 25
        | (rs2 & 0x1F) << 20
        | (rs1 & 0x1F) << 15
        | funct3 << 12
        | (rd & 0x1F) << 7
        | opcode
}

pub enum IType {
    ADDI,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
    SLTI,
    SLTIU,
}

impl IType {
    pub fn funct(self) -> (u32, u32) {
        match self {
            IType::ADDI  => (0x0, 0b000_0000),
            IType::XORI  => (0x4, 0b000_0000),
            IType::ORI   => (0x6, 0b000_0000),
            IType::ANDI  => (0x7, 0b000_0000),
            IType::SLLI  => (0x1, 0b000_0000),
            IType::SRLI  => (0x5, 0b000_0000),
            IType::SRAI  => (0x5, 0b010_0000),
            IType::SLTI  => (0x2, 0b000_0000),
            IType::SLTIU => (0x3, 0b000_0000),
        }
    }
}

pub fn encode_itype(op: IType, rs1: u32, imm: i32, rd: u32) -> u32 {
    let opcode = 0b001_0011;
    let (funct3, _) = op.funct();

    ((imm as u32) & 0xFFF) << 20
        | (rs1 & 0x1F) << 15
        | funct3 << 12
        | (rd & 0x1F) << 7
        | opcode
}

pub fn encode_itype_shift(op: IType, rs1: u32, shamt: u32, rd: u32) -> u32 {
    let opcode = 0b001_0011;
    let (funct3, funct7) = op.funct();

    funct7 << 25
        | (shamt & 0x1F) << 20
        | (rs1 & 0x1F) << 15
        | funct3 << 12
        | (rd & 0x1F) << 7
        | opcode
}
