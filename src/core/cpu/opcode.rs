//! A module that provide opcode's infomations
//!
//! TODO:
//! - [ ] Clean OPCODE_TABLE with function: put together same code.

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// The enum that represent mnemonics 
#[derive(Debug, PartialEq)]
pub enum Mnemonic {
    Adc, And, Asl, Bcc, Bcs, Beq, Bit, Bmi, Bne, Bpl, Brk, Bvc, Bvs, Clc,
    Cld, Cli, Clv, Cmp, Cpx, Cpy, Dec, Dex, Dey, Eor, Inc, Inx, Iny, Jmp,
    Jsr, Lda, Ldx, Ldy, Lsr, Nop, Ora, Pha, Php, Pla, Plp, Rol, Ror, Rti,
    Rts, Sbc, Sec, Sed, Sei, Sta, Stx, Sty, Tax, Tay, Tsx, Txa, Txs, Tya,
}

/// The enum that represent addressing mode
#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    Accumulator, Absolute, AbsoluteX, AbsoluteY,
    Immediate,   Implied,  Indirect,  IndirectX,
    IndirectY,   Relative, ZeroPage,  ZeroPageX,
    ZeroPageY,
}

/// An type that is specialized u8 type to represent opcode.
///
/// This type is generally used to specify that some function's argument is
/// required to be interger that represent opcode.
pub type Opcode = u8;

// This is used in OPCODE_TABLE to associate some infomation with opcode.
#[derive(Debug, PartialEq)]
pub struct OpcodeInfo {
    pub byte: u8,
    pub cycle: u8,
    pub name: Mnemonic,
    pub mode: AddressingMode,
}

impl OpcodeInfo {
    fn new(byte: u8, cycle: u8, name: Mnemonic, mode: AddressingMode) -> Self {
        Self { byte, cycle, name, mode }
    }
}

