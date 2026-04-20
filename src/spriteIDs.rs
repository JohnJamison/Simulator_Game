// sprite_ids.rs (or at the top of your world_gen.rs)
pub const TILE_SIZE: f32 = 32.0;

#[inline(always)]
pub fn get_sprite_uv(sprite_id: u16) -> (f32, f32) {

    let col = (sprite_id % 10) as f32;
    let row = (sprite_id / 10) as f32;
    (col, row)
}

// --- GRASS ---
pub const GRASS: u16 = 11;

// --- CLIFFS ---
// You will define similar IDs for your 20 cliff variants here...
pub const CLIFF_T: u16 = 1;
pub const CLIFF_B: u16 = 21;
pub const CLIFF_L: u16 = 10;
pub const CLIFF_R: u16 = 12;
pub const CLIFF_TL: u16 = 0;
pub const CLIFF_TR: u16 = 2;
pub const CLIFF_BL: u16 = 20;
pub const CLIFF_BR: u16 = 22;
pub const CLIFF_INV_TL: u16 = 3;
pub const CLIFF_INV_TR: u16 = 4;
pub const CLIFF_INV_BL: u16 = 13;
pub const CLIFF_INV_BR: u16 = 14;
pub const CLIFF_DL: u16 = 23;
pub const CLIFF_DR: u16 = 24;


// --- WATER (20 Tiles) ---
pub const WATER_C: u16 = 41;
pub const WATER_T: u16 = 31;
pub const WATER_B: u16 = 51;
pub const WATER_L: u16 = 40;
pub const WATER_R: u16 = 42;
pub const WATER_TL: u16 = 30;
pub const WATER_TR: u16 = 32;
pub const WATER_BL: u16 = 50;
pub const WATER_BR: u16 = 52;
pub const WATER_INV_TL: u16 = 33;
pub const WATER_INV_TR: u16 = 34;
pub const WATER_INV_BL: u16 = 43;
pub const WATER_INV_BR: u16 = 44;
pub const WATER_DL: u16 = 53;
pub const WATER_DR: u16 = 54;
