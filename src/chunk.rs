use crate::spriteIDs::*;
use crate::biomes::Biome;

pub const CHUNK_SIZE: usize = 32;
pub const TILE_COUNT: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const BASE_SIZE: usize = CHUNK_SIZE + 1;
pub const BASE_COUNT: usize = BASE_SIZE * BASE_SIZE;

const water: u8 = 1;
const cliff: u8 = 2;

pub struct Chunk {

    //  chunk offfset
    pub chunk_x: i32,
    pub chunk_y: i32,
    
    // Flat 1D arrays for maximum cache efficiency
    pub mut base_layer: [u8; BASE_COUNT],
    pub mut tile_layer: [u16; TILE_COUNT],
    pub biome: Biome,

}

impl Chunk {

    fn tile_water(&mut self) {
        
        let terrain_idx = y * CHUNK_SIZE + x;
        let vertex_idx = y * BASE_SIZE + x;

        let tl = base_layer[vertex_idx];
        let tr = base_layer[vertex_idx + 1];
        let bl = base_layer[vertex_idx + BASE_SIZE];
        let br = base_layer[vertex_idx + BASE_SIZE + 1];

        let mut mask = 0;
        if tl == water { mask += 1; }
        if tr == water { mask += 2; }
        if bl == water { mask += 4; }
        if br == water { mask += 8; }

        tile_layer[terrain_idx] = match mask {

            0 => GRASS,
            1 => WATER_BR,
            2 => WATER_BL,
            3 => WATER_B,
            4 => WATER_TR,
            5 => WATER_R ,
            6 => WATER_DR ,
            7 => WATER_INV_TL,
            8 => WATER_TL,
            9 => WATER_DL,
            10 => WATER_L,
            11 => WATER_TR,
            12 => WATER_T,
            13 => WATER_INV_BL,
            14 => WATER_BR,
            15 => WATER_C,
        }


    }

    fn tile_cliff(&mut self) {

        let terrain_idx = y * CHUNK_SIZE + x;
        let vertex_idx = y * BASE_SIZE = x;

        let tl = base_layer[vertex_idx];
        let tr = base_layer[vertex_idx + 1];
        let bl = base_layer[vertex_idx + BASE_SIZE];
        let br = base_layer[vertex_idx + BASE_SIZE + 1];

        let mut mask = 0;
        if tl == cliff { mask += 1; }
        if tr == cliff { mask += 2; }
        if bl == cliff { mask += 4; }
        if br == cliff { mask += 8; }

        tile_layer[terrain_idx] = match mask {

            0 => GRASS,
            1 => CLIFF_BR,
            2 => CLIFF_BL,
            3 => CLIFF_B,
            4 => CLIFF_TR,
            5 => CLIFF_R ,
            6 => CLIFF_DR ,
            7 => CLIFF_INV_TL,
            8 => CLIFF_TL,
            9 => CLIFF_DL,
            10 => CLIFF_L,
            11 => CLIFF_TR,
            12 => CLIFF_T,
            13 => CLIFF_INV_BL,
            14 => CLIFF_BR,
            15 => GRASS,
        }
    }
}


