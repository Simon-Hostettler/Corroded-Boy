pub struct ppu {
    vram: [u8; 0x4000],
    oam: [u8; 0xA0],
    lcd_en: bool,
    win_tile_area: bool,
    win_en: bool,
    bg_win_tile_area: bool,
    bg_tile_area: bool,
    obj_size: bool,
    obj_en: bool,
    bg_win_en: bool,
    cur_vram_bank: u8,
}

impl ppu {
    pub fn new() -> ppu {
        ppu {
            vram: [0; 0x4000],
            oam: [0; 0xA0],
            lcd_en: false,
            win_tile_area: false,
            win_en: false,
            bg_win_tile_area: false,
            bg_tile_area: false,
            obj_size: false,
            obj_en: false,
            bg_win_en: false,
            cur_vram_bank: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => {
                self.vram[(self.cur_vram_bank as usize * 0x2000) | (addr as usize - 0x8000)]
            }
            0xFE00..=0xFE9F => self.oam[addr as usize - 0xFE00],
            0xFF40 => {
                let mut byte = 0x00;
                byte |= if self.lcd_en { 0x1 << 7 } else { 0 };
                byte |= if self.win_tile_area { 0x1 << 6 } else { 0 };
                byte |= if self.win_en { 0x1 << 5 } else { 0 };
                byte |= if self.bg_win_tile_area { 0x1 << 4 } else { 0 };
                byte |= if self.bg_tile_area { 0x1 << 3 } else { 0 };
                byte |= if self.obj_size { 0x1 << 2 } else { 0 };
                byte |= if self.obj_en { 0x1 << 1 } else { 0 };
                byte |= if self.bg_win_en { 0x1 } else { 0 };
                byte
            }
            _ => panic!("Not a valid ppu memory area"),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9FFF => {
                self.vram[(self.cur_vram_bank as usize * 0x2000) | (addr as usize - 0x8000)] =
                    value;
            }
            0xFE00..=0xFE9F => self.oam[addr as usize - 0xFE00] = value,
            0xFF40 => {
                self.lcd_en = value & (0x1 << 7) != 0;
                self.win_tile_area = value & (0x1 << 6) != 0;
                self.win_en = value & (0x1 << 5) != 0;
                self.bg_win_tile_area = value & (0x1 << 4) != 0;
                self.bg_tile_area = value & (0x1 << 3) != 0;
                self.obj_size = value & (0x1 << 2) != 0;
                self.obj_en = value & (0x1 << 1) != 0;
                self.bg_win_en = value & (0x1) != 0;
            }
            _ => panic!("Not a valid ppu memory area"),
        }
    }
}
