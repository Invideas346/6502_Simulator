#[cfg(test)]
mod tests {
    use crate::memory::ZP_S;
    use crate::OPCODE::{
        ADC_A, ADC_AX, ADC_AY, ADC_I, ADC_IX, ADC_IY, ADC_ZP, ADC_ZPX, AND_A, AND_AX, AND_AY,
        AND_I, AND_IX, AND_IY, AND_ZP, AND_ZPX, ASL_A, ASL_ACC, ASL_AX, ASL_ZP, ASL_ZPX, BCC, BCS,
        BEQ, BIT_A, BIT_ZP, BMI, BNE, BPL, BVC, BVS, CMP_I, CPX_A, CPX_I, CPX_ZP, CPY_A, CPY_I,
        CPY_ZP, DEC_A, DEC_AX, DEC_ZP, DEC_ZPX, DEX, INC_A, INC_AX, INC_ZP, INC_ZPX, INX, JMP_A,
        JMP_I, LDA_AX, LDA_AY, LDA_I, LDA_IX, LDA_IY, LDA_ZP, LDA_ZPX, LDX_A, LDX_AY, LDX_I,
        LDX_ZP, LDX_ZPY, LDY_A, LDY_AX, LDY_I, LDY_ZP, LDY_ZPX, STA_A, STA_AX, STA_AY, STA_IX,
        STA_IY, STA_ZP, STA_ZPX, STX_A, STX_ZP, STX_ZPY, STY_A, STY_ZP, STY_ZPX,
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
        mem.write_byte(&(0x51 as u16), &0x14);
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x1412 as u16)), 0x13);
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
        mem.physical_mem[(0x22) as usize] = 0x10;
        mem.physical_mem[(0x23) as usize] = 0xFF;
        mem.physical_mem[(0xFF10) as usize] = 0xAA;
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xAA);
    }
    #[test]
    fn test_lda_iy() {
        let mut cpu: CPU = CPU::new(0, 0, 0x40, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(LDA_IY, &vec![0x50]));
        mem.physical_mem[(0x0050) as usize] = 0x20;
        mem.physical_mem[(0x0051) as usize] = 0x10;
        mem.physical_mem[(0x1060) as usize] = 0xFF;
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

    #[test]
    fn test_adc_i() {
        let mut cpu: CPU = CPU::new(0x10, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ADC_I, &vec![0x20]));
        cpu.execute(&mut mem);

        assert_eq!(cpu.a().value, 0x30);
    }

    #[test]
    fn test_adc_zp() {
        let mut cpu: CPU = CPU::new(0x10, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ADC_ZP, &vec![0x20]));
        mem.physical_mem[(ZP_S + 0x20) as usize] = 0x32;
        cpu.execute(&mut mem);

        assert_eq!(cpu.a().value, 0x42);
    }

    #[test]
    fn test_adc_zpx() {
        let mut cpu: CPU = CPU::new(0x10, 0x15, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ADC_ZPX, &vec![0x20]));
        mem.physical_mem[(ZP_S + 0x20 + 0x15) as usize] = 0x32;
        cpu.execute(&mut mem);

        assert_eq!(cpu.a().value, 0x42);
    }

    #[test]
    fn test_adc_a() {
        let mut cpu: CPU = CPU::new(0x10, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ADC_A, &vec![0x20, 0x40]));
        mem.physical_mem[(0x4020) as usize] = 0x32;
        cpu.execute(&mut mem);

        assert_eq!(cpu.a().value, 0x42);
    }

    #[test]
    fn test_adc_ax() {
        let mut cpu: CPU = CPU::new(0x10, 0x20, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ADC_AX, &vec![0x20, 0x40]));
        mem.physical_mem[(0x4020 + 0x20) as usize] = 0x32;
        cpu.execute(&mut mem);

        assert_eq!(cpu.a().value, 0x42);
    }

    #[test]
    fn test_adc_ay() {
        let mut cpu: CPU = CPU::new(0x10, 0, 0x20, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ADC_AY, &vec![0x20, 0x40]));
        mem.physical_mem[(0x4020 + 0x20) as usize] = 0x32;
        cpu.execute(&mut mem);

        assert_eq!(cpu.a().value, 0x42);
    }

    #[test]
    fn test_adc_ix() {
        let mut cpu: CPU = CPU::new(0x10, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ADC_IX, &vec![0x40]));
        mem.physical_mem[(0x50) as usize] = 0x20;
        mem.physical_mem[(0x51) as usize] = 0x32;
        mem.physical_mem[(0x3220) as usize] = 0x32;
        cpu.execute(&mut mem);

        assert_eq!(cpu.a().value, 0x42);
    }

    #[test]
    fn test_adc_iy() {
        let mut cpu: CPU = CPU::new(0x10, 0, 0x10, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ADC_IY, &vec![0x40]));
        mem.physical_mem[(0x40) as usize] = 0x11;
        mem.physical_mem[(0x41) as usize] = 0x51;
        mem.physical_mem[(0x5121) as usize] = 0xAA;
        cpu.execute(&mut mem);

        assert_eq!(cpu.a().value, 0xBA);
    }

    #[test]
    fn test_and_i() {
        let mut cpu: CPU = CPU::new(0xFF, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(AND_I, &vec![0xF0]));
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xF0);
    }

    #[test]
    fn test_and_zp() {
        let mut cpu: CPU = CPU::new(0xFF, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(AND_ZP, &vec![0x20]));
        mem.write_byte(&(ZP_S + 0x20), &(0xF0));
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xF0);
    }

    #[test]
    fn test_and_zpx() {
        let mut cpu: CPU = CPU::new(0xFF, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(AND_ZPX, &vec![0x20]));
        mem.write_byte(&(ZP_S + 0x30), &(0xF0));
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xF0);
    }

    #[test]
    fn test_and_a() {
        let mut cpu: CPU = CPU::new(0xFF, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(AND_A, &vec![0x20, 0x40]));
        mem.write_byte(&(0x4020), &(0xF0));
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xF0);
    }

    #[test]
    fn test_and_ax() {
        let mut cpu: CPU = CPU::new(0xFF, 0xAA, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(AND_AX, &vec![0x20, 0x40]));
        mem.write_byte(&(0x4020 + 0xAA), &(0xF0));
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xF0);
    }

    #[test]
    fn test_and_ay() {
        let mut cpu: CPU = CPU::new(0xFF, 0, 0xAA, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(AND_AY, &vec![0x20, 0x40]));
        mem.write_byte(&(0x4020 + 0xAA), &(0xF0));
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xF0);
    }

    #[test]
    fn test_and_ix() {
        let mut cpu: CPU = CPU::new(0xFF, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(AND_IX, &vec![0x20]));
        mem.write_byte(&(0x30), &(0x10));
        mem.write_byte(&(0x31), &(0x20));
        mem.write_byte(&(0x2010), &(0xF0));
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xF0);
    }

    #[test]
    fn test_and_iy() {
        let mut cpu: CPU = CPU::new(0xFF, 0, 0x10, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(AND_IY, &vec![0x20]));
        mem.write_byte(&(0x20), &(0x10));
        mem.write_byte(&(0x21), &(0x20));
        mem.write_byte(&(0x2010 + 0x10), &(0xF0));
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xF0);
    }

    #[test]
    fn test_asl_acc() {
        let mut cpu: CPU = CPU::new(0xFF, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ASL_ACC, &vec![]));
        cpu.execute(&mut mem);
        assert_eq!(cpu.a().value, 0xFE);
    }

    #[test]
    fn test_asl_zp() {
        let mut cpu: CPU = CPU::new(0xFF, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ASL_ZP, &vec![0x20]));
        mem.write_byte(&(ZP_S + 0x20), &(0xFF));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(ZP_S + 0x20)), 0xFE);
    }

    #[test]
    fn test_asl_zpx() {
        let mut cpu: CPU = CPU::new(0xFF, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ASL_ZPX, &vec![0x20]));
        mem.write_byte(&(ZP_S + 0x20 + 0x10), &(0xFF));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(ZP_S + 0x20 + 0x10)), 0xFE);
    }

    #[test]
    fn test_asl_a() {
        let mut cpu: CPU = CPU::new(0xFF, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ASL_A, &vec![0x20, 0x60]));
        mem.write_byte(&(0x6020), &(0x01));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x6020)), 0x02);
    }

    #[test]
    fn test_asl_ax() {
        let mut cpu: CPU = CPU::new(0xFF, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ASL_AX, &vec![0x20, 0x60]));
        mem.write_byte(&(0x6020 + 0x10), &(0xFF));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x6020 + 0x10)), 0xFE);
    }

    #[test]
    fn test_bit_zp() {
        let mut cpu: CPU = CPU::new(0xFF, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(BIT_ZP, &vec![0x60]));
        mem.write_byte(&(ZP_S + 0x60), &(0b10100000));
        cpu.execute(&mut mem);
        assert_eq!(*cpu.z_flag(), false);
        assert_eq!(*cpu.n_flag(), true);
        assert_eq!(*cpu.v_flag(), false);
    }

    #[test]
    fn test_bit_a() {
        let mut cpu: CPU = CPU::new(0xFF, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(BIT_A, &vec![0x60, 0x20]));
        mem.write_byte(&(0x2060), &(0b10100000));
        cpu.execute(&mut mem);
        assert_eq!(*cpu.z_flag(), false);
        assert_eq!(*cpu.n_flag(), true);
        assert_eq!(*cpu.v_flag(), false);
    }

    #[test]
    fn test_bpl() {
        let mut cpu: CPU = CPU::new(0x10, 0x20, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(CMP_I, &vec![0x11]));
        mem.push_back_ins(Instruction::new(BPL, &vec![0x1])); // 1 - Byte offset because INX is 1 Byte lone (jump over it)
        mem.push_back_ins(Instruction::new(INX, &vec![]));
        mem.push_back_ins(Instruction::new(DEX, &vec![]));
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0x1F);
    }

    #[test]
    fn test_bmi() {
        let mut cpu: CPU = CPU::new(0x10, 0x20, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(CMP_I, &vec![0x09]));
        mem.push_back_ins(Instruction::new(BMI, &vec![0x1])); // 1 - Byte offset because INX is 1 Byte lone (jump over it)
        mem.push_back_ins(Instruction::new(INX, &vec![]));
        mem.push_back_ins(Instruction::new(DEX, &vec![]));
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0x1F);
    }

    #[test]
    fn test_bvc() {
        let mut cpu: CPU = CPU::new(0xFF, 0x20, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ADC_I, &vec![0x00]));
        mem.push_back_ins(Instruction::new(BVC, &vec![0x1])); // 1 - Byte offset because INX is 1 Byte lone (jump over it)
        mem.push_back_ins(Instruction::new(INX, &vec![]));
        mem.push_back_ins(Instruction::new(DEX, &vec![]));
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0x1F);
    }

    #[test]
    fn test_bvs() {
        let mut cpu: CPU = CPU::new(0xFF, 0x20, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ADC_I, &vec![0x01]));
        mem.push_back_ins(Instruction::new(BVS, &vec![0x1])); // 1 - Byte offset because INX is 1 Byte lone (jump over it)
        mem.push_back_ins(Instruction::new(INX, &vec![]));
        mem.push_back_ins(Instruction::new(DEX, &vec![]));
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0x1F);
    }

    #[test]
    fn test_bcc() {
        let mut cpu: CPU = CPU::new(0xFF, 0x20, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ADC_I, &vec![0x00]));
        mem.push_back_ins(Instruction::new(BCC, &vec![0x1])); // 1 - Byte offset because INX is 1 Byte lone (jump over it)
        mem.push_back_ins(Instruction::new(INX, &vec![]));
        mem.push_back_ins(Instruction::new(DEX, &vec![]));
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0x1F);
    }

    #[test]
    fn test_bcs() {
        let mut cpu: CPU = CPU::new(0xFF, 0x20, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(ADC_I, &vec![0x01]));
        mem.push_back_ins(Instruction::new(BCS, &vec![0x1])); // 1 - Byte offset because INX is 1 Byte lone (jump over it)
        mem.push_back_ins(Instruction::new(INX, &vec![]));
        mem.push_back_ins(Instruction::new(DEX, &vec![]));
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0x1F);
    }

    #[test]
    fn test_bne() {
        let mut cpu: CPU = CPU::new(0xFF, 0x20, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(CMP_I, &vec![0x00]));
        mem.push_back_ins(Instruction::new(BNE, &vec![0x1])); // 1 - Byte offset because INX is 1 Byte lone (jump over it)
        mem.push_back_ins(Instruction::new(INX, &vec![]));
        mem.push_back_ins(Instruction::new(DEX, &vec![]));
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0x1F);
    }

    #[test]
    fn test_beq() {
        let mut cpu: CPU = CPU::new(0xFF, 0x20, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(CMP_I, &vec![0xFF]));
        mem.push_back_ins(Instruction::new(BEQ, &vec![0x1])); // 1 - Byte offset because INX is 1 Byte lone (jump over it)
        mem.push_back_ins(Instruction::new(INX, &vec![]));
        mem.push_back_ins(Instruction::new(DEX, &vec![]));
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0x1F);
    }

    #[test]
    fn test_dec_zp() {
        let mut cpu: CPU = CPU::new(0, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(DEC_ZP, &vec![0x10]));
        mem.write_byte(&(ZP_S + 0x10), &(0x14));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(ZP_S + 0x10)), 0x13);
    }

    #[test]
    fn test_dec_zpx() {
        let mut cpu: CPU = CPU::new(0, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(DEC_ZPX, &vec![0x10]));
        mem.write_byte(&(ZP_S + 0x20), &(0x14));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(ZP_S + 0x20)), 0x13);
    }

    #[test]
    fn test_dec_a() {
        let mut cpu: CPU = CPU::new(0, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(DEC_A, &vec![0x10, 0x40]));
        mem.write_byte(&(0x4010), &(0x14));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x4010)), 0x13);
    }

    #[test]
    fn test_dec_ax() {
        let mut cpu: CPU = CPU::new(0, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(DEC_AX, &vec![0x10, 0x40]));
        mem.write_byte(&(0x4010 + 0x10), &(0x14));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x4010 + 0x10)), 0x13);
    }

    #[test]
    fn test_inc_zp() {
        let mut cpu: CPU = CPU::new(0, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(INC_ZP, &vec![0x10]));
        mem.write_byte(&(ZP_S + 0x10), &(0x14));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(ZP_S + 0x10)), 0x15);
    }

    #[test]
    fn test_inc_zpx() {
        let mut cpu: CPU = CPU::new(0, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(INC_ZPX, &vec![0x10]));
        mem.write_byte(&(ZP_S + 0x20), &(0x14));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(ZP_S + 0x20)), 0x15);
    }

    #[test]
    fn test_inc_a() {
        let mut cpu: CPU = CPU::new(0, 0, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(INC_A, &vec![0x10, 0x40]));
        mem.write_byte(&(0x4010), &(0x14));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x4010)), 0x15);
    }

    #[test]
    fn test_inc_ax() {
        let mut cpu: CPU = CPU::new(0, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(INC_AX, &vec![0x10, 0x40]));
        mem.write_byte(&(0x4010 + 0x10), &(0x14));
        cpu.execute(&mut mem);
        assert_eq!(*mem.read_byte(&(0x4010 + 0x10)), 0x15);
    }

    #[test]
    fn test_jmp_a() {
        let mut cpu: CPU = CPU::new(0, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(JMP_A, &vec![0x05, 0x80]));
        mem.push_back_ins(Instruction::new(INX, &vec![]));
        mem.push_back_ins(Instruction::new(DEX, &vec![]));
        mem.push_back_ins(Instruction::new(INX, &vec![]));
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0x11);
    }

    #[test]
    fn test_jmp_i() {
        let mut cpu: CPU = CPU::new(0, 0x10, 0, 0);
        let mut mem: Memory = Memory::new();
        mem.push_back_ins(Instruction::new(JMP_I, &vec![0x00, 0x40]));
        mem.push_back_ins(Instruction::new(INX, &vec![]));
        mem.push_back_ins(Instruction::new(DEX, &vec![]));
        mem.push_back_ins(Instruction::new(INX, &vec![]));
        mem.write_byte(&(0x4000), &(0x05));
        mem.write_byte(&(0x4001), &(0x80));
        cpu.execute(&mut mem);
        cpu.execute(&mut mem);
        assert_eq!(cpu.x().value, 0x11);
    }
}
