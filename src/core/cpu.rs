#![allow(dead_code)]

// TODO:
// - [x] We need to do refactoring.
// - [x] Overflow in adc and sbc may incorrect.
// - [x] Implement jmp indirect addressing mode error.
// - [ ] Implement zeropage addressing mode error ($xx, x, $xx, y, $(xx, x), $(xx), y).
// - [ ] Implement brk, jsr, rti, rts. -- Need more infomatin about their behavior
// - [ ] Consider that BREAK2 in p register is always on.

pub mod memory;

mod opcode;
mod register;

use self::memory::Memory;
use self::opcode::*;
use self::register::*;
use crate::core::types::*;

pub struct Cpu {
    regs: Register,
    mem:  Box<dyn Memory>,
}

impl Cpu {
    // Get opcode and increment its program counter
    fn fetch_opcode(&mut self) -> Opcode {
        let opcode = self.mem.read_byte(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        opcode
    }

    // Return address where contain data, or return 0
    // After get address, increments its program counter
    fn fetch_address<'a>(&mut self, info: &'a OpcodeInfo) -> (Address, &'a AddressingMode) {
        // Fetch address
        let address = match info.mode {
            AddressingMode::Accumulator | AddressingMode::Implied => 0,
            AddressingMode::Absolute  => self.fetch_absolute_with_index(0),
            AddressingMode::AbsoluteX => self.fetch_absolute_with_index(self.regs.x),
            AddressingMode::AbsoluteY => self.fetch_absolute_with_index(self.regs.y),
            AddressingMode::Immediate => self.fetch_immediate(),
            AddressingMode::Indirect  => self.fetch_indirect(),
            AddressingMode::IndirectX => self.fetch_indirect_with_index(self.regs.x, 0),
            AddressingMode::IndirectY => self.fetch_indirect_with_index(0, self.regs.y),
            AddressingMode::Relative  => self.fetch_relative(),
            AddressingMode::ZeroPage  => self.fetch_zero_page_with_index(0),
            AddressingMode::ZeroPageX => self.fetch_zero_page_with_index(self.regs.x),
            AddressingMode::ZeroPageY => self.fetch_zero_page_with_index(self.regs.y),
        };

        // Increase program counter and return reslut
        self.regs.pc = self.regs.pc.wrapping_add(info.byte as Address - 1);
        (address, &info.mode)
    }

    fn fetch_absolute_with_index(&self, index: Byte) -> Address {
        self.mem.read_word(self.regs.pc.wrapping_add(index as Word)) as Address
    }

    fn fetch_immediate(&self) -> Address {
        self.regs.pc
    }

    fn fetch_indirect(&self) -> Address {
        let addr = self.mem.read_word(self.regs.pc);
        let lsb  = self.mem.read_byte(addr);
        // If addr == 0xXXFF (X is arbitrarily), msb will be fetched from 0xXX00
        let msb  = self.mem.read_byte(if addr & 0xFF == 0xFF { addr & 0xFF00 } else { addr + 1 });

        Address::from_le_bytes([lsb, msb])
    }

    fn fetch_indirect_with_index(&self, msb_index: Byte, lsb_index: Byte) -> Address {
        let addr = self.mem.read_byte(self.regs.pc).wrapping_add(msb_index) as Address;
        self.mem.read_word(addr).wrapping_add(lsb_index as Word) as Address
    }

    fn fetch_relative(&self) -> Address {
        let offset = self.mem.read_byte(self.regs.pc);
        if offset >> 7 == 1 {
            self.regs.pc.wrapping_add(1).wrapping_sub((!offset).wrapping_add(1) as Address)
        } else {
            self.regs.pc.wrapping_add(1).wrapping_add(offset as Address)
        }
    }

    fn fetch_zero_page_with_index(&self, index: Byte) -> Address {
        self.mem.read_byte(self.regs.pc).wrapping_add(index) as Address
    }
}

impl Cpu {
    pub fn new(mem: Box<dyn Memory>) -> Self {
        Self { regs: Register::new(), mem }
    }

    fn power_on(&mut self) {
        self.regs.a  = 0x00;
        self.regs.x  = 0x00;
        self.regs.y  = 0x00;
        self.regs.pc = self.mem.read_word(0xFFFC);
        self.regs.s  = 0xFD;
        self.regs.p.remove(Status::all());
        self.regs.p.insert(Status::INTERRUPT | Status::BREAK1 | Status::BREAK2);
    }