// Due to struct can't contain static variable, this static variable is
// declared as global and private.
pub static OPCODE_TABLE: Lazy<HashMap<Opcode, OpcodeInfo>> = Lazy::new(|| {
    HashMap::from([
        (0x69, OpcodeInfo::new(2, 2, Mnemonic::Adc, AddressingMode::Immediate)),
        (0x65, OpcodeInfo::new(2, 3, Mnemonic::Adc, AddressingMode::ZeroPage )),
        (0x75, OpcodeInfo::new(2, 4, Mnemonic::Adc, AddressingMode::ZeroPageX)),
        (0x6D, OpcodeInfo::new(3, 4, Mnemonic::Adc, AddressingMode::Absolute )),
        (0x7D, OpcodeInfo::new(3, 4, Mnemonic::Adc, AddressingMode::AbsoluteX)),
        (0x79, OpcodeInfo::new(3, 4, Mnemonic::Adc, AddressingMode::AbsoluteY)),
        (0x61, OpcodeInfo::new(2, 6, Mnemonic::Adc, AddressingMode::IndirectX)),
        (0x71, OpcodeInfo::new(2, 5, Mnemonic::Adc, AddressingMode::IndirectY)),

        (0x29, OpcodeInfo::new(2, 2, Mnemonic::And, AddressingMode::Immediate)),
        (0x25, OpcodeInfo::new(2, 3, Mnemonic::And, AddressingMode::ZeroPage )),
        (0x35, OpcodeInfo::new(2, 4, Mnemonic::And, AddressingMode::ZeroPageX)),
        (0x2D, OpcodeInfo::new(3, 4, Mnemonic::And, AddressingMode::Absolute )),
        (0x3D, OpcodeInfo::new(3, 4, Mnemonic::And, AddressingMode::AbsoluteX)),
        (0x39, OpcodeInfo::new(3, 4, Mnemonic::And, AddressingMode::AbsoluteY)),
        (0x21, OpcodeInfo::new(2, 6, Mnemonic::And, AddressingMode::IndirectX)),
        (0x31, OpcodeInfo::new(2, 5, Mnemonic::And, AddressingMode::IndirectY)),

        (0x0A, OpcodeInfo::new(1, 2, Mnemonic::Asl, AddressingMode::Accumulator)),
        (0x06, OpcodeInfo::new(2, 5, Mnemonic::Asl, AddressingMode::ZeroPage   )),
        (0x16, OpcodeInfo::new(2, 6, Mnemonic::Asl, AddressingMode::ZeroPageX  )),
        (0x0E, OpcodeInfo::new(3, 6, Mnemonic::Asl, AddressingMode::Absolute   )),
        (0x1E, OpcodeInfo::new(3, 7, Mnemonic::Asl, AddressingMode::AbsoluteX  )),

        (0x90, OpcodeInfo::new(2, 2, Mnemonic::Bcc, AddressingMode::Relative)),

        (0xB0, OpcodeInfo::new(2, 2, Mnemonic::Bcs, AddressingMode::Relative)),

        (0xF0, OpcodeInfo::new(2, 2, Mnemonic::Beq, AddressingMode::Relative)),

        (0x24, OpcodeInfo::new(2, 3, Mnemonic::Bit, AddressingMode::ZeroPage)),
        (0x2C, OpcodeInfo::new(3, 4, Mnemonic::Bit, AddressingMode::Absolute)),

        (0x30, OpcodeInfo::new(2, 2, Mnemonic::Bmi, AddressingMode::Relative)),

        (0xD0, OpcodeInfo::new(2, 2, Mnemonic::Bne, AddressingMode::Relative)),

        (0x10, OpcodeInfo::new(2, 2, Mnemonic::Bpl, AddressingMode::Relative)),

        (0x00, OpcodeInfo::new(1, 7, Mnemonic::Brk, AddressingMode::Implied)),

        (0x50, OpcodeInfo::new(2, 2, Mnemonic::Bvc, AddressingMode::Relative)),

        (0x70, OpcodeInfo::new(2, 2, Mnemonic::Bvs, AddressingMode::Relative)),

        (0x18, OpcodeInfo::new(1, 2, Mnemonic::Clc, AddressingMode::Implied)),

        (0xD8, OpcodeInfo::new(1, 2, Mnemonic::Cld, AddressingMode::Implied)),

        (0x58, OpcodeInfo::new(1, 2, Mnemonic::Cli, AddressingMode::Implied)),

        (0xB8, OpcodeInfo::new(1, 2, Mnemonic::Clv, AddressingMode::Implied)),

        (0xC9, OpcodeInfo::new(2, 2, Mnemonic::Cmp, AddressingMode::Immediate)),
        (0xC5, OpcodeInfo::new(2, 3, Mnemonic::Cmp, AddressingMode::ZeroPage )),
        (0xD5, OpcodeInfo::new(2, 4, Mnemonic::Cmp, AddressingMode::ZeroPageX)),
        (0xCD, OpcodeInfo::new(3, 4, Mnemonic::Cmp, AddressingMode::Absolute )),
        (0xDD, OpcodeInfo::new(3, 4, Mnemonic::Cmp, AddressingMode::AbsoluteX)),
        (0xD9, OpcodeInfo::new(3, 4, Mnemonic::Cmp, AddressingMode::AbsoluteY)),
        (0xC1, OpcodeInfo::new(2, 6, Mnemonic::Cmp, AddressingMode::IndirectX)),
        (0xD1, OpcodeInfo::new(2, 5, Mnemonic::Cmp, AddressingMode::IndirectY)),

        (0xE0, OpcodeInfo::new(2, 2, Mnemonic::Cpx, AddressingMode::Immediate)),
        (0xE4, OpcodeInfo::new(2, 3, Mnemonic::Cpx, AddressingMode::ZeroPage )),
        (0xEC, OpcodeInfo::new(3, 4, Mnemonic::Cpx, AddressingMode::Absolute )),

        (0xC0, OpcodeInfo::new(2, 2, Mnemonic::Cpy, AddressingMode::Immediate)),
        (0xC4, OpcodeInfo::new(2, 3, Mnemonic::Cpy, AddressingMode::ZeroPage )),
        (0xCC, OpcodeInfo::new(3, 4, Mnemonic::Cpy, AddressingMode::Absolute )),

        (0xC6, OpcodeInfo::new(2, 5, Mnemonic::Dec, AddressingMode::ZeroPage )),
        (0xD6, OpcodeInfo::new(2, 6, Mnemonic::Dec, AddressingMode::ZeroPageX)),
        (0xCE, OpcodeInfo::new(3, 6, Mnemonic::Dec, AddressingMode::Absolute )),
        (0xDE, OpcodeInfo::new(3, 7, Mnemonic::Dec, AddressingMode::AbsoluteX)),

        (0xCA, OpcodeInfo::new(1, 2, Mnemonic::Dex, AddressingMode::Implied)),

        (0x88, OpcodeInfo::new(1, 2, Mnemonic::Dey, AddressingMode::Implied)),

        (0x49, OpcodeInfo::new(2, 2, Mnemonic::Eor, AddressingMode::Immediate)),
        (0x45, OpcodeInfo::new(2, 3, Mnemonic::Eor, AddressingMode::ZeroPage )),
        (0x55, OpcodeInfo::new(2, 4, Mnemonic::Eor, AddressingMode::ZeroPageX)),
        (0x4D, OpcodeInfo::new(3, 4, Mnemonic::Eor, AddressingMode::Absolute )),
        (0x5D, OpcodeInfo::new(3, 4, Mnemonic::Eor, AddressingMode::AbsoluteX)),
        (0x59, OpcodeInfo::new(3, 4, Mnemonic::Eor, AddressingMode::AbsoluteY)),
        (0x41, OpcodeInfo::new(2, 6, Mnemonic::Eor, AddressingMode::IndirectX)),
        (0x51, OpcodeInfo::new(2, 5, Mnemonic::Eor, AddressingMode::IndirectY)),

        (0xE6, OpcodeInfo::new(2, 5, Mnemonic::Inc, AddressingMode::ZeroPage )),
        (0xF6, OpcodeInfo::new(2, 6, Mnemonic::Inc, AddressingMode::ZeroPageX)),
        (0xEE, OpcodeInfo::new(3, 6, Mnemonic::Inc, AddressingMode::Absolute )),
        (0xFE, OpcodeInfo::new(3, 7, Mnemonic::Inc, AddressingMode::AbsoluteX)),

        (0xE8, OpcodeInfo::new(1, 2, Mnemonic::Inx, AddressingMode::Implied)),

        (0xC8, OpcodeInfo::new(1, 2, Mnemonic::Iny, AddressingMode::Implied)),

        (0x4C, OpcodeInfo::new(3, 3, Mnemonic::Jmp, AddressingMode::Absolute)),
        (0x6C, OpcodeInfo::new(3, 5, Mnemonic::Jmp, AddressingMode::Indirect)),

        (0x20, OpcodeInfo::new(3, 6, Mnemonic::Jsr, AddressingMode::Absolute)),

        (0xA9, OpcodeInfo::new(2, 2, Mnemonic::Lda, AddressingMode::Immediate)),
        (0xA5, OpcodeInfo::new(2, 3, Mnemonic::Lda, AddressingMode::ZeroPage )),
        (0xB5, OpcodeInfo::new(2, 4, Mnemonic::Lda, AddressingMode::ZeroPageX)),
        (0xAD, OpcodeInfo::new(3, 4, Mnemonic::Lda, AddressingMode::Absolute )),
        (0xBD, OpcodeInfo::new(3, 4, Mnemonic::Lda, AddressingMode::AbsoluteX)),
        (0xB9, OpcodeInfo::new(3, 4, Mnemonic::Lda, AddressingMode::AbsoluteY)),
        (0xA1, OpcodeInfo::new(2, 6, Mnemonic::Lda, AddressingMode::IndirectX)),
        (0xB1, OpcodeInfo::new(2, 5, Mnemonic::Lda, AddressingMode::IndirectY)),

        (0xA2, OpcodeInfo::new(2, 2, Mnemonic::Ldx, AddressingMode::Immediate)),
        (0xA6, OpcodeInfo::new(2, 3, Mnemonic::Ldx, AddressingMode::ZeroPage )),
        (0xB6, OpcodeInfo::new(2, 4, Mnemonic::Ldx, AddressingMode::ZeroPageY)),
        (0xAE, OpcodeInfo::new(3, 4, Mnemonic::Ldx, AddressingMode::Absolute )),
        (0xBE, OpcodeInfo::new(3, 4, Mnemonic::Ldx, AddressingMode::AbsoluteY)),

        (0xA0, OpcodeInfo::new(2, 2, Mnemonic::Ldy, AddressingMode::Immediate)),
        (0xA4, OpcodeInfo::new(2, 3, Mnemonic::Ldy, AddressingMode::ZeroPage )),
        (0xB4, OpcodeInfo::new(2, 4, Mnemonic::Ldy, AddressingMode::ZeroPageX)),
        (0xAC, OpcodeInfo::new(3, 4, Mnemonic::Ldy, AddressingMode::Absolute )),
        (0xBC, OpcodeInfo::new(3, 4, Mnemonic::Ldy, AddressingMode::AbsoluteX)),

        (0x4A, OpcodeInfo::new(1, 2, Mnemonic::Lsr, AddressingMode::Accumulator)),
        (0x46, OpcodeInfo::new(2, 5, Mnemonic::Lsr, AddressingMode::ZeroPage   )),
        (0x56, OpcodeInfo::new(2, 6, Mnemonic::Lsr, AddressingMode::ZeroPageX  )),
        (0x4E, OpcodeInfo::new(3, 6, Mnemonic::Lsr, AddressingMode::Absolute   )),
        (0x5E, OpcodeInfo::new(3, 7, Mnemonic::Lsr, AddressingMode::AbsoluteX  )),

        (0xEA, OpcodeInfo::new(1, 2, Mnemonic::Nop, AddressingMode::Implied)),

        (0x09, OpcodeInfo::new(2, 2, Mnemonic::Ora, AddressingMode::Immediate)),
        (0x05, OpcodeInfo::new(2, 3, Mnemonic::Ora, AddressingMode::ZeroPage )),
        (0x15, OpcodeInfo::new(2, 4, Mnemonic::Ora, AddressingMode::ZeroPageX)),
        (0x0D, OpcodeInfo::new(3, 4, Mnemonic::Ora, AddressingMode::Absolute )),
        (0x1D, OpcodeInfo::new(3, 4, Mnemonic::Ora, AddressingMode::AbsoluteX)),
        (0x19, OpcodeInfo::new(3, 4, Mnemonic::Ora, AddressingMode::AbsoluteY)),
        (0x01, OpcodeInfo::new(2, 6, Mnemonic::Ora, AddressingMode::IndirectX)),
        (0x11, OpcodeInfo::new(2, 5, Mnemonic::Ora, AddressingMode::IndirectY)),

        (0x48, OpcodeInfo::new(1, 3, Mnemonic::Pha, AddressingMode::Implied)),

        (0x08, OpcodeInfo::new(1, 3, Mnemonic::Php, AddressingMode::Implied)),

        (0x68, OpcodeInfo::new(1, 4, Mnemonic::Pla, AddressingMode::Implied)),

        (0x28, OpcodeInfo::new(1, 4, Mnemonic::Plp, AddressingMode::Implied)),

        (0x2A, OpcodeInfo::new(1, 2, Mnemonic::Rol, AddressingMode::Accumulator)),
        (0x26, OpcodeInfo::new(2, 5, Mnemonic::Rol, AddressingMode::ZeroPage   )),
        (0x36, OpcodeInfo::new(2, 6, Mnemonic::Rol, AddressingMode::ZeroPageX  )),
        (0x2E, OpcodeInfo::new(3, 6, Mnemonic::Rol, AddressingMode::Absolute   )),
        (0x3E, OpcodeInfo::new(3, 7, Mnemonic::Rol, AddressingMode::AbsoluteX  )),

        (0x6A, OpcodeInfo::new(1, 2, Mnemonic::Ror, AddressingMode::Accumulator)),
        (0x66, OpcodeInfo::new(2, 5, Mnemonic::Ror, AddressingMode::ZeroPage   )),
        (0x76, OpcodeInfo::new(2, 6, Mnemonic::Ror, AddressingMode::ZeroPageX  )),
        (0x6E, OpcodeInfo::new(3, 6, Mnemonic::Ror, AddressingMode::Absolute   )),
        (0x7E, OpcodeInfo::new(3, 7, Mnemonic::Ror, AddressingMode::AbsoluteX  )),

        (0x40, OpcodeInfo::new(1, 6, Mnemonic::Rti, AddressingMode::Implied)),

        (0x60, OpcodeInfo::new(1, 6, Mnemonic::Rts, AddressingMode::Implied)),

        (0xE9, OpcodeInfo::new(2, 2, Mnemonic::Sbc, AddressingMode::Immediate)),
        (0xE5, OpcodeInfo::new(2, 3, Mnemonic::Sbc, AddressingMode::ZeroPage )),
        (0xF5, OpcodeInfo::new(2, 4, Mnemonic::Sbc, AddressingMode::ZeroPageX)),
        (0xED, OpcodeInfo::new(3, 4, Mnemonic::Sbc, AddressingMode::Absolute )),
        (0xFD, OpcodeInfo::new(3, 4, Mnemonic::Sbc, AddressingMode::AbsoluteX)),
        (0xF9, OpcodeInfo::new(3, 4, Mnemonic::Sbc, AddressingMode::AbsoluteY)),
        (0xE1, OpcodeInfo::new(2, 6, Mnemonic::Sbc, AddressingMode::IndirectX)),
        (0xF1, OpcodeInfo::new(2, 5, Mnemonic::Sbc, AddressingMode::IndirectY)),

        (0x38, OpcodeInfo::new(1, 2, Mnemonic::Sec, AddressingMode::Implied)),

        (0xF8, OpcodeInfo::new(1, 2, Mnemonic::Sed, AddressingMode::Implied)),

        (0x78, OpcodeInfo::new(1, 2, Mnemonic::Sei, AddressingMode::Implied)),

        (0x85, OpcodeInfo::new(2, 3, Mnemonic::Sta, AddressingMode::ZeroPage )),
        (0x95, OpcodeInfo::new(2, 4, Mnemonic::Sta, AddressingMode::ZeroPageX)),
        (0x8D, OpcodeInfo::new(3, 4, Mnemonic::Sta, AddressingMode::Absolute )),
        (0x9D, OpcodeInfo::new(3, 4, Mnemonic::Sta, AddressingMode::AbsoluteX)),
        (0x99, OpcodeInfo::new(3, 4, Mnemonic::Sta, AddressingMode::AbsoluteY)),
        (0x81, OpcodeInfo::new(2, 6, Mnemonic::Sta, AddressingMode::IndirectX)),
        (0x91, OpcodeInfo::new(2, 5, Mnemonic::Sta, AddressingMode::IndirectY)),

        (0x86, OpcodeInfo::new(2, 3, Mnemonic::Stx, AddressingMode::ZeroPage )),
        (0x96, OpcodeInfo::new(2, 4, Mnemonic::Stx, AddressingMode::ZeroPageY)),
        (0x8E, OpcodeInfo::new(3, 4, Mnemonic::Stx, AddressingMode::Absolute )),

        (0x84, OpcodeInfo::new(2, 3, Mnemonic::Sty, AddressingMode::ZeroPage )),
        (0x94, OpcodeInfo::new(2, 4, Mnemonic::Sty, AddressingMode::ZeroPageX)),
        (0x8C, OpcodeInfo::new(3, 4, Mnemonic::Sty, AddressingMode::Absolute )),

        (0xAA, OpcodeInfo::new(1, 2, Mnemonic::Tax, AddressingMode::Implied)),

        (0xA8, OpcodeInfo::new(1, 2, Mnemonic::Tay, AddressingMode::Implied)),

        (0xBA, OpcodeInfo::new(1, 2, Mnemonic::Tsx, AddressingMode::Implied)),

        (0x8A, OpcodeInfo::new(1, 2, Mnemonic::Txa, AddressingMode::Implied)),

        (0x9A, OpcodeInfo::new(1, 2, Mnemonic::Txs, AddressingMode::Implied)),

        (0x98, OpcodeInfo::new(1, 2, Mnemonic::Tya, AddressingMode::Implied)),
    ])
});

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_opcode_table_with_supported_opcode() {
        let brk = OPCODE_TABLE.get(&0x00);
        let adc = OPCODE_TABLE.get(&0x69);

        assert_eq!(brk, Some(&OpcodeInfo::new(1, 7, Mnemonic::Brk, AddressingMode::Implied)));
        assert_eq!(adc, Some(&OpcodeInfo::new(2, 2, Mnemonic::Adc, AddressingMode::Immediate)));
    }

    #[test]
    fn test_opcode_table_with_illegal_opcode() {
        assert_eq!(None, OPCODE_TABLE.get(&0x02));
    }
}
