use crate::memory::{Memory, PROGRAM_ROM_S, ZP_S};
use crate::register::Register;
use crate::{Instruction, OPCODE};

enum AddressingMode {
    ZEROPAGE,
    ZEROPAGEX,
    ZEROPAGEY,
    ABSOLUTE,
    ABSOLUTEX,
    ABSOLUTEY,
    INDIRECTX,
    INDIRECTY,
}

pub struct CPU {
    a: Register<u8>,
    y: Register<u8>,
    x: Register<u8>,
    ins: Register<u8>,

    n_flag: bool,
    v_flag: bool,
    b_flag: bool,
    d_flag: bool,
    i_flag: bool,
    z_flag: bool,
    c_flag: bool,

    program_counter: Register<u16>,
    clock_cycles_elapsed: u64,
}

impl CPU {
    fn inc_clock_cycles_via_ins(&mut self, ins: &Instruction) {
        self.clock_cycles_elapsed += (*ins.cycles()) as u64;
    }
    fn cmp_op(&mut self, reg_value: u8, value: u8) {
        if value == reg_value {
            self.n_flag = false;
            self.z_flag = true;
            self.c_flag = true;
        } else if value >= reg_value {
            self.z_flag = false;
            self.c_flag = false;
        } else if value <= reg_value {
            self.z_flag = false;
            self.c_flag = true;
        }
    }
    fn adc_calc_flags(&mut self, prev_value: &u8) {
        if self.a.value < *prev_value {
            self.c_flag = true;
        }
        if self.a.value == 0 {
            self.z_flag = true;
        }
        if self.a.value >= 0x80 {
            self.n_flag = true;
        }
        if self.c_flag ^ self.n_flag {
            self.v_flag = true;
        }
    }
    fn get_addr(&self, memory: &mut Memory, mode: AddressingMode) -> u16 {
        let mut addr: u16;
        match mode {
            AddressingMode::ZEROPAGE => {
                addr = *memory.read_byte(&(self.program_counter.value + 1)) as u16;
                addr += ZP_S;
            }
            AddressingMode::ZEROPAGEX => {
                addr = *memory.read_byte(&(self.program_counter.value + 1)) as u16;
                addr += ZP_S + self.x.value as u16
            }
            AddressingMode::ZEROPAGEY => {
                addr = *memory.read_byte(&(self.program_counter.value + 1)) as u16;
                addr += ZP_S + self.y.value as u16
            }
            AddressingMode::ABSOLUTE => {
                addr = (*memory.read_byte(&(self.program_counter.value + 1)) as u16)
                    + ((*memory.read_byte(&(self.program_counter.value + 2)) as u16) << 8);
            }
            AddressingMode::ABSOLUTEX => {
                addr = (*memory.read_byte(&(self.program_counter.value + 1)) as u16)
                    + ((*memory.read_byte(&(self.program_counter.value + 2)) as u16) << 8)
                    + (self.x.value as u16);
            }
            AddressingMode::ABSOLUTEY => {
                addr = (*memory.read_byte(&(self.program_counter.value + 1)) as u16)
                    + ((*memory.read_byte(&(self.program_counter.value + 2)) as u16) << 8)
                    + (self.y.value as u16);
            }
            AddressingMode::INDIRECTX => {
                let high_addr: u16 = *memory.read_byte(&(self.program_counter.value + 1)) as u16;
                let low_addr: u16 = *memory.read_byte(&(high_addr + self.x.value as u16)) as u16;
                addr = (high_addr << 8) + low_addr;
            }
            AddressingMode::INDIRECTY => {
                addr = ((*memory
                    .read_byte(&(*memory.read_byte(&(self.program_counter.value + 1)) as u16)))
                    as u16)
                    + (((*memory.read_byte(
                        &(((*memory.read_byte(&((self.program_counter.value + 1) as u16))) as u16)
                            + 1),
                    ) as u16)
                        << 8) as u16)
                    + (self.y.value as u16);
            }
        }
        addr
    }
    pub fn new(value_a: u8, value_x: u8, value_y: u8, ins: u8) -> CPU {
        CPU {
            a: Register::new(value_a),
            x: Register::new(value_x),
            y: Register::new(value_y),
            ins: Register::new(ins),

            n_flag: false,
            v_flag: false,
            b_flag: false,
            d_flag: false,
            i_flag: false,
            z_flag: false,
            c_flag: false,

            program_counter: Register::new(PROGRAM_ROM_S),
            clock_cycles_elapsed: 0,
        }
    }
    pub fn execute(&mut self, memory: &mut Memory) {
        /* obtain the instruction opcode */
        let opcode_addr = self.program_counter.value;
        let opcode = (*memory.read_byte(&opcode_addr)).clone();
        self.ins.value = opcode;
        let instruction = Instruction::try_from(opcode).unwrap();
        let first_opr = self.program_counter.value + 1;
        match instruction.opc() {
            OPCODE::STA_ZP => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGE);
                memory.write_byte(&addr, &self.a.value);
            }
            OPCODE::STA_ZPX => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGEX);
                memory.write_byte(&addr, &self.a.value);
            }
            OPCODE::STA_A => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTE);
                memory.write_byte(&addr, &self.a.value);
            }
            OPCODE::STA_AX => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTEX);
                memory.write_byte(&addr, &self.a.value);
            }
            OPCODE::STA_AY => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTEY);
                memory.write_byte(&addr, &self.a.value);
            }
            OPCODE::STA_IX => {
                let addr = self.get_addr(memory, AddressingMode::INDIRECTX);
                memory.write_byte(&addr, &self.a.value);
            }
            OPCODE::STA_IY => {
                let addr = self.get_addr(memory, AddressingMode::INDIRECTY);
                memory.write_byte(&addr, &self.a.value);
            }

            OPCODE::STX_ZP => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGE);
                memory.write_byte(&addr, &self.x.value);
            }
            OPCODE::STX_ZPY => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGEY);
                memory.write_byte(&addr, &self.x.value);
            }
            OPCODE::STX_A => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTE);
                memory.write_byte(&addr, &self.x.value);
            }

            OPCODE::STY_ZP => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGE);
                memory.write_byte(&(ZP_S + addr), &self.y.value);
            }
            OPCODE::STY_ZPX => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGEX);
                memory.write_byte(&addr, &self.y.value);
            }
            OPCODE::STY_A => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTE);
                memory.write_byte(&addr, &self.y.value);
            }

            OPCODE::LDA_I => {
                let value = memory.read_byte(&first_opr);
                self.a.value = *value;
            }
            OPCODE::LDA_ZP => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGE);
                let value = memory.read_byte(&addr);
                self.a.value = *value;
            }
            OPCODE::LDA_ZPX => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGEX);
                let value: u8 = *memory.read_byte(&addr);
                self.a.value = value;
            }
            OPCODE::LDA_A => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTE);
                let value = *memory.read_byte(&addr);
                self.a.value = value;
            }
            /* TODO: consider page crosses and add a extra cpu cycle to the execution */
            OPCODE::LDA_AX => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTEX);
                let value = *memory.read_byte(&addr);
                self.a.value = value;
            }
            /* TODO: consider page crosses and add a extra cpu cycle to the execution */
            OPCODE::LDA_AY => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTEY);
                let value = *memory.read_byte(&addr);
                self.a.value = value;
            }
            OPCODE::LDA_IX => {
                let addr = self.get_addr(memory, AddressingMode::INDIRECTX);
                let value = memory.read_byte(&addr);
                self.a.value = *value;
            }
            /* TODO: consider page crosses and add a extra cpu cycle to the execution (not totally sure why this only occurs if y reg is used)*/
            OPCODE::LDA_IY => {
                let addr = self.get_addr(memory, AddressingMode::INDIRECTY);
                let value = memory.read_byte(&addr);
                self.a.value = *value;
            }

            OPCODE::LDX_I => {
                let value = memory.read_byte(&first_opr);
                self.x.value = *value;
            }
            OPCODE::LDX_A => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTE);
                let value = *memory.read_byte(&addr);
                self.x.value = value;
            }
            /* TODO: consider page crosses and add a extra cpu cycle to the execution */
            OPCODE::LDX_AY => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTEY);
                let value = *memory.read_byte(&addr);
                self.x.value = value;
            }
            OPCODE::LDX_ZP => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGE);
                let value = memory.read_byte(&addr);
                self.x.value = *value;
            }
            OPCODE::LDX_ZPY => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGEY);
                let value: u8 = *memory.read_byte(&addr);
                self.x.value = value;
            }

            OPCODE::LDY_I => {
                let value = memory.read_byte(&first_opr);
                self.y.value = *value;
            }
            OPCODE::LDY_A => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTE);
                let value = *memory.read_byte(&addr);
                self.y.value = value;
            }
            /* TODO: consider page crosses and add a extra cpu cycle to the execution */
            OPCODE::LDY_AX => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTEX);
                let value = *memory.read_byte(&addr);
                self.y.value = value;
            }
            OPCODE::LDY_ZP => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGE);
                let value = memory.read_byte(&addr);
                self.y.value = *value;
            }
            OPCODE::LDY_ZPX => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGEX);
                let value: u8 = *memory.read_byte(&addr);
                self.y.value = value;
            }

            OPCODE::CPX_I => {
                self.cmp_op(self.x.value, *memory.read_byte(&first_opr));
            }
            OPCODE::CPX_A => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTE);
                self.cmp_op(self.x.value, *memory.read_byte(&addr));
            }
            OPCODE::CPX_ZP => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGE);
                self.cmp_op(self.x.value, *memory.read_byte(&addr));
            }

            OPCODE::CPY_I => {
                self.cmp_op(self.y.value, *memory.read_byte(&first_opr));
            }
            OPCODE::CPY_A => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTE);
                self.cmp_op(self.y.value, *memory.read_byte(&addr));
            }
            OPCODE::CPY_ZP => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGE);
                self.cmp_op(self.y.value, *memory.read_byte(&addr));
            }

            OPCODE::CMP_I => {
                self.cmp_op(self.a.value, *memory.read_byte(&first_opr));
            }
            OPCODE::CMP_ZP => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGE);
                self.cmp_op(self.a.value, *memory.read_byte(&addr));
            }
            OPCODE::CMP_ZPX => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGEX);
                let value = memory.read_byte(&addr);
                self.cmp_op(self.a.value, *value);
            }
            OPCODE::CMP_A => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTE);
                self.cmp_op(self.a.value, *memory.read_byte(&addr));
            }
            OPCODE::CMP_AX => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTEX);
                self.cmp_op(self.a.value, *memory.read_byte(&addr));
            }
            OPCODE::CMP_AY => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTEY);
                self.cmp_op(self.a.value, *memory.read_byte(&addr));
            }
            OPCODE::CMP_IX => {
                let addr = self.get_addr(memory, AddressingMode::INDIRECTX);
                let value = memory.read_byte(&addr);
                self.cmp_op(self.a.value, *value);
            }
            OPCODE::CMP_IY => {
                let addr = self.get_addr(memory, AddressingMode::INDIRECTY);
                let value = memory.read_byte(&addr);
                self.cmp_op(self.a.value, *value);
            }

            OPCODE::ADC_I => {
                let value = *memory.read_byte(&first_opr);
                let prev_value = self.a.value;
                self.a.value += value;
                self.adc_calc_flags(&prev_value)
            }
            OPCODE::ADC_ZP => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGE);
                let value = *memory.read_byte(&addr);
                let prev_value = self.a.value;
                self.a.value += value;
                self.adc_calc_flags(&prev_value)
            }
            OPCODE::ADC_ZPX => {
                let addr = self.get_addr(memory, AddressingMode::ZEROPAGEX);
                let value = *memory.read_byte(&addr);
                let prev_value = self.a.value;
                self.a.value += value;
                self.adc_calc_flags(&prev_value)
            }
            OPCODE::ADC_A => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTE);
                let value = *memory.read_byte(&addr);
                let prev_value = self.a.value;
                self.a.value += value;
                self.adc_calc_flags(&prev_value)
            }
            OPCODE::ADC_AX => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTEX);
                let value = *memory.read_byte(&addr);
                let prev_value = self.a.value;
                self.a.value += value;
                self.adc_calc_flags(&prev_value)
            }
            OPCODE::ADC_AY => {
                let addr = self.get_addr(memory, AddressingMode::ABSOLUTEY);
                let value = *memory.read_byte(&addr);
                let prev_value = self.a.value;
                self.a.value += value;
                self.adc_calc_flags(&prev_value)
            }
            OPCODE::ADC_IX => {
                let addr = self.get_addr(memory, AddressingMode::INDIRECTX);
                let value = *memory.read_byte(&addr);
                let prev_value = self.a.value;
                self.a.value += value;
                self.adc_calc_flags(&prev_value)
            }
            OPCODE::ADC_IY => {
                let addr = self.get_addr(memory, AddressingMode::INDIRECTY);
                let value = *memory.read_byte(&addr);
                let prev_value = self.a.value;
                self.a.value += value;
                self.adc_calc_flags(&prev_value)
            }

            _ => {
                let opcode: u8 = (*instruction.opc()).into();
                panic!("Instruction not handled by simulation. OPCODE: {}", opcode);
            }
        }
        self.program_counter.value += instruction.size();
        self.inc_clock_cycles_via_ins(&instruction);
    }

    /*
        This methods are only supposed for unit tests.
        They are needed to set up the register inorder to effectively test the cpu executing instructions correctly.
    */
    pub fn a(&mut self) -> &Register<u8> {
        &self.a
    }
    pub fn x(&mut self) -> &Register<u8> {
        &self.x
    }
    pub fn y(&mut self) -> &Register<u8> {
        &self.y
    }
    pub fn ins(&mut self) -> &Register<u8> {
        &self.ins
    }
    pub fn n_flag(&mut self) -> &bool {
        &self.n_flag
    }
    pub fn c_flag(&mut self) -> &bool {
        &self.c_flag
    }
    pub fn i_flag(&mut self) -> &bool {
        &self.i_flag
    }
    pub fn z_flag(&mut self) -> &bool {
        &self.z_flag
    }
    pub fn d_flag(&mut self) -> &bool {
        &self.d_flag
    }
    pub fn v_flag(&mut self) -> &bool {
        &self.v_flag
    }
}
