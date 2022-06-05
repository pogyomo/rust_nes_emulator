//! Almost all code is from https://github.com/bugzmanov/nes_ebook

#![allow(dead_code)]

use super::cpu::memory::Memory;
use super::cartridge::Cartoridge;
use super::types::{Byte, Address};

pub struct Bus {
    ram: [Byte; 0x0800],
    cartridge: Cartoridge,
}

impl Bus {
    pub fn new(cartridge: Cartoridge) -> Self {
        Self {
            ram: [0; 0x0800],
            cartridge,
        }
    }

    fn read_prg_rom(&self, addr: Address) -> u8 {
        let mut addr = addr - 0x8000;
        if self.cartridge.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr = addr & 0x4000;
        }
        self.cartridge.prg_rom[addr as usize]
    }
}

const RAM_BEGIN: Address = 0x0000;
const RAM_END:   Address = 0x1FFF;
const PPU_BEGIN: Address = 0x2000;
const PPU_END:   Address = 0x3FFF;

impl Memory for Bus {
    fn read_byte(&self, addr: Address) -> Byte {
        match addr {
            RAM_BEGIN..=RAM_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.ram[mirror_down_addr as usize]
            }
            PPU_BEGIN..=PPU_END => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU is not implemented yet")
            }
            0x8000..=0xFFFF => self.read_prg_rom(addr),
            _ => {
                println!("This access is ignored: 0x{:x}", addr);
                0
            }
        }
    }

    fn write_byte(&mut self, addr: Address, value: Byte) {
        match addr {
            RAM_BEGIN..=RAM_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.ram[mirror_down_addr as usize] = value;
            }
            PPU_BEGIN..=PPU_END => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU is not implemented yet")
            }
            0x8000..=0xFFFF => {
                panic!("Attempt to write to cartridge rom space");
            }
            _ => {
                println!("This access is ignored: 0x{:x}", addr);
            }
        }
    }
}
