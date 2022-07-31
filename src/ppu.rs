#![allow(unused_variables, dead_code)]

use std::{fmt::Debug, borrow::Borrow};

use crate::{palette::Palette, Cart, PpuBus};
// #![allow(non_snake_case)]

const OUTPUT_W: usize = 256;
const OUTPUT_H: usize = 240;
const OUTPUT_SIZE: usize = OUTPUT_W * OUTPUT_H;

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

    //vram: [u8; 2048],
    color_palette: Palette,
    
    clock_count: usize,
    scanline: usize,
    scanline_cycle: usize,

    pub nmi_signal: bool,
    pub output: Box<[Pixel]>,
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
            // vram: [0; 2048],
            color_palette,
            clock_count: 0,
            scanline: 261,
            scanline_cycle: 0,
            nmi_signal: false,
            output: vec![Pixel::new(0x22, 0x22, 0x22, 0xff); OUTPUT_SIZE].into_boxed_slice(),
        }
    }
        
    pub fn step(&mut self, ppu_bus: &PpuBus) {
        // let cart = ppu_bus.cart.as_ref().unwrap();

        match self.scanline {
            241 => {
                if self.scanline_cycle == 1 {
                    self.draw_debug(ppu_bus);
                    //self.draw_bg(ppu_bus);
                    self.ppu_status.vblank = true;
                    self.nmi_signal = self.ppu_status.vblank && self.ppu_ctrl.nmi_enable;
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
        self.clock_count %= 262*341;

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
                let result = self.ppu_status.as_ref().into();
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
        let addr = addr & 0xfff7;

        match addr {
            // PPUCTRL
            0x2000 => self.ppu_ctrl = value.into(),

            // PPUMASK
            0x2001 => self.ppu_mask = value.into(),

            // PPUSTATUS
            0x2002 => {todo!()},

            // OAMADDR
            0x2003 => {todo!()},

            // OAMDATA
            0x2004 => {todo!()},

            // PPUSCROLL
            0x2005 => self.ppu_scroll.store(value),

            // PPUADDR
            0x2006 => self.ppu_addr.store(value),

            // OAMDMA
            0x2007 => {
                // TODO
            },

            _ => unreachable!()
        }

    }

    fn draw_debug(&mut self, ppu_bus: &PpuBus) {
        let mut tile_data = [0; 64];
        for ty in 0..16 {
            for tx in 0..16 {
                // left
                for i in 0..16 {
                    tile_data[i] = ppu_bus.read(((ty * 16 + tx) * 16 + i) as u16);
                }

                draw_tile(
                    &tile_data,
                    &mut self.output,
                    (tx, ty),
                    (0, 0),
                    &self.color_palette,
                );

                // right
                for i in 0..16 {
                    let addr = (((ty * 16 + tx) * 16 + i) | 0x1000) as u16;
                    tile_data[i] =
                        ppu_bus.read(addr);
                }

                draw_tile(
                    &tile_data,
                    &mut self.output,
                    (tx, ty),
                    (128, 0),
                    &self.color_palette,
                );
            }
        }

        draw_palette(&mut self.output, &self.color_palette);

        // println!("{:#x?}", self.output
        //     .iter()
        //     .map(
        //         |x| (x.r as u32) << 3 | (x.g as u32) << 2 | (x.b as u32) << 1 | x.a as u32
        //     )
        //     .collect::<Vec<u32>>()
        // );
    }

    /// Read nametable byte, attribute table byte.
    /// Fetch pattern table byte low/hi.
    fn draw_bg(&mut self, ppu_bus: &PpuBus) {
        // let mut v = 0;

        for row in 0..OUTPUT_H {
            for col in (0..OUTPUT_W).step_by(8) {
                let coarse_x = (col/8) as u16;
                let coarse_y = (row/8) as u16;
                let fine_y = (row%8) as u16;
                let pattern_table_half = self.ppu_ctrl.bg_table_select as u16;

                // v := Current VRAM address (in name table):
                // yyy NN YYYYY XXXXX
                // ||| || ||||| +++++-- coarse X scroll
                // ||| || +++++-------- coarse Y scroll
                // ||| ++-------------- nametable select
                // +++----------------- fine Y scroll
                let v = coarse_x as u16
                    | (coarse_y << 5)
                    | (self.ppu_ctrl.nametable_select as u16) << 10
                    | fine_y << 12;

                // Read nametable byte.
                let tile_addr = ppu_bus.read(
                    0x2000 | (v & 0x0fff)
                ) as u16;

                // Fetch from attribute table.
                let attr_addr = ppu_bus.read(
                    0x23C0
                    | (v & 0x0C00)
                    | ((v >> 4) & 0x38)
                    | ((v >> 2) & 0x07)
                ) as u16;

                // Read pattern table
                // DCBA98 76543210
                // ---------------
                // 0HRRRR CCCCPTTT
                // |||||| |||||+++- T: Fine Y offset, the row number within a tile
                // |||||| ||||+---- P: Bit plane (0: "lower"; 1: "upper")
                // |||||| ++++----- C: Tile column
                // ||++++---------- R: Tile row
                // |+-------------- H: Half of pattern table (0: "left"; 1: "right")
                // +--------------- 0: Pattern table is at $0000-$1FFF
                let tile_sliver_lo = ppu_bus.read(
                    fine_y
                    | 0 << 3
                    | tile_addr << 4
                    | pattern_table_half << 12
                );

                let tile_sliver_hi = ppu_bus.read(
                    fine_y
                    | 1 << 3
                    | tile_addr << 4
                    | pattern_table_half << 12
                );

                // println!("hi|lo = {:08b}|{:08b}", tile_sliver_hi, tile_sliver_lo);
                
                let attr = ppu_bus.read(attr_addr);

                for i in 0..8 {
                    let bit_lo = (tile_sliver_lo >> (7-i)) & 1;
                    let bit_hi = (tile_sliver_hi >> (7-i)) & 1;
                    let color_idx = (bit_hi << 1) & bit_lo;
                    let shift = (
                        (coarse_y%4 & 0b10)
                        | (coarse_x as u16 % & 0b10) >> 1
                    ) << 1;
                    let palette_idx = (attr as u16 >> shift) & 3;
                    let final_idx = palette_idx << 2 + color_idx;
                    let color = self.color_palette[final_idx as usize];
                    let pixel = Pixel::new(color.r, color.g, color.b, 0xff);
                    self.output[col+i+row*OUTPUT_W] = pixel;
                }
            }
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
    sprite_table_select: bool,
    /// `B`: bit 4
    bg_table_select: bool,
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
            sprite_table_select: (b & 0b_0000_1000) != 0,
            bg_table_select: (b & 0b_0001_0000) != 0,
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

impl AsRef<RegPPUStatus> for RegPPUStatus {
    fn as_ref(&self) -> &RegPPUStatus {
        &self
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

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub fn zeros() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0xff,
        }
    }
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

fn draw_tile(
    tile_data: &[u8],
    output: &mut Box<[Pixel]>,
    tilexy: (usize, usize),
    oxy: (usize, usize),
    palette: &Palette,
) {
    let px = tilexy.0 * 8 + oxy.0;
    let py = tilexy.1 * 8 + oxy.1;

    for y in 0..8 {
        for x in 0..8 {
            let byte0 = tile_data[y];
            let byte1 = tile_data[y + 8];

            let shift = 7 - x;
            let bit0 = 1 & (byte0 >> shift);
            let bit1 = 1 & (byte1 >> shift);
            let value = bit0 | (bit1 << 1);

            let c = palette[value as usize];
            output[(py + y) * OUTPUT_W + (px + x)] = Pixel::new(c.r, c.g, c.b, 0xff);
        }
    }
}

fn draw_palette(output: &mut Box<[Pixel]>, palette: &Palette) {
    const SAMPLE_W: usize = 16;
    const SAMPLE_H: usize = 8;

    let total_x = 16 * SAMPLE_W;
    let total_y = 4 * SAMPLE_H;

    let ox = OUTPUT_W - total_x;
    let oy = OUTPUT_H - total_y;

    for y in 0..total_y {
        for x in 0..total_x {
            let c = palette[y / SAMPLE_H * 16 + x / SAMPLE_W];
            // output[oy+y*OUTPUT_W + ox+x] = Pixel::new(c.r, c.g, c.b, 0xff);
            output[(oy + y) * OUTPUT_W + ox + x] = Pixel::new(c.r, c.g, c.b, 0xff);
        }
    }
}
