use bitflags::bitflags;
use crate::core::types::*;

bitflags! {
    pub struct Status: u8 {
        const CARRY     = 0b0000_0001;
        const ZERO      = 0b0000_0010;
        const INTERRUPT = 0b0000_0100;
        const DECIMAL   = 0b0000_1000;
        const BREAK1    = 0b0001_0000;
        const BREAK2    = 0b0010_0000; // This flag is always on
        const OVERFLOW  = 0b0100_0000;
        const NEGATIVE  = 0b1000_0000;
    }
}

// In this implementation, I don't create methods that operate carry and overflow flags.
// Becase, all instruction that operate these flag have different method for changing it.
impl Status {
    pub fn new() -> Self {
        Self::empty() | Status::BREAK2
    }

    // There is no official mnemonic that only change zero flag or negative flag.
    pub fn update_zero_and_negative(&mut self, result: u8) {
        if result == 0 {
            self.insert(Self::ZERO);
        } else {
            self.remove(Self::ZERO);
        }

        if result >> 7 == 1 {
            self.insert(Self::NEGATIVE);
        } else {
            self.remove(Self::NEGATIVE);
        }
    }
}

pub struct Register {
    pub a: Byte,
    pub x: Byte,
    pub y: Byte,
    pub pc: Address,
    pub s: Byte,
    pub p: Status,
}

impl Register {
    pub fn new() -> Self {
        Self { a: 0, x: 0, y: 0, pc: 0, s: 0, p: Status::new() }
    }
}