    fn reset(&mut self) {
        self.regs.pc = self.mem.read_word(0xFFFC);
        self.regs.s  = self.regs.s.wrapping_sub(3);
        self.regs.p.insert(Status::INTERRUPT);
    }

    fn run(&mut self) {
        loop {
            let opcode       = self.fetch_opcode();
            let info = OPCODE_TABLE.get(&opcode).unwrap_or_else(|| panic!("No such opcode: 0x{:x}", opcode));
            let (addr, name) = self.fetch_address(info); 

            match info.name {
                Mnemonic::Adc => self.adc(addr),
                Mnemonic::And => self.and(addr),
                Mnemonic::Asl if *name == AddressingMode::Accumulator => self.asl_acc(),
                Mnemonic::Asl if *name != AddressingMode::Implied     => self.asl(addr),
                Mnemonic::Bcc => self.bcc(addr),
                Mnemonic::Bcs => self.bcs(addr),
                Mnemonic::Beq => self.beq(addr),
                Mnemonic::Bit => self.bit(addr),
                Mnemonic::Bmi => self.bmi(addr),
                Mnemonic::Bne => self.bne(addr),
                Mnemonic::Bpl => self.bpl(addr),
                Mnemonic::Brk => return,
                Mnemonic::Bvc => self.bvc(addr),
                Mnemonic::Bvs => self.bvs(addr),
                Mnemonic::Clc if *name == AddressingMode::Implied => self.clc(),
                Mnemonic::Cld if *name == AddressingMode::Implied => self.cld(),
                Mnemonic::Cli if *name == AddressingMode::Implied => self.cli(),
                Mnemonic::Clv if *name == AddressingMode::Implied => self.clv(),
                Mnemonic::Cmp => self.cmp(addr),
                Mnemonic::Cpx => self.cpx(addr),
                Mnemonic::Cpy => self.cpy(addr),
                Mnemonic::Dec => self.dec(addr),
                Mnemonic::Dex if *name == AddressingMode::Implied => self.dex(),
                Mnemonic::Dey if *name == AddressingMode::Implied => self.dey(),
                Mnemonic::Eor => self.eor(addr),
                Mnemonic::Inc => self.inc(addr),
                Mnemonic::Inx if *name == AddressingMode::Implied => self.inx(),
                Mnemonic::Iny if *name == AddressingMode::Implied => self.iny(),
                Mnemonic::Jmp => self.jmp(addr),
                Mnemonic::Jsr => todo!("Jsr is not implemented"),
                Mnemonic::Lda => self.lda(addr),
                Mnemonic::Ldx => self.ldx(addr),
                Mnemonic::Ldy => self.ldy(addr),
                Mnemonic::Lsr if *name == AddressingMode::Accumulator => self.lsr_acc(),
                Mnemonic::Lsr if *name != AddressingMode::Implied     => self.lsr(addr),
                Mnemonic::Nop => (), // Nothing happen, so there is no function
                Mnemonic::Ora => self.ora(addr),
                Mnemonic::Pha if *name == AddressingMode::Implied => self.pha(),
                Mnemonic::Php if *name == AddressingMode::Implied => self.php(),
                Mnemonic::Pla if *name == AddressingMode::Implied => self.pla(),
                Mnemonic::Plp if *name == AddressingMode::Implied => self.plp(),
                Mnemonic::Rol if *name == AddressingMode::Accumulator => self.rol_acc(),
                Mnemonic::Rol if *name != AddressingMode::Implied     => self.rol(addr),
                Mnemonic::Ror if *name == AddressingMode::Accumulator => self.ror_acc(),
                Mnemonic::Ror if *name != AddressingMode::Implied     => self.ror(addr),
                Mnemonic::Rti if *name == AddressingMode::Implied => todo!("Rti is not implemented"),
                Mnemonic::Rts if *name == AddressingMode::Implied => todo!("Rts is not implemented"),
                Mnemonic::Sbc => self.sbc(addr),
                Mnemonic::Sec if *name == AddressingMode::Implied => self.sec(),
                Mnemonic::Sed if *name == AddressingMode::Implied => self.sed(),
                Mnemonic::Sei if *name == AddressingMode::Implied => self.sei(),
                Mnemonic::Sta => self.sta(addr),
                Mnemonic::Stx => self.stx(addr),
                Mnemonic::Sty => self.sty(addr),
                Mnemonic::Tax if *name == AddressingMode::Implied => self.tax(),
                Mnemonic::Tay if *name == AddressingMode::Implied => self.tay(),
                Mnemonic::Tsx if *name == AddressingMode::Implied => self.tsx(),
                Mnemonic::Txa if *name == AddressingMode::Implied => self.txa(),
                Mnemonic::Txs if *name == AddressingMode::Implied => self.txs(),
                Mnemonic::Tya if *name == AddressingMode::Implied => self.tya(),
                _ => panic!("{:?} is not exist on {:?}", info.mode, info.name),
            }
        }
    }

