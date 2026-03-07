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
    SLTI,
    SLTIU,
}

impl IType {
    pub fn funct(self) -> u32 {
        match self {
            IType::ADDI  => 0x0,
            IType::XORI  => 0x4,
            IType::ORI   => 0x6,
            IType::ANDI  => 0x7,
            IType::SLTI  => 0x2,
            IType::SLTIU => 0x3,
        }
    }
}

pub enum ITypeShift {
    SLLI,
    SRLI,
    SRAI,
}

impl ITypeShift {
    pub fn funct(self) -> (u32, u32) {
        match self {
            ITypeShift::SLLI  => (0x1, 0b000_0000),
            ITypeShift::SRLI  => (0x5, 0b000_0000),
            ITypeShift::SRAI  => (0x5, 0b010_0000),
        }
    }
}

pub fn encode_itype(op: IType, rs1: u32, imm: i32, rd: u32) -> u32 {
    let opcode = 0b001_0011;
    let funct3 = op.funct();

    ((imm as u32) & 0xFFF) << 20
        | (rs1 & 0x1F) << 15
        | funct3 << 12
        | (rd & 0x1F) << 7
        | opcode
}

pub fn encode_itype_shift(op: ITypeShift, rs1: u32, shamt: u32, rd: u32) -> u32 {
    let opcode = 0b001_0011;
    let (funct3, funct7) = op.funct();

    funct7 << 25
        | (shamt & 0x1F) << 20
        | (rs1 & 0x1F) << 15
        | funct3 << 12
        | (rd & 0x1F) << 7
        | opcode
}

pub enum Load {
    LB,
    LH,
    LW,
    LBU,
    LHU,
}

impl Load {
    pub fn funct(self) -> u32 {
        match self {
            Load::LB  => 0x0,
            Load::LH  => 0x1,
            Load::LW  => 0x2,
            Load::LBU => 0x4,
            Load::LHU => 0x5,
        }
    }
}

pub fn encode_load(op: Load, rs1: u32, imm: i32, rd: u32) -> u32 {
    let opcode = 0b000_0011;
    let funct3 = op.funct();

    ((imm as u32) & 0xFFF) << 20
        | (rs1 & 0x1F) << 15
        | funct3 << 12
        | (rd & 0x1F) << 7
        | opcode
}
