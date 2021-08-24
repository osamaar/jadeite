use std::{fs::File, io::Read};

pub struct Rom {
    pub prg_rom_page_count: u8,             // 4 - N x 16kb
    pub chr_rom_page_count: u8,             // 5 - N x 8kb

    pub mirroring: Mirroring,               // 6(0)
    pub sram_enable: bool,                  // 6(1)
    pub trainer_present: bool,              // 6(2)
    pub four_screen_vram_layout: bool,      // 6(3)

    pub mapper: u8,                         // low: 6(4:7), high: 7(4:7)

    pub is_vs_system: bool,                 // 7(0)
    pub ram_banks: u8,                      // 8 - 0: assume 1x8kb
    pub tv_system: TVSystem,                // 9(0)

    pub trainer: Option<Vec<u8>>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub extra_bytes: Vec<u8>,
}

pub enum Mirroring { Vertical, Horizontal }
pub enum TVSystem { NTSC, PAL }


impl Rom {
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

        let sram_enable = (header_buf[6] & 0b0000_0010) == 1;
        let trainer_present = (header_buf[6] & 0b0000_0100) == 1;
        let four_screen_vram_layout = (header_buf[6] & 0b0000_1000) == 1;
        let mapper = (header_buf[6] & 0b1111_0000) & ((header_buf[7] & 0b1111_0000) << 4);
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

        Ok(Self {
            prg_rom_page_count,
            chr_rom_page_count,
            mirroring,
            sram_enable,
            trainer_present,
            four_screen_vram_layout,
            mapper,
            is_vs_system,
            ram_banks,
            tv_system,
            trainer,
            prg_rom,
            chr_rom,
            extra_bytes,
        })
    }
}