    fn branch(&mut self, addr: Address, success: bool) {
        if success {
            self.regs.pc = addr;
        }
    }

    fn push_byte(&mut self, value: Byte) {
        self.mem.write_byte(self.regs.s as Address + 0x0100, value);
        self.regs.s = self.regs.s.wrapping_sub(1);
    }

    fn pull_byte(&mut self) -> Byte {
        self.regs.s = self.regs.s.wrapping_add(1);
        self.mem.read_byte(self.regs.s as Address + 0x0100)
    }

    fn push_word(&mut self, value: Word) {
        let bytes = value.to_le_bytes();
        self.push_byte(bytes[0]);
        self.push_byte(bytes[1])
    }

    fn pull_word(&mut self) -> Word {
        let msb = self.pull_byte();
        let lsb = self.pull_byte();
        Word::from_le_bytes([msb, lsb])
    }

    fn adc(&mut self, addr: Address) {
        let carry        = if self.regs.p.contains(Status::CARRY) { 1 } else { 0 };
        let value_to_add = self.mem.read_byte(addr).overflowing_add(carry);
        let result       = self.regs.a.overflowing_add(value_to_add.0);
        let is_carry     = value_to_add.1 | result.1;
        let is_overflow  = (self.regs.a >> 7) == (value_to_add.0 >> 7) &&
                           (self.regs.a >> 7) != (result.0       >> 7);

        self.regs.a = result.0;

        self.regs.p.set(Status::CARRY, is_carry);
        self.regs.p.set(Status::OVERFLOW, is_overflow);
        self.regs.p.update_zero_and_negative(self.regs.a);
    }

    fn and(&mut self, addr: Address) {
        self.regs.a &= self.mem.read_byte(addr);
        self.regs.p.update_zero_and_negative(self.regs.a);
    }

    fn asl_acc(&mut self) {
        let is_carry = self.regs.a >> 7 == 1;
        self.regs.a <<= 1;

        self.regs.p.set(Status::CARRY, is_carry);
        self.regs.p.update_zero_and_negative(self.regs.a);
    }

    fn asl(&mut self, addr: Address) {
        let is_carry = self.mem.read_byte(addr) >> 7 == 1;
        self.mem.write_byte(addr, self.mem.read_byte(addr) << 1);

        self.regs.p.set(Status::CARRY, is_carry);
        self.regs.p.update_zero_and_negative(self.mem.read_byte(addr));
    }

    fn bcc(&mut self, addr: Address) {
        self.branch(addr, !self.regs.p.contains(Status::CARRY));
    }

    fn bcs(&mut self, addr: Address) {
        self.branch(addr, self.regs.p.contains(Status::CARRY));
    }

    fn beq(&mut self, addr: Address) {
        self.branch(addr, self.regs.p.contains(Status::ZERO));
    }

    fn bit(&mut self, addr: Address) {
        self.regs.p.set(Status::NEGATIVE, self.mem.read_byte(addr) & 0b1000_0000 != 0);
        self.regs.p.set(Status::OVERFLOW, self.mem.read_byte(addr) & 0b0100_0000 != 0);
        self.regs.p.set(Status::ZERO,     self.mem.read_byte(addr) & self.regs.a != 0);
    }

    fn bmi(&mut self, addr: Address) {
        self.branch(addr, self.regs.p.contains(Status::NEGATIVE));
    }

    fn bne(&mut self, addr: Address) {
        self.branch(addr, !self.regs.p.contains(Status::ZERO));
    }

    fn bpl(&mut self, addr: Address) {
        self.branch(addr, !self.regs.p.contains(Status::NEGATIVE));
    }

    fn bvc(&mut self, addr: Address) {
        self.branch(addr, !self.regs.p.contains(Status::OVERFLOW));
    }

    fn bvs(&mut self, addr: Address) {
        self.branch(addr, self.regs.p.contains(Status::OVERFLOW));
    }

    fn clc(&mut self) {
        self.regs.p.remove(Status::CARRY);
    }

    fn cld(&mut self) {
        self.regs.p.remove(Status::DECIMAL);
    }

    fn cli(&mut self) {
        self.regs.p.remove(Status::INTERRUPT);
    }

    fn clv(&mut self) {
        self.regs.p.remove(Status::OVERFLOW);
    }

