//! Almost all code is from https://github.com/bugzmanov/nes_ebook

#![allow(dead_code)]

const NES_IDENTIFIER: [u8; 4]  = [0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_PAGE_SIZE: usize = 0x4000;
const CHR_ROM_PAGE_SIZE: usize = 0x2000;

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

pub struct Cartoridge {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub mirroring: Mirroring,
}

impl Cartoridge {
    pub fn new(bytes: Vec<u8>) -> Result<Cartoridge, String> {
        if bytes[0..4] != NES_IDENTIFIER {
            return Err("The bytes is not iNES format".to_string());
        } else {
            let mapper = (bytes[7] & 0b1111_0000) | (bytes[6] >> 4);

            let ines_version = bytes[7] >> 2;
            if ines_version != 0 {
                return Err("iNES 2.0 is not supported".to_string());
            }

            let is_four_screen = bytes[6] & 0b1000 != 0;
            let is_vertical    = bytes[6] & 0b0001 != 0;
            let mirroring      = match (is_four_screen, is_vertical) {
                (true, _)      => Mirroring::FourScreen,
                (false, true)  => Mirroring::Vertical,
                (false, false) => Mirroring::Horizontal,
            };

            let prg_rom_size  = bytes[4] as usize * PRG_ROM_PAGE_SIZE;
            let chr_rom_size  = bytes[5] as usize * CHR_ROM_PAGE_SIZE;

            let skip_trainer  = bytes[6] & 0b0100 != 0;

            let prg_rom_start = 16 + if skip_trainer { 0x0200 } else { 0 };
            let chr_rom_start = prg_rom_start + prg_rom_size;

            Ok(Cartoridge {
                prg_rom: bytes[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
                chr_rom: bytes[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
                mapper,
                mirroring,
            })
        }
    }
}
