//! A module that provide trait to represent memory

use crate::core::types::*;

/// A trait that represent memory.
///
/// This trait is used to represent memory that cpu will read.
///
/// In arbitrarily address in closed interval [0x0000, 0xFFFF], all method must success.
pub trait Memory {
    /// Get 16-bit address and return 8-bit value from the address
    fn read_byte(&self, addr: Address) -> Byte;

    /// Get 16-bit address and write supplied 8-bit value to the address
    fn write_byte(&mut self, addr: Address, value: Byte);

    /// Get 16-bit address and return 16-bit value from the address
    ///
    /// The return value is encoded as native endian
    fn read_word(&self, addr: Address) -> Word {
        let bytes = [ self.read_byte(addr), self.read_byte(addr.wrapping_add(1)) ];
        u16::from_le_bytes(bytes)
    } 

    /// Get 16-bit address and write supplied 16-bit value to the address
    ///
    /// The supplied value should be native endian
    fn write_word(&mut self, addr: Address, value: Word) {
        let bytes = value.to_le_bytes();
        self.write_byte(addr.wrapping_add(0), bytes[0]);
        self.write_byte(addr.wrapping_add(1), bytes[1]);
    }
}

#[cfg(test)]
mod test {
    use super::Memory;

    struct MyVec {
        vec: Vec<u8>,
    }

    impl Memory for MyVec {
        fn read_byte(&self, addr: u16) -> u8 {
            if self.vec.len() > addr as usize {
                self.vec[addr as usize]
            } else {
                0
            }
        }

        fn write_byte(&mut self, addr: u16, value: u8) {
            if self.vec.len() > addr as usize {
                self.vec[addr as usize] = value;
            }
        }
    }

    #[test]
    fn test_read_word() {
        let mem = MyVec{ vec: vec![0x00, 0x01] };
        assert_eq!(mem.read_word(0), 0x0100);
    }

    #[test]
    fn test_write_word() {
        let mut mem = MyVec{ vec: vec![0x00, 0x01] };
        mem.write_word(0, 0x0100);
        assert_eq!(mem.vec, vec![0x00, 0x01]);
    }
}