    fn cmp(&mut self, addr: Address) {
        let result = self.regs.a.overflowing_sub(self.mem.read_byte(addr));

        self.regs.p.set(Status::CARRY, !result.1);
        self.regs.p.update_zero_and_negative(result.0);
    }

    fn cpx(&mut self, addr: Address) {
        let result = self.regs.x.overflowing_sub(self.mem.read_byte(addr));

        self.regs.p.set(Status::CARRY, !result.1);
        self.regs.p.update_zero_and_negative(result.0);
    }

    fn cpy(&mut self, addr: Address) {
        let result = self.regs.y.overflowing_sub(self.mem.read_byte(addr));

        self.regs.p.set(Status::CARRY, !result.1);
        self.regs.p.update_zero_and_negative(result.0);
    }

    fn dec(&mut self, addr: Address) {
        let result = self.mem.read_byte(addr).wrapping_sub(1);
        self.mem.write_byte(addr, result);

        self.regs.p.update_zero_and_negative(result);
    }

    fn dex(&mut self) {
        self.regs.x = self.regs.x.wrapping_sub(1);

        self.regs.p.update_zero_and_negative(self.regs.x);
    }

    fn dey(&mut self) {
        self.regs.y = self.regs.y.wrapping_sub(1);

        self.regs.p.update_zero_and_negative(self.regs.y);
    }

    fn eor(&mut self, addr: Address) {
        self.regs.a = self.regs.a ^ self.mem.read_byte(addr);

        self.regs.p.update_zero_and_negative(self.regs.a);
    }

    fn inc(&mut self, addr: Address) {
        let result = self.mem.read_byte(addr).wrapping_add(1);
        self.mem.write_byte(addr, result);

        self.regs.p.update_zero_and_negative(result);
    }

    fn inx(&mut self) {
        self.regs.x = self.regs.x.wrapping_add(1);

        self.regs.p.update_zero_and_negative(self.regs.x);
    }

    fn iny(&mut self) {
        self.regs.y = self.regs.y.wrapping_add(1);

        self.regs.p.update_zero_and_negative(self.regs.y);
    }

    fn jmp(&mut self, addr: Address) {
        self.regs.pc = addr;
    }

    fn lda(&mut self, addr: Address) {
        self.regs.a = self.mem.read_byte(addr);

        self.regs.p.update_zero_and_negative(self.regs.a);
    }

    fn ldx(&mut self, addr: Address) {
        self.regs.x = self.mem.read_byte(addr);

        self.regs.p.update_zero_and_negative(self.regs.x);
    }

    fn ldy(&mut self, addr: Address) {
        self.regs.y = self.mem.read_byte(addr);

        self.regs.p.update_zero_and_negative(self.regs.y);
    }

    fn lsr_acc(&mut self) {
        let is_carry = self.regs.a & 0b0000_0001 == 0b0000_0001;
        self.regs.a >>= 1;

        self.regs.p.set(Status::CARRY, is_carry);
        self.regs.p.update_zero_and_negative(self.regs.a);
    }

    fn lsr(&mut self, addr: Address) {
        let is_carry = self.mem.read_byte(addr) & 0b0000_0001 == 0b0000_0001;
        self.mem.write_byte(addr, self.mem.read_byte(addr) >> 1);

        self.regs.p.set(Status::CARRY, is_carry);
        self.regs.p.update_zero_and_negative(self.mem.read_byte(addr));
    }

    fn ora(&mut self, addr: Address) {
        self.regs.a |= self.mem.read_byte(addr);

        self.regs.p.update_zero_and_negative(self.regs.a);
    }

    fn pha(&mut self) {
        self.push_byte(self.regs.a);
    }

    fn php(&mut self) {
        self.push_byte(self.regs.p.bits());
    }

    fn pla(&mut self) {
        self.regs.a = self.pull_byte();

        self.regs.p.update_zero_and_negative(self.regs.a);
    }

    fn plp(&mut self) {
        self.regs.p = Status::from_bits_truncate(self.pull_byte());
    }

    fn rol_acc(&mut self) {
        let is_carry = self.regs.a >> 7 == 1;
        let carry    = if self.regs.p.contains(Status::CARRY) { 1 } else { 0 };
        self.regs.a  = (self.regs.a << 1) + carry;

        self.regs.p.set(Status::CARRY, is_carry);
        self.regs.p.update_zero_and_negative(self.regs.a);
    }

