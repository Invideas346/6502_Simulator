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
            _ => { Err(()) }
        }
    }
}