use std::convert::From;
use num_enum::IntoPrimitive;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, IntoPrimitive, Debug)]
#[repr(u8)]
pub enum OPCODE {
    STA_ZP = 0x85,
    STA_ZPX = 0x95,
    STA_A = 0x8D,
    STA_AX = 0x9D,
    STA_AY = 0x99,
    STA_IX = 0x81,
    STA_IY = 0x91,

    STX_ZP = 0x86,
    STX_ZPY = 0x96,
    STX_A = 0x8E,

    STY_ZP = 0x84,
    STY_ZPX = 0x94,
    STY_A = 0x8C,

    LDA_I = 0xA9,
    LDA_ZP = 0xA5,
    LDA_ZPX = 0xB5,
    LDA_A = 0xAD,
    LDA_AX = 0xBD,
    LDA_AY = 0xB9,
    LDA_IX = 0xA1,
    LDA_IY = 0xB1,

    LDX_I = 0xA2,
    LDX_A = 0xAE,
    LDX_AY = 0xBE,
    LDX_ZP = 0xA6,
    LDX_ZPY = 0xB6,

    LDY_I = 0xA0,
    LDY_A = 0xAC,
    LDY_AX = 0xBC,
    LDY_ZP = 0xA4,
    LDY_ZPX = 0xB4,

    CMP_I = 0xC9,
    CMP_ZP = 0xC5,
    CMP_ZPX = 0xD5,
    CMP_A = 0xCD,
    CMP_AX = 0xDD,
    CMP_AY = 0xD9,
    CMP_IX = 0xC1,
    CMP_IY = 0xD1,

    CPX_I = 0xE0,
    CPX_A = 0xEC,
    CPX_ZP = 0xE4,

    CPY_I = 0xC0,
    CPY_A = 0xCC,
    CPY_ZP = 0xC4,

    /*
        TODO: ONLY non-BDC addition is implemented
        https://www.electrical4u.com/bcd-or-binary-coded-decimal-bcd-conversion-addition-subtraction/ 
    */
    ADC_I = 0x69,
    ADC_ZP = 0x65,
    ADC_ZPX = 0x75,
    ADC_A = 0x6D,
    ADC_AX = 0x7D,
    ADC_AY = 0x79,
    ADC_IX = 0x61,
    ADC_IY = 0x71,

    AND_I = 0x29,
    AND_ZP = 0x25,
    AND_ZPX = 0x35,
    AND_A = 0x2D,
    AND_AX = 0x3D,
    AND_AY = 0x39,
    AND_IX = 0x21,
    AND_IY = 0x31,

    ASL_ACC = 0x0A,
    ASL_ZP = 0x06,
    ASL_ZPX = 0x16,
    ASL_A = 0x0E,
    ASL_AX = 0x1E,

    BIT_ZP = 0x24,
    BIT_A = 0x2C,

    /*Branch instructions*/
    BPL = 0x10,
    BMI = 0x30,
    BVC = 0x50,
    BVS = 0x70,
    BCC = 0x90,
    BCS = 0xB0,
    BNE = 0xD0,
    BEQ = 0xF0,

    BRK = 0x00,

    DEC_ZP = 0xC6,
    DEC_ZPX = 0xD6,
    DEC_A = 0xCE,
    DEC_AX = 0xDE,

    EOR_I = 0x49,
    EOR_ZP = 0x45,
    EOR_ZPX = 0x55,
    EOR_A = 0x4D,
    EOR_AX = 0x5D,
    EOR_AY = 0x59,
    EOR_IX = 0x41,
    EOR_IY = 0x51,

    CLC = 0x18,
    SEC = 0x38,
    CLI = 0x58,
    SEI = 0x78,
    CLV = 0xB8,
    CLD = 0xD8,
    SED = 0xF8,

    INC_ZP = 0xE6,
    INC_ZPX = 0xF6,
    INC_A = 0xEE,
    INC_AX = 0xFE,

    JMP_A = 0x4C,
    JMP_I = 0x6C,

    JSR = 0x20,

    LSR_ACC = 0x4A,
    LSR_ZP = 0x46,
    LSR_ZPX = 0x56,
    LSR_A = 0x4E,
    LSR_AX = 0x5E,

    NOP = 0xEA,

    ORA_I = 0x09,
    ORA_ZP = 0x05,
    ORA_ZPX = 0x15,
    ORA_A = 0x0D,
    ORA_AX = 0x1D,
    ORA_AY = 0x19,
    ORA_IX = 0x01,
    ORA_IY = 0x11,