    fn rol(&mut self, addr: Address) {
        let is_carry = self.mem.read_byte(addr) >> 7 == 1;
        let carry    = if self.regs.p.contains(Status::CARRY) { 1 } else { 0 };
        self.mem.write_byte(addr, (self.mem.read_byte(addr) << 1) + carry);

        self.regs.p.set(Status::CARRY, is_carry);
        self.regs.p.update_zero_and_negative(self.mem.read_byte(addr));
    }

    fn ror_acc(&mut self) {
        let is_carry = self.regs.a & 0b0000_0001 == 0b0000_0001;
        let carry    = if self.regs.p.contains(Status::CARRY) { 0b1000_0000 } else { 0 };
        self.regs.a  = (self.regs.a >> 1) + carry;

        self.regs.p.set(Status::CARRY, is_carry);
        self.regs.p.update_zero_and_negative(self.regs.a);
    }

    fn ror(&mut self, addr: Address) {
        let is_carry = self.regs.a & 0b0000_0001 == 0b0000_0001;
        let carry    = if self.regs.p.contains(Status::CARRY) { 0b1000_0000 } else { 0 };
        self.mem.write_byte(addr, (self.mem.read_byte(addr) >> 1) + carry);

        self.regs.p.set(Status::CARRY, is_carry);
        self.regs.p.update_zero_and_negative(self.mem.read_byte(addr));
    }

    fn sbc(&mut self, addr: Address) {
        let carry        = if self.regs.p.contains(Status::CARRY) { 0 } else { 1 };
        let value_to_sub = self.mem.read_byte(addr).overflowing_add(carry);
        let result       = self.regs.a.overflowing_sub(value_to_sub.0);
        let is_carry     = !(value_to_sub.1 | result.1);
        let is_overflow  = (self.regs.a >> 7) == (value_to_sub.0 >> 7) &&
                           (self.regs.a >> 7) != (result.0       >> 7);

        self.regs.a = result.0;

        self.regs.p.set(Status::CARRY, is_carry);
        self.regs.p.set(Status::OVERFLOW, is_overflow);
        self.regs.p.update_zero_and_negative(self.regs.a);
    }

    fn sec(&mut self) {
        self.regs.p.insert(Status::CARRY);
    }

    fn sed(&mut self) {
        self.regs.p.insert(Status::DECIMAL);
    }

    fn sei(&mut self) {
        self.regs.p.insert(Status::INTERRUPT);
    }

    fn sta(&mut self, addr: Address) {
        self.mem.write_byte(addr, self.regs.a);
    }

    fn stx(&mut self, addr: Address) {
        self.mem.write_byte(addr, self.regs.x);
    }

    fn sty(&mut self, addr: Address) {
        self.mem.write_byte(addr, self.regs.y);
    }

    fn tax(&mut self) {
        self.regs.x = self.regs.a;

        self.regs.p.update_zero_and_negative(self.regs.x);
    }

    fn tay(&mut self) {
        self.regs.y = self.regs.a;

        self.regs.p.update_zero_and_negative(self.regs.y);
    }

    fn tsx(&mut self) {
        self.regs.x = self.regs.s;

        self.regs.p.update_zero_and_negative(self.regs.x);
    }

    fn txa(&mut self) {
        self.regs.a = self.regs.x;

        self.regs.p.update_zero_and_negative(self.regs.a);
    }

    fn txs(&mut self) {
        self.regs.s = self.regs.x;
    }

    fn tya(&mut self) {
        self.regs.a = self.regs.y;

        self.regs.p.update_zero_and_negative(self.regs.a);
    }
}

#[cfg(test)]
mod test {
    use std::fs::*;
    use std::io::*;
    use super::*;
    use crate::core::bus::Bus;
    use crate::core::cartridge::Cartoridge;

    struct MyVec {
        vec: Vec<u8>,
    }

    impl MyVec {
        fn new(program: Vec<u8>) -> Self {
            let mut vec: Vec<u8> = vec![2; 0xFFFF];
            vec[0xFFFC] = 0x00;
            vec[0xFFFD] = 0x80;
            for (index, byte) in program.into_iter().enumerate() {
                vec[0x8000 + index] = byte;
            }
            Self { vec }
        }
    }

    impl Memory for MyVec {
        fn read_byte(&self, addr: Address) -> Byte {
            if self.vec.len() > addr as usize {
                self.vec[addr as usize]
            } else {
                0
            }
        }

        fn write_byte(&mut self, addr: Address, value: Byte) {
            if self.vec.len() > addr as usize {
                self.vec[addr as usize] = value;
            }
        }
    }

