use crate::Cpu;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone)]
pub enum OpCodeType {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    /// Illegal; Not a real thing in the actual NES. Anything not implemented is illegal
    XXX,
}

impl OpCodeType {
    pub fn executable(&self) -> fn(&mut Cpu) -> u8 {
        use self::OpCodeType::*;
        use super::operations::*;

        match *self {
            ADC => adc_fn,
            AND => and_fn,
            ASL => asl_fn,
            BCC => bcc_fn,
            BCS => bcs_fn,
            BEQ => beq_fn,
            BIT => bit_fn,
            BMI => bmi_fn,
            BNE => bne_fn,
            BPL => bpl_fn,
            BRK => brk_fn,
            BVC => bvc_fn,
            BVS => bvs_fn,
            CLC => clc_fn,
            CLD => cld_fn,
            CLI => cli_fn,
            CLV => clv_fn,
            CMP => cmp_fn,
            CPX => cpx_fn,
            CPY => cpy_fn,
            DEC => dec_fn,
            DEX => dex_fn,
            DEY => dey_fn,
            EOR => eor_fn,
            INC => inc_fn,
            INX => inx_fn,
            INY => iny_fn,
            JMP => jmp_fn,
            JSR => jsr_fn,
            LDA => lda_fn,
            LDX => ldx_fn,
            LDY => ldy_fn,
            LSR => lsr_fn,
            NOP => nop_fn,
            ORA => ora_fn,
            PHA => pha_fn,
            PHP => php_fn,
            PLA => pla_fn,
            PLP => plp_fn,
            ROL => rol_fn,
            ROR => ror_fn,
            RTI => rti_fn,
            RTS => rts_fn,
            SBC => sbc_fn,
            SEC => sec_fn,
            SED => sed_fn,
            SEI => sei_fn,
            STA => sta_fn,
            STX => stx_fn,
            STY => sty_fn,
            TAX => tax_fn,
            TAY => tay_fn,
            TSX => tsx_fn,
            TXA => txa_fn,
            TXS => txs_fn,
            TYA => tya_fn,
            XXX => xxx_fn,
        }
    }
}