    TAX = 0xAA,
    TXA = 0x8A,
    DEX = 0xCA,
    INX = 0xE8,
    TAY = 0xA8,
    TYA = 0x98,
    DEY = 0x88,
    INY = 0xC8,

    ROL_ACC = 0x2A,
    ROL_ZP = 0x26,
    ROL_ZPX = 0x36,
    ROL_A = 0x2E,
    ROL_AX = 0x3E,

    ROR_ACC = 0x6A,
    ROR_ZP = 0x66,
    ROR_ZPX = 0x76,
    ROR_A = 0x6E,
    ROR_AX = 0x7E,

    RTI = 0x40,
    RTS = 0x60,

    SBC_I = 0xE9,
    SBC_ZP = 0xE5,
    SBC_ZPX = 0xF5,
    SBC_A = 0xED,
    SBC_AX = 0xFD,
    SBC_AY = 0xF9,
    SBC_IX = 0xE1,
    SBC_IY = 0xF1,

    TXS = 0x9A,
    TSX = 0xBA,
    PHA = 0x48,
    PLA = 0x68,
    PHP = 0x08,
    PLP = 0x28
}

#[derive(Debug)]
pub struct Instruction {
    opc: OPCODE,
    param: Vec<u8>,
    size: u16,
    cycles: u8,
}

