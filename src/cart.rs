use std::io::Read;
use std::fs::File;
use std::fmt::Debug;

use crate::mapper::{Mapper, Mapper000};

pub struct Cart {
    pub data: CartData,
    pub mapper: Box<dyn Mapper>,            // low: 6(4:7), high: 7(4:7)
}

pub struct CartData {
    pub prg_rom_page_count: u8,             // 4 - N x 16kb
    pub chr_rom_page_count: u8,             // 5 - N x 8kb

    pub mirroring: Mirroring,               // 6(0)
    pub sram_enable: bool,                  // 6(1)
    pub trainer_present: bool,              // 6(2)
    pub four_screen_vram_layout: bool,      // 6(3)

    pub mapper_id: u16,                      // low: 6(4:7), high: 7(4:7)

    pub is_vs_system: bool,                 // 7(0)
    pub ram_banks: u8,                      // 8 - 0: assume 1x8kb
    pub tv_system: TVSystem,                // 9(0)

    // memory
    pub trainer: Option<Vec<u8>>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub extra_bytes: Vec<u8>,
    pub chr_ram: Vec<u8>,
    
}

#[derive(Debug)]
pub enum Mirroring { Vertical, Horizontal }

#[derive(Debug)]
pub enum TVSystem { NTSC, PAL }


impl Cart {
    pub fn read_file(fname: &str) -> Result<Self, ()> {
        let mut src = File::open(fname).map_err(|_| ())?;
        Self::read_from(&mut src)
    }

    pub fn read_from<T: Read>(src: &mut T) -> Result<Self, ()> {
        let mut header_buf = vec![0u8; 16];
        src.read_exact(&mut header_buf).map_err(|_| ())?;

        let prg_rom_page_count = header_buf[4];
        let chr_rom_page_count = header_buf[5];

        let mirroring = match header_buf[6] & 0b0000_0001 {
            0 => Mirroring::Horizontal,
            1 => Mirroring::Vertical,
            _ => unreachable!()
        };

        let sram_enable = (header_buf[6] & 0b0000_0010) >> 1 == 1;
        let trainer_present = (header_buf[6] & 0b0000_0100) >> 2 == 1;
        let four_screen_vram_layout = (header_buf[6] & 0b0000_1000) >> 3 == 1;

        let mapper_id_lo = (header_buf[6] & 0b1111_0000) as u16;
        let mapper_id_hi = (header_buf[7] & 0b1111_0000) as u16;
        let mapper_id = mapper_id_lo & (mapper_id_hi << 4);
        let mapper = Self::create_mapper(mapper_id);

        let is_vs_system = (header_buf[7] & 0b0000_0001) == 1;
        let ram_banks = header_buf[8];
        let tv_system = match header_buf[9] & 0b0000_0001 {
            0 => TVSystem::NTSC,
            1 => TVSystem::PAL,
            _ => unreachable!()
        };

        let trainer = match trainer_present {
            true => {
                let mut buf = vec![0u8; 512];
                src.read_exact(&mut buf).map_err(|_| ())?;
                Some(buf)
            },
            false => None
        };

        let mut prg_rom: Vec<u8> = vec![0u8; prg_rom_page_count as usize*16*1024];
        src.read_exact(&mut prg_rom).map_err(|_| ())?;
        
        let mut chr_rom: Vec<u8> = vec![0u8; chr_rom_page_count as usize*8*1024];
        src.read_exact(&mut chr_rom).map_err(|_| ())?;

        let mut extra_bytes = Vec::new();
        src.read_to_end(&mut extra_bytes).map_err(|_| ())?;

        let chr_ram_size = 0x1fff*(1-chr_rom_page_count) as usize;
        let chr_ram = vec![0u8; chr_ram_size];

        Ok(Self {
            data: CartData {
                prg_rom_page_count,
                chr_rom_page_count,
                mirroring,
                sram_enable,
                trainer_present,
                four_screen_vram_layout,
                mapper_id,
                is_vs_system,
                ram_banks,
                tv_system,
                trainer,
                prg_rom,
                chr_rom,
                extra_bytes,
                chr_ram,
            },
            mapper,
        })
    }

    fn create_mapper(id: u16) -> Box<dyn Mapper> {
        match id {
            0 => Box::new(Mapper000::new()),
            _ => unimplemented!()
        }
    }

    pub fn cpu_read(&self, addr: u16) -> u8 {
        self.mapper.cpu_read(&self.data, addr)
    }

    pub fn cpu_write(&mut self, addr: u16, value: u8) {
        self.mapper.cpu_write(&mut self.data, addr, value)
    }


    pub fn ppu_read(&self, addr: u16) -> u8 {
        self.mapper.ppu_read(&self.data, addr)
    }

    pub fn ppu_write(&mut self, addr: u16, value: u8) {
        self.mapper.ppu_write(&mut self.data, addr, value)
    }
}

fn vec_to_u8_4_arr(v: &Vec<u8>) -> [u8; 4] {
    let mut mem4 = [0u8; 4];

    for (dst, src) in mem4.iter_mut().zip(v.iter()) {
        *dst = *src;
    }

    mem4
}

impl Debug for Cart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s;
        let trainer = match &self.data.trainer {
            Some(mem) => {
                s = format!( "{} bytes of memory: {:x?}..."
                    , mem.len(), vec_to_u8_4_arr(mem)
                );
                &s
            },
            None => "no trainer"
        };

        f.debug_struct("Cart")
            .field("prg_rom_page_count", &self.data.prg_rom_page_count)
            .field("chr_rom_page_count", &self.data.chr_rom_page_count)
            .field("mirroring", &self.data.mirroring)
            .field("sram_enable", &self.data.sram_enable)
            .field("trainer_present", &self.data.trainer_present)
            .field("four_screen_vram_layout", &self.data.four_screen_vram_layout)
            .field("mapper", &self.mapper.id())
            .field("is_vs_system", &self.data.is_vs_system)
            .field("ram_banks", &self.data.ram_banks)
            .field("tv_system", &self.data.tv_system)
            .field("trainer", &trainer)
            .field("prg_rom", &format!("PRG ROM: {} bytes", self.data.prg_rom.len()))
            .field("chr_rom", &format!("CHR ROM: {} bytes", self.data.chr_rom.len()))
            .field("extra_bytes", &format!("extra bytes: {} bytes", self.data.extra_bytes.len()))
            .field("chr_ram", &format!("CHR RAM: {} bytes", self.data.chr_ram.len()))
            .finish()
    }
}