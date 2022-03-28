#![allow(unused_variables, dead_code)]

use std::fmt::Debug;

use crate::{Bus, palette::Palette};
// #![allow(non_snake_case)]

pub struct Ppu {
    /// `$2000` Write
    ppu_ctrl: RegPPUCtrl,
    /// `$2001` Write
    ppu_mask: RegPPUMask,
    /// `$2002` Read
    ppu_status: RegPPUStatus,
    /// `$2003` Write
    oam_addr: u8,

    // `$2004` Read/Write
    // OAM Data             
 
    /// `$2005` Write (x2)
    ppu_scroll: PPUScroll,
    /// `$2006` Write (x2)
    ppu_addr: PPUAddress,
    /// `$2007` Read/Write
    ppu_data: u8,
    /// `$4014` Write
    oam_dma: u8,

    vram: [u8; 2048],
    color_palette: Palette,
    
    clock_count: usize,
    scanline: usize,
    scanline_cycle: usize,

    pub nmi_signal: bool,
}

impl Ppu {
    pub fn new() -> Self {
        let color_palette = Palette::from_file(
            "resources/ntscpalette.pal"
        ).unwrap();

        Self {
            ppu_ctrl: RegPPUCtrl::default(),
            ppu_mask: RegPPUMask::default(),
            ppu_status: RegPPUStatus::default(),
            oam_addr: 0,
            ppu_scroll: PPUScroll::default(),
            ppu_addr: PPUAddress::default(),
            ppu_data: 0,
            oam_dma: 0,
            vram: [0; 2048],
            color_palette,
            clock_count: 0,
            scanline: 261,
            scanline_cycle: 0,
            nmi_signal: false,
        }
    }
        
    pub fn step(&mut self, bus: &mut Bus) {
        match self.scanline {
            241 => {
                if self.scanline_cycle == 1 {
                    self.ppu_status.vblank = true;
                    self.nmi_signal = self.ppu_status.vblank &&
                        self.ppu_ctrl.nmi_enable;
                }
            },
            261 => {
                if self.scanline_cycle == 1 {
                    self.ppu_status.vblank = false;
                }
            },
            _ => {
            }
        }

        // Generate NMI
        // println!(
        //     "{} {} vblnk:{} nmi_e:{} trig:{}",
        //     self.scanline,
        //     self.scanline_cycle,
        //     self.ppu_status.vblank,
        //     self.ppu_ctrl.nmi_enable,
        //     self.nmi_signal
        // );

        self.clock_count += 1;
        self.clock_count %= 262*340;

        self.scanline_cycle += 1;
        self.scanline_cycle %= 341;

        self.scanline += (self.scanline_cycle == 0) as usize;
        self.scanline %= 262;
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x2000 => unimplemented!(),
            0x2001 => unimplemented!(),
            0x2002 => {
                let result = (&self.ppu_status).into();
                self.ppu_status.vblank = false;
                result
            },
            0x2003 => unimplemented!(),
            0x2004 => unimplemented!(),
            0x2005 => unimplemented!(),
            0x2006 => unimplemented!(),
            0x2007 => self.ppu_data,
            _ => unreachable!()
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x2000 => self.ppu_ctrl = value.into(),
            0x2001 => self.ppu_mask = value.into(),
            0x2002 => {todo!()},
            0x2003 => {todo!()},
            0x2004 => {todo!()},
            0x2005 => self.ppu_scroll.store(value),
            0x2006 => self.ppu_addr.store(value),
            0x2007 => {
                self.vram[self.ppu_addr.value as usize] = value;
            },
            _ => unreachable!()
        }

    }
}

impl Debug for Ppu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ppu")
            .field("ppu_ctrl", &self.ppu_ctrl)
            .field("ppu_mask", &self.ppu_mask)
            .field("ppu_status", &self.ppu_status)
            .field("oam_addr", &self.oam_addr)
            .field("ppu_scroll", &self.ppu_scroll)
            .field("ppu_addr", &self.ppu_addr)
            .field("ppu_data", &self.ppu_data)
            .field("oam_dma", &self.oam_dma)
            .field("color_palette", &"<Color Palette>").finish()
    }
}

/// ```text
/// 7  bit  0
/// ---- ----
/// VPHB SINN
/// |||| ||||
/// |||| ||++- Base nametable address
/// |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
/// |||| |+--- VRAM address increment per CPU read/write of PPUDATA
/// |||| |     (0: add 1, going across; 1: add 32, going down)
/// |||| +---- Sprite pattern table address for 8x8 sprites
/// ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
/// |||+------ Background pattern table address (0: $0000; 1: $1000)
/// ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
/// |+-------- PPU master/slave select
/// |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
/// +--------- Generate an NMI at the start of the
///            vertical blanking interval (0: off; 1: on)
/// ```
/// Equivalently, bits 1 and 0 are the most significant bit of the scrolling coordinates:
/// ```text
/// 7  bit  0
/// ---- ----
/// .... ..YX
///        ||
///        |+- 1: Add 256 to the X scroll position
///        +-- 1: Add 240 to the Y scroll position
/// ```

#[derive(Default, Debug)]
struct RegPPUCtrl {
    /// `N`: Bits 0-1
    nametable_select: u8,
    /// `I`: bit 2
    increment_mode: bool,
    /// `S`: bit 3
    sprite_tile_select: bool,
    /// `B`: bit 4
    bg_tile_select: bool,
    /// `H`: bit 5
    sprite_height: bool,
    /// `P`: bit 6
    ppu_master_slave: bool,
    /// `V`: bit 7
    nmi_enable: bool,
}