impl Instruction {
    pub fn size(&self) -> &u16 {
       &self.size
    }
    pub fn opc(&self) -> &OPCODE {
        &self.opc
    }
    pub fn param(&self) -> &Vec<u8> {
        &self.param
    }
    pub fn cycles(&self) -> &u8 {
        &self.cycles
    }
    pub fn convert_to_mem_layout(&self) -> Vec<u8> {
        let mut layout = vec![self.opc.into()];
        for param in &self.param {
            layout.push(param.clone());
        }
        layout
    }
    pub fn new(opcode: OPCODE, param: &Vec<u8>) -> Instruction {
        let mut ins = Instruction::from(opcode);

        /* if - is the right amount of params supplied */
        if ins.size - 1 > param.len() as u16 {
            let name: u8 = ins.opc.into();
            panic!("Too few params for instruction with opcode {}. Expected {} params got {}.",
                   name, ins.size - 1, param.len());
        } else if ins.size - 1 < param.len() as u16 {
            let name: u8 = ins.opc.into();
            panic!("Too many params for instruction with opcode {}. Expected {} params got {}.",
                   name, ins.size - 1, param.len());
        }
        /* end if - is the right amount of params supplied */

        ins.param = param.clone();
        ins
    }
}
impl From<OPCODE> for Instruction {
    fn from(code: OPCODE) -> Self {
        match code {
            OPCODE::STA_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles:  3},
            OPCODE::STA_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 4 },
            OPCODE::STA_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4 },
            OPCODE::STA_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 5 },
            OPCODE::STA_AY => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 5 },
            OPCODE::STA_IX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6 },
            OPCODE::STA_IY => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6 },

            OPCODE::STX_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 3 },
            OPCODE::STX_ZPY => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 4 },
            OPCODE::STX_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4 },

            OPCODE::STY_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 3 },
            OPCODE::STY_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 4 },
            OPCODE::STY_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4 },

            OPCODE::LDA_I => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 2 },
            OPCODE::LDA_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 3},
            OPCODE::LDA_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 4},
            OPCODE::LDA_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::LDA_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4 },
            OPCODE::LDA_AY => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::LDA_IX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::LDA_IY => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},

            OPCODE::LDX_I => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 4},
            OPCODE::LDX_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4 },
            OPCODE::LDX_AY => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::LDX_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::LDX_ZPY => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},

            OPCODE::LDY_I => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 4},
            OPCODE::LDY_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4 },
            OPCODE::LDY_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::LDY_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::LDY_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},

            OPCODE::CMP_I => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 2},
            OPCODE::CMP_ZP => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 3},
            OPCODE::CMP_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 4},
            OPCODE::CMP_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::CMP_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::CMP_AY => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::CMP_IX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 6},
            OPCODE::CMP_IY => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},

            OPCODE::CPX_I => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 2},
            OPCODE::CPX_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::CPX_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 3},

            OPCODE::CPY_I => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 2},
            OPCODE::CPY_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::CPY_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 3},

            OPCODE::ADC_I => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 2},
            OPCODE::ADC_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 3},
            OPCODE::ADC_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 4},
            OPCODE::ADC_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::ADC_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::ADC_AY => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::ADC_IX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::ADC_IY => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},

            OPCODE::AND_I => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 2},
            OPCODE::AND_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 2},
            OPCODE::AND_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 3},
            OPCODE::AND_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::AND_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::AND_AY => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::AND_IX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::AND_IY => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},

            OPCODE::ASL_ACC  => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::ASL_ZP  => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},
            OPCODE::ASL_ZPX  => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::ASL_A  => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 6},
            OPCODE::ASL_AX  => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 7},

            OPCODE::BIT_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 3},
            OPCODE::BIT_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},

            OPCODE::BPL => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::BMI => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::BVC => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::BVS => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::BCC => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::BCS => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::BNE => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::BEQ => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},

            OPCODE::BRK => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 7},

            OPCODE::DEC_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},
            OPCODE::DEC_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::DEC_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 6},
            OPCODE::DEC_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 7},

            OPCODE::EOR_I => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 2},
            OPCODE::EOR_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 3},
            OPCODE::EOR_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 4},
            OPCODE::EOR_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::EOR_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::EOR_AY => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::EOR_IX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::EOR_IY => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},

            OPCODE::CLC => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::SEC => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::CLI => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::SEI => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::CLV => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::CLD => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::SED => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},

            OPCODE::INC_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},
            OPCODE::INC_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::INC_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 6},
            OPCODE::INC_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 7},

            OPCODE::JMP_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 3},
            OPCODE::JMP_I => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 5},

            OPCODE::JSR => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 6},

            OPCODE::LSR_ACC => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::LSR_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},
            OPCODE::LSR_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::LSR_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 6},
            OPCODE::LSR_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 7},

            OPCODE::NOP => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},

            OPCODE::ORA_I => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 2},
            OPCODE::ORA_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 2},
            OPCODE::ORA_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 3},
            OPCODE::ORA_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::ORA_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::ORA_AY => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::ORA_IX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::ORA_IY => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},

            OPCODE::TAX => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::TXA => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::DEX => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::INX => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::TAY => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::TYA => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::DEY => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::INY => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},

            OPCODE::ROL_ACC => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::ROL_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},
            OPCODE::ROL_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::ROL_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 6},
            OPCODE::ROL_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 7},

            OPCODE::ROR_ACC => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::ROR_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},
            OPCODE::ROR_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::ROR_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 6},
            OPCODE::ROR_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 7},

            OPCODE::RTI => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 6},
            OPCODE::RTS => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 6},

            OPCODE::SBC_I => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 2},
            OPCODE::SBC_ZP => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 3},
            OPCODE::SBC_ZPX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 4},
            OPCODE::SBC_A => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::SBC_AX => Instruction { opc: code, param: Vec::new(), size: 3, cycles: 4},
            OPCODE::SBC_IX => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 6},
            OPCODE::SBC_IY => Instruction { opc: code, param: Vec::new(), size: 2, cycles: 5},

            OPCODE::TXS => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::TSX => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 2},
            OPCODE::PHA => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 3},
            OPCODE::PLA => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 4},
            OPCODE::PHP => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 3},
            OPCODE::PLP => Instruction { opc: code, param: Vec::new(), size: 1, cycles: 4},
            _ => panic!("Not implemented!")
        }
    }
}
impl TryFrom<u8> for Instruction {
    type Error = ();
    fn try_from(code: u8) -> Result<Self, Self::Error> {
        match code {
            x if x == OPCODE::STA_ZP as u8 => Ok(Instruction { opc: OPCODE::STA_ZP, param: Vec::new(), size: 2, cycles: 3 }),
            x if x == OPCODE::STA_ZPX as u8 => Ok(Instruction { opc: OPCODE::STA_ZPX, param: Vec::new(), size: 2, cycles: 4 }),
            x if x == OPCODE::STA_A as u8 => Ok(Instruction { opc: OPCODE::STA_A, param: Vec::new(), size: 3, cycles: 4 }),
            x if x == OPCODE::STA_AX as u8 => Ok(Instruction { opc: OPCODE::STA_AX, param: Vec::new(), size: 3, cycles: 5 }),
            x if x == OPCODE::STA_AY as u8 => Ok(Instruction { opc: OPCODE::STA_AY, param: Vec::new(), size: 3, cycles: 5 }),
            x if x == OPCODE::STA_IX as u8 => Ok(Instruction { opc: OPCODE::STA_IX, param: Vec::new(), size: 2, cycles: 6 }),
            x if x == OPCODE::STA_IY as u8 => Ok(Instruction { opc: OPCODE::STA_IY, param: Vec::new(), size: 2, cycles: 6 }),

            x if x == OPCODE::STX_ZP as u8 => Ok(Instruction { opc: OPCODE::STX_ZP, param: Vec::new(), size: 2, cycles: 3 }),
            x if x == OPCODE::STX_ZPY as u8 => Ok(Instruction { opc: OPCODE::STX_ZPY, param: Vec::new(), size: 2, cycles: 4 }),
            x if  x == OPCODE::STX_A as u8 => Ok(Instruction { opc: OPCODE::STX_A, param: Vec::new(), size: 3, cycles: 4 }),

            x if x == OPCODE::STY_ZP as u8 => Ok(Instruction { opc: OPCODE::STY_ZP, param:Vec::new(), size: 2, cycles: 3 }),
            x if  x == OPCODE::STY_ZPX as u8 => Ok(Instruction { opc: OPCODE::STY_ZPX, param:Vec::new(), size: 2, cycles: 4 }),
            x if  x == OPCODE::STY_A as u8 => Ok(Instruction { opc: OPCODE::STY_A, param:Vec::new(), size: 3, cycles: 4}),

            x if x == OPCODE::LDA_I as u8 => Ok(Instruction { opc: OPCODE::LDA_I, param: Vec::new(), size: 2, cycles: 2 }),
            x if x == OPCODE::LDA_ZP as u8 => Ok(Instruction { opc: OPCODE::LDA_ZP, param: Vec::new(), size: 2, cycles: 3 }),
            x if x == OPCODE::LDA_ZPX as u8 => Ok(Instruction { opc: OPCODE::LDA_ZPX, param: Vec::new(), size: 2, cycles: 4 }),
            x if x == OPCODE::LDA_A as u8 => Ok(Instruction { opc: OPCODE::LDA_A, param: Vec::new(), size: 3, cycles: 4 }),
            x if x == OPCODE::LDA_AX as u8 => Ok(Instruction { opc: OPCODE::LDA_AX, param: Vec::new(), size: 3, cycles: 4 }),
            x if x == OPCODE::LDA_AY as u8 => Ok(Instruction { opc: OPCODE::LDA_AY, param: Vec::new(), size: 3, cycles: 4 }),
            x if x == OPCODE::LDA_IX as u8 => Ok(Instruction { opc: OPCODE::LDA_IX, param: Vec::new(), size: 2, cycles: 6 }),
            x if x == OPCODE::LDA_IY as u8 => Ok(Instruction { opc: OPCODE::LDA_IY, param: Vec::new(), size: 2, cycles: 5 }),

            x if x == OPCODE::LDX_I as u8 => Ok(Instruction { opc: OPCODE::LDX_I, param: Vec::new(), size: 2, cycles: 2 }),
            x if x == OPCODE::LDX_A as u8 => Ok(Instruction { opc: OPCODE::LDX_A, param: Vec::new(), size: 3, cycles: 4 }),
            x if x == OPCODE::LDX_AY as u8 => Ok(Instruction { opc: OPCODE::LDX_AY, param: Vec::new(), size: 3, cycles: 4 }),
            x if x == OPCODE::LDX_ZP as u8 => Ok(Instruction { opc: OPCODE::LDX_ZP, param: Vec::new(), size: 2, cycles: 3 }),
            x if x == OPCODE::LDX_ZPY as u8 => Ok(Instruction { opc: OPCODE::LDX_ZPY, param: Vec::new(), size: 2, cycles: 4 }),

            x if x == OPCODE::LDY_I as u8 => Ok(Instruction { opc: OPCODE::LDY_I, param: Vec::new(), size: 2, cycles: 2 }),
            x if x == OPCODE::LDY_A as u8 => Ok(Instruction { opc: OPCODE::LDY_A, param: Vec::new(), size: 3, cycles: 4 }),
            x if x == OPCODE::LDY_AX as u8 => Ok(Instruction { opc: OPCODE::LDY_AX, param: Vec::new(), size: 3, cycles: 4 }),
            x if x == OPCODE::LDY_ZP as u8 => Ok(Instruction { opc: OPCODE::LDY_ZP, param: Vec::new(), size: 2, cycles: 3 }),
            x if x == OPCODE::LDY_ZPX as u8 => Ok(Instruction { opc: OPCODE::LDY_ZPX, param: Vec::new(), size: 2, cycles: 4 }),

            x if x == OPCODE::CMP_I as u8 => Ok(Instruction { opc: OPCODE::CMP_I, param: Vec::new(), size: 2, cycles: 2 }),
            x if x == OPCODE::CMP_ZP as u8 => Ok(Instruction { opc: OPCODE::CMP_ZP, param: Vec::new(), size: 2, cycles: 3 }),
            x if x == OPCODE::CMP_ZPX as u8 => Ok(Instruction { opc: OPCODE::CMP_ZPX, param: Vec::new(), size: 2, cycles: 4 }),
            x if x == OPCODE::CMP_A as u8 => Ok(Instruction { opc: OPCODE::CMP_A, param: Vec::new(), size: 3, cycles: 4 }),
            x if x == OPCODE::CMP_AX as u8 => Ok(Instruction { opc: OPCODE::CMP_AX, param: Vec::new(), size: 3, cycles: 4 }),
            x if x == OPCODE::CMP_AY as u8 => Ok(Instruction { opc: OPCODE::CMP_AY, param: Vec::new(), size: 3, cycles: 3 }),
            x if x == OPCODE::CMP_IX as u8 => Ok(Instruction { opc: OPCODE::CMP_IX, param: Vec::new(), size: 2, cycles: 6 }),
            x if x == OPCODE::CMP_IY as u8 => Ok(Instruction { opc: OPCODE::CMP_IY, param: Vec::new(), size: 2, cycles: 5 }),

            x if x == OPCODE::CPX_I as u8 => Ok(Instruction { opc: OPCODE::CPX_I, param: Vec::new(), size: 2, cycles: 2 }),
            x if x == OPCODE::CPX_A as u8 => Ok(Instruction { opc: OPCODE::CPX_A, param: Vec::new(), size: 3, cycles: 4 }),
            x if x == OPCODE::CPX_ZP as u8 => Ok(Instruction { opc: OPCODE::CPX_ZP, param: Vec::new(), size: 2, cycles: 3 }),

            x if x == OPCODE::CPY_I as u8 => Ok(Instruction { opc: OPCODE::CPY_I, param: Vec::new(), size: 2, cycles: 2 }),
            x if x == OPCODE::CPY_A as u8 => Ok(Instruction { opc: OPCODE::CPY_A, param: Vec::new(), size: 3, cycles: 4 }),
            x if x == OPCODE::CPY_ZP as u8 => Ok(Instruction { opc: OPCODE::CPY_ZP, param: Vec::new(), size: 2, cycles: 3 }),

            x if x == OPCODE::ADC_I as u8 => Ok(Instruction { opc: OPCODE::ADC_I, param: Vec::new(), size: 2, cycles: 2}),
            x if x == OPCODE::ADC_ZP as u8 => Ok(Instruction { opc: OPCODE::ADC_ZP, param: Vec::new(), size: 2, cycles: 3}),
            x if x == OPCODE::ADC_ZPX as u8 => Ok(Instruction { opc: OPCODE::ADC_ZPX, param: Vec::new(), size: 2, cycles: 4}),
            x if x == OPCODE::ADC_A as u8 => Ok(Instruction { opc: OPCODE::ADC_A, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::ADC_AX as u8 => Ok(Instruction { opc: OPCODE::ADC_AX, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::ADC_AY as u8 => Ok(Instruction { opc: OPCODE::ADC_AY, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::ADC_IX as u8 => Ok(Instruction { opc: OPCODE::ADC_IX, param: Vec::new(), size: 2, cycles: 6}),
            x if x == OPCODE::ADC_IY as u8 => Ok(Instruction { opc: OPCODE::ADC_IY, param: Vec::new(), size: 2, cycles: 5}),

            x if x == OPCODE::AND_I as u8 => Ok(Instruction { opc: OPCODE::AND_I, param: Vec::new(), size: 2, cycles: 2}),
            x if x == OPCODE::AND_ZP as u8 => Ok(Instruction { opc: OPCODE::AND_ZP, param: Vec::new(), size: 2, cycles: 2}),
            x if x == OPCODE::AND_ZPX as u8 => Ok(Instruction { opc: OPCODE::AND_ZPX, param: Vec::new(), size: 2, cycles: 3}),
            x if x == OPCODE::AND_A as u8 => Ok(Instruction { opc: OPCODE::AND_A, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::AND_AX as u8 => Ok(Instruction { opc: OPCODE::AND_AX, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::AND_AY as u8 => Ok(Instruction { opc: OPCODE::AND_AY, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::AND_IX as u8 => Ok(Instruction { opc: OPCODE::AND_IX, param: Vec::new(), size: 2, cycles: 6}),
            x if x == OPCODE::AND_IY as u8 => Ok(Instruction { opc: OPCODE::AND_IY, param: Vec::new(), size: 2, cycles: 5}),

            x if x == OPCODE::ASL_ACC as u8 => Ok(Instruction { opc: OPCODE::ASL_ACC, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::ASL_ZP as u8 => Ok(Instruction { opc: OPCODE::ASL_ZP, param: Vec::new(), size: 2, cycles: 5}),
            x if x == OPCODE::ASL_ZPX as u8 => Ok(Instruction { opc: OPCODE::ASL_ZPX, param: Vec::new(), size: 2, cycles: 6}),
            x if x == OPCODE::ASL_A as u8 => Ok(Instruction { opc: OPCODE::ASL_A, param: Vec::new(), size: 3, cycles: 6}),
            x if x == OPCODE::ASL_AX as u8 => Ok(Instruction { opc: OPCODE::ASL_AX, param: Vec::new(), size: 3, cycles: 7}),

            x if x == OPCODE::BIT_ZP as u8 => Ok(Instruction { opc: OPCODE::BIT_ZP, param: Vec::new(), size: 2, cycles: 3}),
            x if x == OPCODE::BIT_A as u8 => Ok(Instruction { opc: OPCODE::BIT_A, param: Vec::new(), size: 3, cycles: 4}),

            x if x == OPCODE::BPL as u8 => Ok(Instruction { opc: OPCODE::BPL, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::BMI as u8 => Ok(Instruction { opc: OPCODE::BMI, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::BVC as u8 => Ok(Instruction { opc: OPCODE::BVC, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::BVS as u8 => Ok(Instruction { opc: OPCODE::BVS, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::BCC as u8 => Ok(Instruction { opc: OPCODE::BCC, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::BCS as u8 => Ok(Instruction { opc: OPCODE::BCS, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::BNE as u8 => Ok(Instruction { opc: OPCODE::BNE, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::BEQ as u8 => Ok(Instruction { opc: OPCODE::BEQ, param: Vec::new(), size: 1, cycles: 2}),

            x if x == OPCODE::BRK as u8 => Ok(Instruction { opc: OPCODE::BRK, param: Vec::new(), size: 1, cycles: 7}),

            x if x == OPCODE::DEC_ZP as u8 => Ok(Instruction { opc: OPCODE::DEC_ZP, param: Vec::new(), size: 2, cycles: 5}),
            x if x == OPCODE::DEC_ZPX as u8 => Ok(Instruction { opc: OPCODE::DEC_ZPX, param: Vec::new(), size: 2, cycles: 6}),
            x if x == OPCODE::DEC_A as u8 => Ok(Instruction { opc: OPCODE::DEC_A, param: Vec::new(), size: 3, cycles: 6}),
            x if x == OPCODE::DEC_AX as u8 => Ok(Instruction { opc: OPCODE::DEC_AX, param: Vec::new(), size: 3, cycles: 7}),

            x if x == OPCODE::EOR_I as u8 => Ok(Instruction { opc: OPCODE::EOR_I, param: Vec::new(), size: 2, cycles: 2}),
            x if x == OPCODE::EOR_ZP as u8 => Ok(Instruction { opc: OPCODE::EOR_ZP, param: Vec::new(), size: 2, cycles: 3}),
            x if x == OPCODE::EOR_ZPX as u8 => Ok(Instruction { opc: OPCODE::EOR_ZPX, param: Vec::new(), size: 2, cycles: 4}),
            x if x == OPCODE::EOR_A as u8 => Ok(Instruction { opc: OPCODE::EOR_A, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::EOR_AX as u8 => Ok(Instruction { opc: OPCODE::EOR_AX, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::EOR_AY as u8 => Ok(Instruction { opc: OPCODE::EOR_AY, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::EOR_IX as u8 => Ok(Instruction { opc: OPCODE::EOR_IX, param: Vec::new(), size: 2, cycles: 6}),
            x if x == OPCODE::EOR_IY as u8 => Ok(Instruction { opc: OPCODE::EOR_IY, param: Vec::new(), size: 2, cycles: 5}),

            x if x == OPCODE::CLC as u8 => Ok(Instruction { opc: OPCODE::CLC, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::SEC as u8 => Ok(Instruction { opc: OPCODE::SEC, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::CLI as u8 => Ok(Instruction { opc: OPCODE::CLI, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::SEI as u8 => Ok(Instruction { opc: OPCODE::SEI, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::CLV as u8 => Ok(Instruction { opc: OPCODE::CLV, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::CLD as u8 => Ok(Instruction { opc: OPCODE::CLD, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::SED as u8 => Ok(Instruction { opc: OPCODE::SED, param: Vec::new(), size: 1, cycles: 2}),

            x if x == OPCODE::INC_ZP as u8 => Ok(Instruction { opc: OPCODE::INC_ZP, param: Vec::new(), size: 2, cycles: 5}),
            x if x == OPCODE::INC_ZPX as u8 => Ok(Instruction { opc: OPCODE::INC_ZPX, param: Vec::new(), size: 2, cycles: 6}),
            x if x == OPCODE::INC_A as u8 => Ok(Instruction { opc: OPCODE::INC_A, param: Vec::new(), size: 3, cycles: 6}),
            x if x == OPCODE::INC_AX as u8 => Ok(Instruction { opc: OPCODE::INC_AX, param: Vec::new(), size: 3, cycles: 7}),

            x if x == OPCODE::JMP_A as u8 => Ok(Instruction { opc: OPCODE::JMP_A, param: Vec::new(), size: 3, cycles: 3}),
            x if x == OPCODE::JMP_I as u8 => Ok(Instruction { opc: OPCODE::JMP_I, param: Vec::new(), size: 3, cycles: 5}),

            x if x == OPCODE::JSR as u8 => Ok(Instruction { opc: OPCODE::JSR, param: Vec::new(), size: 3, cycles: 6}),

            x if x == OPCODE::LSR_ACC as u8 => Ok(Instruction { opc: OPCODE::LSR_ACC, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::LSR_ZP as u8 => Ok(Instruction { opc: OPCODE::LSR_ZP, param: Vec::new(), size: 2, cycles: 5}),
            x if x == OPCODE::LSR_ZPX as u8 => Ok(Instruction { opc: OPCODE::LSR_ZPX, param: Vec::new(), size: 2, cycles: 6}),
            x if x == OPCODE::LSR_A as u8 => Ok(Instruction { opc: OPCODE::LSR_A, param: Vec::new(), size: 3, cycles: 6}),
            x if x == OPCODE::LSR_AX as u8 => Ok(Instruction { opc: OPCODE::LSR_AX, param: Vec::new(), size: 3, cycles: 7}),

            x if x == OPCODE::NOP as u8 => Ok(Instruction { opc: OPCODE::NOP, param: Vec::new(), size: 1, cycles: 2}),

            x if x == OPCODE::ORA_I as u8 => Ok(Instruction { opc: OPCODE::ORA_I, param: Vec::new(), size: 2, cycles: 2}),
            x if x == OPCODE::ORA_ZP as u8 => Ok(Instruction { opc: OPCODE::ORA_ZP, param: Vec::new(), size: 2, cycles: 2}),
            x if x == OPCODE::ORA_ZPX as u8 => Ok(Instruction { opc: OPCODE::ORA_ZPX, param: Vec::new(), size: 2, cycles: 3}),
            x if x == OPCODE::ORA_A as u8 => Ok(Instruction { opc: OPCODE::ORA_A, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::ORA_AX as u8 => Ok(Instruction { opc: OPCODE::ORA_AX, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::ORA_AY as u8 => Ok(Instruction { opc: OPCODE::ORA_AY, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::ORA_IX as u8 => Ok(Instruction { opc: OPCODE::ORA_IX, param: Vec::new(), size: 2, cycles: 6}),
            x if x == OPCODE::ORA_IY as u8 => Ok(Instruction { opc: OPCODE::ORA_IY, param: Vec::new(), size: 2, cycles: 5}),

            x if x == OPCODE::TAX as u8 => Ok(Instruction { opc: OPCODE::TAX, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::TXA as u8 => Ok(Instruction { opc: OPCODE::TXA, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::DEX as u8 => Ok(Instruction { opc: OPCODE::DEX, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::INX as u8 => Ok(Instruction { opc: OPCODE::INX, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::TAY as u8 => Ok(Instruction { opc: OPCODE::TAY, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::TYA as u8 => Ok(Instruction { opc: OPCODE::TYA, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::DEY as u8 => Ok(Instruction { opc: OPCODE::DEY, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::INY as u8 => Ok(Instruction { opc: OPCODE::INY, param: Vec::new(), size: 1, cycles: 2}),

            x if x == OPCODE::ROL_ACC as u8 => Ok(Instruction { opc: OPCODE::ROL_ACC, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::ROL_ZP as u8 => Ok(Instruction { opc: OPCODE::ROL_ZP, param: Vec::new(), size: 2, cycles: 5}),
            x if x == OPCODE::ROL_ZPX as u8 => Ok(Instruction { opc: OPCODE::ROL_ZPX, param: Vec::new(), size: 2, cycles: 6}),
            x if x == OPCODE::ROL_A as u8 => Ok(Instruction { opc: OPCODE::ROL_A, param: Vec::new(), size: 3, cycles: 6}),
            x if x == OPCODE::ROL_AX as u8 => Ok(Instruction { opc: OPCODE::ROL_AX, param: Vec::new(), size: 3, cycles: 7}),

            x if x == OPCODE::ROR_ACC as u8 => Ok(Instruction { opc: OPCODE::ROR_ACC, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::ROR_ZP as u8 => Ok(Instruction { opc: OPCODE::ROR_ZP, param: Vec::new(), size: 2, cycles: 5}),
            x if x == OPCODE::ROR_ZPX as u8 => Ok(Instruction { opc: OPCODE::ROR_ZPX, param: Vec::new(), size: 2, cycles: 6}),
            x if x == OPCODE::ROR_A as u8 => Ok(Instruction { opc: OPCODE::ROR_A, param: Vec::new(), size: 3, cycles: 6}),
            x if x == OPCODE::ROR_AX as u8 => Ok(Instruction { opc: OPCODE::ROR_AX, param: Vec::new(), size: 3, cycles: 7}),

            x if x == OPCODE::RTI as u8 => Ok(Instruction { opc: OPCODE::RTI, param: Vec::new(), size: 1, cycles: 6}),
            x if x == OPCODE::RTS as u8 => Ok(Instruction { opc: OPCODE::RTS, param: Vec::new(), size: 1, cycles: 6}),

            x if x == OPCODE::SBC_I as u8 => Ok(Instruction { opc: OPCODE::SBC_I, param: Vec::new(), size: 2, cycles: 2}),
            x if x == OPCODE::SBC_ZP as u8 => Ok(Instruction { opc: OPCODE::SBC_ZP, param: Vec::new(), size: 2, cycles: 3}),
            x if x == OPCODE::SBC_ZPX as u8 => Ok(Instruction { opc: OPCODE::SBC_ZPX, param: Vec::new(), size: 2, cycles: 4}),
            x if x == OPCODE::SBC_A as u8 => Ok(Instruction { opc: OPCODE::SBC_A, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::SBC_AX as u8 => Ok(Instruction { opc: OPCODE::SBC_AX, param: Vec::new(), size: 3, cycles: 4}),
            x if x == OPCODE::SBC_IX as u8 => Ok(Instruction { opc: OPCODE::SBC_IX, param: Vec::new(), size: 2, cycles: 6}),
            x if x == OPCODE::SBC_IY as u8 => Ok(Instruction { opc: OPCODE::SBC_IY, param: Vec::new(), size: 2, cycles: 5}),

            x if x == OPCODE::TXS as u8 => Ok(Instruction { opc: OPCODE::TXS, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::TSX as u8 => Ok(Instruction { opc: OPCODE::TSX, param: Vec::new(), size: 1, cycles: 2}),
            x if x == OPCODE::PHA as u8 => Ok(Instruction { opc: OPCODE::PHA, param: Vec::new(), size: 1, cycles: 3}),
            x if x == OPCODE::PLA as u8 => Ok(Instruction { opc: OPCODE::PLA, param: Vec::new(), size: 1, cycles: 4}),
            x if x == OPCODE::PHP as u8 => Ok(Instruction { opc: OPCODE::PHP, param: Vec::new(), size: 1, cycles: 3}),
            x if x == OPCODE::PLP as u8 => Ok(Instruction { opc: OPCODE::PLP, param: Vec::new(), size: 1, cycles: 4}),
            _ => Err(())
        }
    }
}