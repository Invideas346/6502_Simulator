extern crate core;

use crate::cpu::CPU;
use crate::instructions::{Instruction, OPCODE};
use crate::memory::Memory;
use crate::OPCODE::LDA_A;

mod cpu;
mod instructions;
mod memory;
mod register;
mod tests;

fn main() {
    let mut cpu: CPU = CPU::new(0, 0, 0, 0);
    let mut mem: Memory = Memory::new();
    mem.push_back_ins(Instruction::new(LDA_A, &vec![0x12, 0x23]));
    mem.physical_mem[0x2312] = 0xFF;
    cpu.execute(&mut mem);
}
