#[cfg(test)]
mod tests {
    use crate::memory::ZP_S;
    use crate::OPCODE::{
        CPX_A, CPX_I, CPX_ZP, CPY_A, CPY_I, CPY_ZP, LDA_AX, LDA_AY, LDA_I, LDA_IX, LDA_IY, LDA_ZP,
        LDA_ZPX, LDX_A, LDX_AY, LDX_I, LDX_ZP, LDX_ZPY, LDY_A, LDY_AX, LDY_I, LDY_ZP, LDY_ZPX,
        STA_A, STA_AX, STA_AY, STA_IX, STA_IY, STA_ZP, STA_ZPX, STX_A, STX_ZP, STX_ZPY, STY_A,
        STY_ZP, STY_ZPX,
    };
    use crate::{Instruction, Memory, CPU, LDA_A};

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
        mem.push_back_ins(Instruction::new(LDA_I, &vec![0x12]));
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0x12);
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

    #[test]
    fn test_ldx_i() {
        let mut cpu: CPU = CPU::new(0, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDX_I, &vec![0x12]));
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0x12);
    }
    #[test]
    fn test_ldx_a() {
        let mut cpu: CPU = CPU::new(0, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDX_A, &vec![0x10, 0x80]));
        mem.physical_mem[(0x8010) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0xFF);
    }
    #[test]
    fn test_ldx_ay() {
        let mut cpu: CPU = CPU::new(0, 0, 0x2, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDX_AY, &vec![0x10, 0x80]));
        mem.physical_mem[(0x8012) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0xFF);
    }
    #[test]
    fn test_ldx_zp() {
        let mut cpu: CPU = CPU::new(0, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDX_ZP, &vec![0x10]));
        mem.physical_mem[(ZP_S + 0x10) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0xFF);
    }
    #[test]
    fn test_ldx_zpy() {
        let mut cpu: CPU = CPU::new(0, 0, 0x1, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDX_ZPY, &vec![0x10]));
        mem.physical_mem[(ZP_S + 0x11) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0xFF);
    }

    #[test]
    fn test_ldy_i() {
        let mut cpu: CPU = CPU::new(0, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDY_I, &vec![0x12]));
        cpu.execute(&mut mem);
        assert_eq!(cpu.y().value, 0x12);
    }
    #[test]
    fn test_ldy_a() {
        let mut cpu: CPU = CPU::new(0, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDY_A, &vec![0x10, 0x80]));
        mem.physical_mem[(0x8010) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.y().value, 0xFF);
    }
    #[test]
    fn test_ldy_ax() {
        let mut cpu: CPU = CPU::new(0, 0x2, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDY_AX, &vec![0x10, 0x80]));
        mem.physical_mem[(0x8012) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.y().value, 0xFF);
    }
    #[test]
    fn test_ldy_zp() {
        let mut cpu: CPU = CPU::new(0, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDY_ZP, &vec![0x10]));
        mem.physical_mem[(ZP_S + 0x10) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.y().value, 0xFF);
    }
    #[test]
    fn test_ldy_zpx() {
        let mut cpu: CPU = CPU::new(0, 0x1, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDY_ZPX, &vec![0x10]));
        mem.physical_mem[(ZP_S + 0x11) as usize] = 0xFF;
        cpu.execute(&mut mem);
        assert_eq!(cpu.y().value, 0xFF);
    }

    #[test]
    fn test_cpx_i() {
        let mut cpu: CPU = CPU::new(0, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(CPX_I, &vec![0x10]));
        cpu.execute(&mut mem);

        /* test whether a equal can be detected */
        /* to detect a equals only the zero flag has to be true */
        assert_eq!(*cpu.n_flag(), false);
        assert_eq!(*cpu.z_flag(), true);
        assert_eq!(*cpu.c_flag(), true);
    }
    #[test]
    fn test_cpx_a() {
        let mut cpu: CPU = CPU::new(0, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(CPX_A, &vec![0x10, 0x80]));
        mem.physical_mem[(0x8010 as usize)] = 0x10;
        cpu.execute(&mut mem);

        assert_eq!(*cpu.n_flag(), false);
        assert_eq!(*cpu.z_flag(), true);
        assert_eq!(*cpu.c_flag(), true);
    }
    #[test]
    fn test_cpx_zp() {
        let mut cpu: CPU = CPU::new(0, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(CPX_ZP, &vec![0x20]));
        mem.physical_mem[(ZP_S + 0x20) as usize] = 0x10;
        cpu.execute(&mut mem);

        assert_eq!(*cpu.n_flag(), false);
        assert_eq!(*cpu.z_flag(), true);
        assert_eq!(*cpu.c_flag(), true);
    }

    #[test]
    fn test_cpy_i() {
        let mut cpu: CPU = CPU::new(0, 0, 0x10, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(CPY_I, &vec![0x10]));
        cpu.execute(&mut mem);

        assert_eq!(*cpu.n_flag(), false);
        assert_eq!(*cpu.z_flag(), true);
        assert_eq!(*cpu.c_flag(), true);
    }
    #[test]
    fn test_cpy_a() {
        let mut cpu: CPU = CPU::new(0, 0, 0x10, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(CPY_A, &vec![0x10, 0x80]));
        mem.physical_mem[(0x8010 as usize)] = 0x10;
        cpu.execute(&mut mem);

        assert_eq!(*cpu.n_flag(), false);
        assert_eq!(*cpu.z_flag(), true);
        assert_eq!(*cpu.c_flag(), true);
    }
    #[test]
    fn test_cpy_zp() {
        let mut cpu: CPU = CPU::new(0, 0, 0x10, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(CPY_ZP, &vec![0x20]));
        mem.physical_mem[(ZP_S + 0x20) as usize] = 0x10;
        cpu.execute(&mut mem);

        assert_eq!(*cpu.n_flag(), false);
        assert_eq!(*cpu.z_flag(), true);
        assert_eq!(*cpu.c_flag(), true);
    }
}
