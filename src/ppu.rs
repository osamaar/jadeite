#![allow(unused_variables, dead_code)]

use crate::Bus;
// #![allow(non_snake_case)]

#[derive(Default, Debug)]
pub struct Ppu {
    ppu_ctrl: RegPPUCtrl,
    ppu_mask: RegPPUMask,
    ppu_status: RegPPUStatus,
    oam_addr: u8,
    ppu_scroll: u8,
    ppu_addr: u8,
    ppu_data: u8,
    oam_dma: u8,

}

impl Ppu {
    // fn new() -> Self {
    //     Self {

    //     }
    // }

    pub fn step(&mut self, bus: &mut Bus) {

    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x2000 => (&self.ppu_ctrl).into(),
            0x2001 => (&self.ppu_mask).into(),
            0x2002 => (&self.ppu_status).into(),
            0x2003 => self.oam_addr,
            0x2004 => self.ppu_data,
            0x2005 => self.ppu_scroll,
            0x2006 => self.ppu_addr,
            0x2007 => self.ppu_data,
            _ => unreachable!()
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x2000 => {todo!()},
            0x2001 => {todo!()},
            0x2002 => {todo!()},
            0x2003 => {todo!()},
            0x2004 => {todo!()},
            0x2005 => {todo!()},
            0x2006 => {todo!()},
            0x2007 => {todo!()},
            _ => unreachable!()
        }

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
#[derive(Default, Debug)]
struct RegPPUStatus {
    /// `O`: Bit 5
    overflow: bool,
    /// `S`: Bit 6
    sprite0_hit: bool,
    /// `V`: Bit 7
    vblank: bool,
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