    #[test]
    fn test_update_negative_and_zero_is_working() {
        let mut status = Status::new();
        status.update_zero_and_negative(0x00);
        assert!(status.contains(Status::ZERO));
        status.update_zero_and_negative(0xFF);
        assert!(status.contains(Status::NEGATIVE));
    }

    #[test]
    fn test_adc() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x69, 0xFF, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0x10;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.a, 0x0F);
        assert!(cpu.regs.p.contains(Status::CARRY));
    }

    #[test]
    fn test_adc_overflow() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x69, 0x80, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0x80;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.a, 0x00);
        assert!(cpu.regs.p.contains(Status::CARRY & Status::OVERFLOW & Status::ZERO));
    }

    #[test]
    fn test_and() {
        let mut mem = MyVec::new(vec![0x25, 0x00, 0x00]);
        mem.vec[0] = 0x10;

        let mut cpu = Cpu::new(Box::new(mem));
        cpu.power_on();
        cpu.regs.a = 0xFF;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.a, 0x10);
        assert!(cpu.regs.p.is_empty());
    }

    #[test]
    fn test_asl() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x0A, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0xFF;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.a, 0xFF << 1);
        assert_eq!(cpu.regs.p.bits(), Status::CARRY.bits() | Status::NEGATIVE.bits());
    }

    #[test]
    fn test_bcc() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x90, 0x01, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.pc, 0x8004);
    }

    #[test]
    fn test_bcs() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xB0, 0x01, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.p.remove(Status::all());
        cpu.regs.p.insert(Status::CARRY);
        cpu.run();

        assert_eq!(cpu.regs.pc, 0x8004);
    }

    #[test]
    fn test_beq() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xF0, 0x01, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.p.remove(Status::all());
        cpu.regs.p.insert(Status::ZERO);
        cpu.run();

        assert_eq!(cpu.regs.pc, 0x8004);
    }

    #[test]
    fn test_bit() {
        let mut mem = MyVec::new(vec![0x24, 0x00, 0x00]);
        mem.vec[0] = 0xC0;

        let mut cpu = Cpu::new(Box::new(mem));
        cpu.power_on();
        cpu.regs.a = 0xC0;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert!(cpu.regs.p.contains(Status::NEGATIVE | Status::OVERFLOW | Status::ZERO));
    }

    #[test]
    fn test_bmi() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x30, 0x01, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.p.remove(Status::all());
        cpu.regs.p.insert(Status::NEGATIVE);
        cpu.run();

        assert_eq!(cpu.regs.pc, 0x8004);
    }

    #[test]
    fn test_bne() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xD0, 0x01, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.pc, 0x8004);
    }

    #[test]
    fn test_bpl() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x10, 0x01, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.pc, 0x8004);
    }

    #[test]
    fn test_bvc() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x50, 0x01, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.pc, 0x8004);
    }

    #[test]
    fn test_bvs() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x70, 0x01, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.p.remove(Status::all());
        cpu.regs.p.insert(Status::OVERFLOW);
        cpu.run();

        assert_eq!(cpu.regs.pc, 0x8004);
    }

    #[test]
    fn test_clc() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x18, 0x00])));
        cpu.power_on();
        cpu.regs.p.insert(Status::all());
        cpu.run();

        assert!(!cpu.regs.p.contains(Status::CARRY));
    }

    #[test]
    fn test_cld() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xD8, 0x00])));
        cpu.power_on();
        cpu.regs.p.insert(Status::all());
        cpu.run();

        assert!(!cpu.regs.p.contains(Status::DECIMAL));
    }

    #[test]
    fn test_cli() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x58, 0x00])));
        cpu.power_on();
        cpu.regs.p.insert(Status::all());
        cpu.run();

        assert!(!cpu.regs.p.contains(Status::INTERRUPT));
    }

    #[test]
    fn test_clv() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xB8, 0x00])));
        cpu.power_on();
        cpu.regs.p.insert(Status::all());
        cpu.run();

        assert!(!cpu.regs.p.contains(Status::OVERFLOW));
    }

    #[test]
    fn test_cmp() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xC9, 0x10, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0x10;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert!(cpu.regs.p.contains(Status::ZERO & Status::CARRY));
    }

    #[test]
    fn test_cpx() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xE0, 0x10, 0x00])));
        cpu.power_on();
        cpu.regs.x = 0x10;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert!(cpu.regs.p.contains(Status::ZERO & Status::CARRY));
    }

    #[test]
    fn test_cpy() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xC0, 0x10, 0x00])));
        cpu.power_on();
        cpu.regs.y = 0x10;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert!(cpu.regs.p.contains(Status::ZERO & Status::CARRY));
    }

    #[test]
    fn test_dec() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xD6, 0x00, 0x00])));
        cpu.power_on();
        cpu.mem.write_byte(0x0010, 1);
        cpu.regs.x = 0x10;
        cpu.run();

        assert_eq!(cpu.mem.read_byte(0x0010), 0);
        assert!(cpu.regs.p.contains(Status::ZERO));
    }

    #[test]
    fn test_dex() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xCA, 0x00])));
        cpu.power_on();
        cpu.regs.x = 0x01;
        cpu.run();

        assert_eq!(cpu.regs.x, 0);
        assert!(cpu.regs.p.contains(Status::ZERO));
    }

    #[test]
    fn test_dey() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x88, 0x00])));
        cpu.power_on();
        cpu.regs.y = 0x01;
        cpu.run();

        assert_eq!(cpu.regs.y, 0);
        assert!(cpu.regs.p.contains(Status::ZERO));
    }

    #[test]
    fn test_eor() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x51, 0x00, 0x00])));
        cpu.power_on();
        cpu.mem.write_word(0x0000, 0x0020);
        cpu.mem.write_byte(0x0022, 0x10);
        cpu.regs.a = 0x30;
        cpu.regs.y = 0x02;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.a, 0x20);
        assert!(cpu.regs.p.is_empty());
    }

    #[test]
    fn test_inc() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xEE, 0x00, 0x02, 0x00])));
        cpu.power_on();
        cpu.mem.write_byte(0x0200, 0xFF);
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.mem.read_byte(0x0200), 0x00);
        assert!(cpu.regs.p.contains(Status::ZERO));
    }

    #[test]
    fn test_inx() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xE8, 0x00])));
        cpu.power_on();
        cpu.regs.x = 0xFF;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.x, 0x00);
        assert!(cpu.regs.p.contains(Status::ZERO));
    }

    #[test]
    fn test_iny() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xC8, 0x00])));
        cpu.power_on();
        cpu.regs.y = 0xFF;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.y, 0x00);
        assert!(cpu.regs.p.contains(Status::ZERO));
    }

    #[test]
    fn test_jmp() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x6C, 0x00, 0x00])));
        cpu.power_on();
        cpu.mem.write_word(0x0000, 0x0200);
        cpu.mem.write_byte(0x0200, 0x00);
        cpu.run();

        assert_eq!(cpu.regs.pc, 0x0201);
    }

    #[test]
    fn test_jmp_indirect_error() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x6C, 0xFF, 0x00])));
        cpu.power_on();

        cpu.mem.write_byte(0x0000, 0x02);
        cpu.mem.write_byte(0x00FF, 0x00);
        cpu.mem.write_byte(0x0100, 0x03); // Never be touched if movement is correct

        cpu.mem.write_byte(0x0200, 0x00);
        cpu.mem.write_byte(0x0300, 0x00);
        cpu.run();

        assert_eq!(cpu.regs.pc, 0x0201);
    }

    #[test]
    fn test_lda() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xA9, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0x10;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.a, 0x00);
        assert!(cpu.regs.p.contains(Status::ZERO));
    }

    #[test]
    fn test_ldx() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xA2, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.x = 0x10;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.x, 0x00);
        assert!(cpu.regs.p.contains(Status::ZERO));
    }

    #[test]
    fn test_ldy() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xA0, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.y = 0x10;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.y, 0x00);
        assert!(cpu.regs.p.contains(Status::ZERO));
    }

    #[test]
    fn test_lsr() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x4A, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0x10;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.a, 0x08);
        assert!(cpu.regs.p.is_empty());
    }

    #[test]
    fn test_ora() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x09, 0x81, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0x10;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.a, 0x91);
        assert!(cpu.regs.p.contains(Status::NEGATIVE));
    }

    #[test]
    fn test_pha() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x48, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0x10;
        cpu.regs.s = 0xFF;
        cpu.run();

        assert_eq!(cpu.mem.read_byte(0x01FF), 0x10);
        assert_eq!(cpu.regs.s, 0xFE);
    }

    #[test]
    fn test_php() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x08, 0x00])));
        let flags   = Status::CARRY & Status::ZERO & Status::NEGATIVE;
        cpu.power_on();
        cpu.regs.s = 0xFF;
        cpu.regs.p.remove(Status::all());
        cpu.regs.p.insert(flags);
        cpu.run();

        assert_eq!(cpu.mem.read_byte(0x01FF), flags.bits());
        assert_eq!(cpu.regs.s, 0xFE);
    }

    #[test]
    fn test_pla() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x68, 0x00])));
        cpu.power_on();
        cpu.mem.write_byte(0x01FF, 0x00);
        cpu.regs.s = 0xFE;
        cpu.run();

        assert_eq!(cpu.regs.a, 0x00);
        assert!(cpu.regs.p.contains(Status::ZERO));
    }

    #[test]
    fn test_plp() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x28, 0x00])));
        cpu.power_on();
        cpu.mem.write_byte(0x01FF, 0xFF);
        cpu.regs.s = 0xFE;
        cpu.run();

        assert_eq!(cpu.regs.s, 0xFF);
        assert!(cpu.regs.p.is_all());
    }

    #[test]
    fn test_rol() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x2A, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0xFF;
        cpu.regs.p.remove(Status::all());
        cpu.regs.p.insert(Status::CARRY);
        cpu.run();

        assert_eq!(cpu.regs.a, 0xFF);
        assert!(cpu.regs.p.contains(Status::CARRY & Status::NEGATIVE));
    }

    #[test]
    fn test_ror() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x6A, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0xFF;
        cpu.regs.p.remove(Status::all());
        cpu.regs.p.insert(Status::CARRY);
        cpu.run();

        assert_eq!(cpu.regs.a, 0xFF);
        assert!(cpu.regs.p.contains(Status::CARRY & Status::NEGATIVE));
    }

    #[test]
    fn test_sbc() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xE9, 0x10, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0x10;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.a, 0xFF);
        assert!(cpu.regs.p.contains(Status::NEGATIVE));
    }

    #[test]
    fn test_sbc_overflow() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xE9, 0x70, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0x80;
        cpu.regs.p.remove(Status::all());
        cpu.regs.p.insert(Status::CARRY);
        cpu.run();

        assert_eq!(cpu.regs.a, 0x10);
        assert!(cpu.regs.p.contains(Status::CARRY & Status::OVERFLOW));
    }

    #[test]
    fn test_sec() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x38, 0x00])));
        cpu.power_on();
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert!(cpu.regs.p.contains(Status::CARRY));
    }

    #[test]
    fn test_sed() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xF8, 0x00])));
        cpu.power_on();
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert!(cpu.regs.p.contains(Status::DECIMAL));
    }

    #[test]
    fn test_sei() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x78, 0x00])));
        cpu.power_on();
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert!(cpu.regs.p.contains(Status::INTERRUPT));
    }

    #[test]
    fn test_sta() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x85, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0x10;
        cpu.run();

        assert_eq!(cpu.mem.read_byte(0x0000), 0x10);
    }

    #[test]
    fn test_stx() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x86, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.x = 0x10;
        cpu.run();

        assert_eq!(cpu.mem.read_byte(0x0000), 0x10);
    }

    #[test]
    fn test_sty() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x84, 0x00, 0x00])));
        cpu.power_on();
        cpu.regs.y = 0x10;
        cpu.run();

        assert_eq!(cpu.mem.read_byte(0x0000), 0x10);
    }

    #[test]
    fn test_tax() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xAA, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0xFF;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.x, 0xFF);
        assert!(cpu.regs.p.contains(Status::NEGATIVE));
    }

    #[test]
    fn test_tay() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xA8, 0x00])));
        cpu.power_on();
        cpu.regs.a = 0xFF;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.y, 0xFF);
        assert!(cpu.regs.p.contains(Status::NEGATIVE));
    }

    #[test]
    fn test_tsx() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0xBA, 0x00])));
        cpu.power_on();
        cpu.regs.s = 0xFF;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.x, 0xFF);
        assert!(cpu.regs.p.contains(Status::NEGATIVE));
    }

    #[test]
    fn test_txa() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x8A, 0x00])));
        cpu.power_on();
        cpu.regs.x = 0xFF;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.a, 0xFF);
        assert!(cpu.regs.p.contains(Status::NEGATIVE));
    }

    #[test]
    fn test_txs() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x9A, 0x00])));
        cpu.power_on();
        cpu.regs.x = 0xFF;
        cpu.run();

        assert_eq!(cpu.regs.s, 0xFF);
    }

    #[test]
    fn test_tya() {
        let mut cpu = Cpu::new(Box::new(MyVec::new(vec![0x98, 0x00])));
        cpu.power_on();
        cpu.regs.y = 0xFF;
        cpu.regs.p.remove(Status::all());
        cpu.run();

        assert_eq!(cpu.regs.a, 0xFF);
        assert!(cpu.regs.p.contains(Status::NEGATIVE));
    }
}
