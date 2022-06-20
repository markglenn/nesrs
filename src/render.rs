use crate::ppu::Ppu;

use crate::ppu::frame::Frame;
use crate::ppu::palette;

fn bg_pallette(ppu: &Ppu, tile_column: usize, tile_row: usize) -> [u8; 4] {
    let attr_table_idx = tile_row / 4 * 8 + tile_column / 4;

    // note: still using hardcoded first nametable
    let attr_byte = ppu.vram[0x3c0 + attr_table_idx];

    let pallet_idx = match (tile_column % 4 / 2, tile_row % 4 / 2) {
        (0, 0) => attr_byte & 0b11,
        (1, 0) => (attr_byte >> 2) & 0b11,
        (0, 1) => (attr_byte >> 4) & 0b11,
        (1, 1) => (attr_byte >> 6) & 0b11,
        _ => unimplemented!(),
    };

    let pallete_start: usize = 1 + (pallet_idx as usize) * 4;
    [
        ppu.palette_table[0],
        ppu.palette_table[pallete_start],
        ppu.palette_table[pallete_start + 1],
        ppu.palette_table[pallete_start + 2],
    ]
}

fn sprite_palette(ppu: &Ppu, pallete_idx: u8) -> [u8; 4] {
    let start = 0x11 + (pallete_idx * 4) as usize;
    [
        0,
        ppu.palette_table[start],
        ppu.palette_table[start + 1],
        ppu.palette_table[start + 2],
    ]
}

fn render_background(ppu: &Ppu, frame: &mut Frame) {
    let bank = ppu.ctrl.background_pattern_address();

    if ppu.chr_rom.len() == 0 {
        return;
    }

    for i in 0..0x3c0 {
        let tile = ppu.vram[i] as usize;
        let tile_column = i % 32;
        let tile_row = i / 32;
        let tile = &ppu.chr_rom[bank + tile * 16..=bank + tile * 16 + 15];
        let palette = bg_pallette(ppu, tile_column, tile_row);

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & lower) << 1 | (1 & upper);
                upper = upper >> 1;
                lower = lower >> 1;
                let rgb = match value {
                    0 => palette::SYSTEM_PALETTE[ppu.palette_table[0] as usize],
                    1 => palette::SYSTEM_PALETTE[palette[1] as usize],
                    2 => palette::SYSTEM_PALETTE[palette[2] as usize],
                    3 => palette::SYSTEM_PALETTE[palette[3] as usize],
                    _ => unreachable!(),
                };
                frame.set_pixel(tile_column * 8 + x, tile_row * 8 + y, rgb)
            }
        }
    }
}

fn render_sprite(ppu: &Ppu, frame: &mut Frame, i: usize) {
    if ppu.chr_rom.len() == 0 {
        return;
    }

    let oam_tile = ppu.primary_oam.tile(i);

    let sprite_palette = sprite_palette(ppu, oam_tile.palette_idx());

    let bank = ppu.ctrl.sprite_pattern_address();

    let offset = bank + oam_tile.tile_idx() * 16;
    let tile = &ppu.chr_rom[offset..=offset + 15];

    for y in 0..=7 {
        let mut upper = tile[y];
        let mut lower = tile[y + 8];

        for x in (0..=7).rev() {
            let value = (1 & lower) << 1 | (1 & upper);
            upper = upper >> 1;
            lower = lower >> 1;

            let rgb = match value {
                0 => continue, // skip pixel
                1 => palette::SYSTEM_PALETTE[sprite_palette[1] as usize],
                2 => palette::SYSTEM_PALETTE[sprite_palette[2] as usize],
                3 => palette::SYSTEM_PALETTE[sprite_palette[3] as usize],
                _ => unimplemented!(),
            };

            let actual_x = match oam_tile.flip_horizontal() {
                false => oam_tile.x_pos() as usize + x,
                true => oam_tile.x_pos() as usize + 7 - x,
            };

            let actual_y = match oam_tile.flip_vertical() {
                false => oam_tile.y_pos() + y,
                true => oam_tile.y_pos() + 7 - y,
            };

            frame.set_pixel(actual_x, actual_y, rgb);
        }
    }
}

pub fn render(ppu: &Ppu, frame: &mut Frame) {
    render_background(ppu, frame);

    for i in (0..64).rev() {
        render_sprite(ppu, frame, i);
    }
}