impl From<&RegPPUCtrl> for u8 {
    fn from(_: &RegPPUCtrl) -> Self {
        todo!()
    }
}

impl From<u8> for RegPPUCtrl {
    fn from(b: u8) -> Self {
        Self {
            nametable_select: b & 0b_0000_0011,
            increment_mode: (b & 0b_0000_0100) != 0,
            sprite_tile_select: (b & 0b_0000_1000) != 0,
            bg_tile_select: (b & 0b_0001_0000) != 0,
            sprite_height: (b & 0b_0010_0000) != 0,
            ppu_master_slave: (b & 0b_0100_0000) != 0,
            nmi_enable: (b & 0b_1000_0000) != 0,
        }
    }
}

/// ```text
/// 7  bit  0
/// ---- ----
/// BGRs bMmG
/// |||| ||||
/// |||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
/// |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
/// |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
/// |||| +---- 1: Show background
/// |||+------ 1: Show sprites
/// ||+------- Emphasize red (green on PAL/Dendy)
/// |+-------- Emphasize green (red on PAL/Dendy)
/// +--------- Emphasize blue
/// ```
#[derive(Default, Debug)]
struct RegPPUMask {
    /// `G`: Bit 0
    greyscale: bool,
    /// `m`: Bit 1
    bg_left_col_enable: bool,
    /// `M`: Bit 2
    sprite_left_col_enable: bool,
    /// `b`: Bit 3
    bg_enable: bool,
    /// `s`: Bit 4
    sprite_enable: bool,
    /// `R`: Bit 5 (Color Emphasis)
    ce_r: bool,
    /// `G`: Bit 6 (Color Emphasis)
    ce_g: bool,
    /// `B`: Bit 5-7 (Color Emphasis)
    ce_b: bool,
}

impl From<&RegPPUMask> for u8 {
    fn from(val: &RegPPUMask) -> Self {
        todo!()
    }
}

impl From<u8> for RegPPUMask {
    fn from(b: u8) -> Self {
        Self {
            greyscale: (b & 0b_0000_0001) != 0,
            bg_left_col_enable: (b & 0b_0000_0010) != 0,
            sprite_left_col_enable: (b & 0b_0000_0100) != 0,
            bg_enable: (b & 0b_0000_1000) != 0,
            sprite_enable: (b & 0b_0001_0000) != 0,
            ce_r: (b & 0b_0010_0000) != 0,
            ce_g: (b & 0b_0100_0000) != 0,
            ce_b: (b & 0b_1000_0000) != 0,

        }
    }
}

/// Read resets write pair to `$2005`, `$2006`.
/// ```text
/// 7  bit  0
/// ---- ----
/// VSO. ....
/// |||| ||||
/// |||+-++++- Least significant bits previously written into a PPU register
/// |||        (due to register not being updated for this address)
/// ||+------- Sprite overflow. The intent was for this flag to be set
/// ||         whenever more than eight sprites appear on a scanline, but a
/// ||         hardware bug causes the actual behavior to be more complicated
/// ||         and generate false positives as well as false negatives; see
/// ||         PPU sprite evaluation. This flag is set during sprite
/// ||         evaluation and cleared at dot 1 (the second dot) of the
/// ||         pre-render line.
/// |+-------- Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
/// |          a nonzero background pixel; cleared at dot 1 of the pre-render
/// |          line.  Used for raster timing.
/// +--------- Vertical blank has started (0: not in vblank; 1: in vblank).
///            Set at dot 1 of line 241 (the line *after* the post-render
///            line); cleared after reading $2002 and at dot 1 of the
///            pre-render line.
/// ```
#[derive(Debug)]
struct RegPPUStatus {
    /// `O`: Bit 5
    overflow: bool,
    /// `S`: Bit 6
    sprite0_hit: bool,
    /// `V`: Bit 7
    vblank: bool,
}

impl Default for RegPPUStatus {
    fn default() -> Self {
        Self { overflow: Default::default(), sprite0_hit: Default::default(), vblank: true }
    }
}

impl From<&RegPPUStatus> for u8 {
    fn from(val: &RegPPUStatus) -> Self {
        {
            ((val.overflow      as u8) << 5) |
            ((val.sprite0_hit   as u8) << 6) |
            ((val.vblank        as u8) << 7)
        }
    }
}

#[derive(Debug, Default)]
struct PPUScroll {
    pub x: u8,
    pub y: u8,
    counter: usize,
}

impl PPUScroll {
    pub fn store(&mut self, b: u8) {
        self.counter = (self.counter + 1) % 2;

        match self.counter {
            0 => self.x = b,
            1 => self.y = b,
            _ => unreachable!()
        };

        self.counter += 1;
    }
}

#[derive(Debug, Default)]
struct PPUAddress{
    pub value: u16,
    counter: u8,
}

impl PPUAddress {
    pub fn store(&mut self, b: u8) {
        self.counter = (self.counter + 1) % 2;

        match self.counter {
            0 => self.value |= (b as u16) << 8,
            1 => self.value |= b as u16,
            _ => unreachable!()
        };

        self.counter += 1;
    }
}