#[cfg(test)]
mod tests {
    use crate::{CPU, Instruction, LDA_A, Memory};
    use crate::memory::ZP_S;
    use crate::OPCODE::{LDA_AX, LDA_AY, LDA_IX, LDA_IY, LDA_ZP, LDA_ZPX, STA_A, STA_AX, STA_AY, STA_IX, STA_IY, STA_ZP, STA_ZPX, STX_A, STX_ZP, STX_ZPY, STY_A, STY_ZP, STY_ZPX};

    #[test]
    fn test_sta_zp() {
        let mut cpu: CPU = CPU::new(0x11, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(STA_ZP, &vec![0x10]));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(ZP_S + 0x10 as u16)), 0x11);
    }
    #[test]
    fn test_sta_zpx() {
        let mut cpu: CPU = CPU::new(0x11, 0x50, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(STA_ZPX, &vec![0x10]));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(ZP_S + 0x60 as u16)), 0x11);
    }
    #[test]
    fn test_sta_a() {
        let mut cpu: CPU = CPU::new(0x13, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(STA_A, &vec![0x10, 0x40]));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x4010 as u16)), 0x13);
    }
    #[test]
    fn test_sta_ax() {
        let mut cpu: CPU = CPU::new(0x13, 0x02, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(STA_AX, &vec![0x10, 0x40]));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x4012 as u16)), 0x13);
    }
    #[test]
    fn test_sta_ay() {
        let mut cpu: CPU = CPU::new(0x13, 0, 0x02, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(STA_AY, &vec![0x10, 0x40]));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x4012 as u16)), 0x13);
    }
    #[test]
    fn test_sta_ix() {
        let mut cpu: CPU = CPU::new(0x13, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(STA_IX, &vec![0x40]));
        mem.write_byte(&(0x50 as u16), &0x12);
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x4012 as u16)), 0x13);
    }
    #[test]
    fn test_sta_iy() {
        let mut cpu: CPU = CPU::new(0x13, 0, 0x10, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(STA_IY, &vec![0x40]));
        mem.write_byte(&(0x40 as u16), &0x40);
        mem.write_byte(&(0x41 as u16), &0x10);
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x1050 as u16)), 0x13);
    }

    #[test]
    fn test_stx_zp() {
        let mut cpu: CPU = CPU::new(0, 0x11, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(STX_ZP, &vec![0x10]));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(ZP_S + 0x10 as u16)), 0x11);
    }
    #[test]
    fn test_stx_zpy() {
        let mut cpu: CPU = CPU::new(0, 0x11, 0x50, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(STX_ZPY, &vec![0x10]));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(ZP_S + 0x60 as u16)), 0x11);
    }
    #[test]
    fn test_stx_a() {
        let mut cpu: CPU = CPU::new(0, 0x13, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(STX_A, &vec![0x10, 0x40]));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x4010 as u16)), 0x13);
    }

    #[test]
    fn test_sty_zp() {
        let mut cpu: CPU = CPU::new(0, 0, 0x11, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(STY_ZP, &vec![0x10]));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(ZP_S + 0x10 as u16)), 0x11);
    }
    #[test]
    fn test_sty_zpx() {
        let mut cpu: CPU = CPU::new(0, 0x50, 0x11, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(STY_ZPX, &vec![0x10]));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(ZP_S + 0x60 as u16)), 0x11);
    }
    #[test]
    fn test_sty_a() {
        let mut cpu: CPU = CPU::new(0, 0, 0x13, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(STY_A, &vec![0x10, 0x40]));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x4010 as u16)), 0x13);
    }


    #[test]
    fn test_lda_i() {
        let mut cpu: CPU = CPU::new(0, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDA_A, &vec![0x12, 0x23]));
        mem.physical_mem[0x2312] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xFF);
    }
    #[test]
    fn test_lda_zp() {
        let mut cpu: CPU = CPU::new(0, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDA_ZP, &vec![0x10]));
        mem.physical_mem[(ZP_S + 0x10) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xFF);
    }
    #[test]
    fn test_lda_zpx() {
        let mut cpu: CPU = CPU::new(0, 0x1, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDA_ZPX, &vec![0x10]));
        mem.physical_mem[(ZP_S + 0x11) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xFF);
    }
    #[test]
    fn test_lda_a() {
        let mut cpu: CPU = CPU::new(0, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDA_A, &vec![0x10, 0x80]));
        mem.physical_mem[(0x8010) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xFF);
    }
    #[test]
    fn test_lda_ax() {
        let mut cpu: CPU = CPU::new(0, 0x8, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDA_AX, &vec![0x10, 0x80]));
        mem.physical_mem[(0x8018) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xFF);
    }
    #[test]
    fn test_lda_ay() {
        let mut cpu: CPU = CPU::new(0, 0, 0x2, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDA_AY, &vec![0x10, 0x80]));
        mem.physical_mem[(0x8012) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xFF);
    }
    #[test]
    fn test_lda_ix() {
        let mut cpu: CPU = CPU::new(0, 0x2, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDA_IX, &vec![0x20]));
        mem.physical_mem[(0x0022) as usize] = 0x10;
        mem.physical_mem[(0x2010) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xFF);
    }
    #[test]
    fn test_lda_iy() {
        let mut cpu: CPU = CPU::new(0, 0, 0x40, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDA_IY, &vec![0x50]));
        mem.physical_mem[(0x0050 + ZP_S) as usize] = 0x10;
        mem.physical_mem[(0x0051 + ZP_S) as usize] = 0x10;
        mem.physical_mem[(0x1050) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xFF);
    }
}