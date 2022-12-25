use crate::Instruction;

/* Zero-page start and end address */
pub const ZP_S: u16 = 0x0000;
pub const ZP_E: u16 = 0x00FF;

/* Stack start and end address */
pub const STACK_S: u16 = 0x0100;
pub const STACK_E: u16 = 0x01FF;

/* Program-RAM start and end address */
pub const PROGRAM_RAM_S: u16 = 0x0200;
pub const PROGRAM_RAM_E: u16 = 0x3FFF;

/* Memory mapped io start and end address */
pub const MEMORY_MAPPED_IO_S: u16 = 0x4000;
pub const MEMORY_MAPPED_IO_E: u16 = 0x7FFF;

/* Program-ROM start and end address */
pub const PROGRAM_ROM_S: u16 = 0x8000;
pub const PROGRAM_ROM_E: u16 = 0xFFF9;

pub const VECTOR_ADDR_NMI_LOW: u16 = 0xFFFA;
pub const VECTOR_ADDR_NMI_HIGH: u16 = 0xFFFB;
pub const VECTOR_ADDR_RESET_LOW: u16 = 0xFFFC;
pub const VECTOR_ADDR_RESET_HIGH: u16 = 0xFFFD;
pub const VECTOR_ADDR_IRQ_BRK_LOW: u16 = 0xFFFD;
pub const VECTOR_ADDR_IRQ_BRK_HIGH: u16 = 0xFFFF;

pub struct Memory {
    pub physical_mem: [u8; u16::MAX as usize + 1],
    instruction_pos: u16,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            physical_mem: [0; u16::MAX as usize + 1],
            instruction_pos: PROGRAM_ROM_S,
        }
    }

    pub fn push_back_ins(&mut self, ins: Instruction) {
        let layout = ins.convert_to_mem_layout();
        for i in 0..*ins.size() {
            self.physical_mem[(i + self.instruction_pos) as usize] = layout[i as usize];
        }
        self.instruction_pos += ins.size();
    }

    pub fn read_byte(&self, addr: &u16) -> &u8 {
        &self.physical_mem[*addr as usize]
    }

    pub fn write_byte(&mut self, addr: &u16, value: &u8) {
        self.physical_mem[*addr as usize] = *value;
    }
}
