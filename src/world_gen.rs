// world_gen.rs
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use::crate::biomes::Biome;
use::crate::chunk::{Chunk, CHUNK_SIZE, TILE_COUNT, BASE_SIZE};

const water = 1;
const cliff = 2;


pub fn generate_chunk(x_chunk: i32, y_chunk: i:32, biome_: &Biome, seed: u32) -> Chunk {
    

    let mut chunk = Chunk {
        chunk_x: x_chunk,
        chunk_y: y_chunk, 
        biome: biome_,
        tile_layer: [0; TILE_COUNT],
        base_layer: [0; BASE_SIZE],
    }

    let moisture_fbm = Fbm::<Perlin>::new(seed)
        .set_octaves(biome.water_octaves)
        .set_frequency(biome.cliff_frequency);

    let elevation_fbm = Fbm::<Perlin>::new(seed.wrapping_add(100))
        .set_octaves(biome.cliff_octaves)
        .set_frequency(biome.cliff_frequency);

    let global_scale = 0.03;
    

    for x in 0..BASE_SIZE {
        for y in 0..BASE_SIZE {
            let global_x = ((x_chunk * CHUNK_SIZE as i32) + x as i32) as f64 * global_scale;
            let global_y = ((y_chunk * CHUNK_SIZE as i32) + y as i32) as f64 * global_scale;
            
            //  For parsing normal chunk array
            let idx = y * BASE_SIZE + x;

            //  Check Water
            if moisture_fbm.get([global_x, global_y]) < biome.water_threshold {
                chunk.base_layer[idx] = water;
                continue;
            }

            //   If not water, Check elevation
            let elevation_val = elevation_fbm.get([global_x, global_y]);
            if elevation_val > biome.cliff_threshold {

                let overflow = elevation_val - biome.cliff_threshold;
                let calculated_layer = (overflow / biome.cliff_spacing).floor() as u8 + 1;

                let final_layer = setd::cmp::min(calculated_layer, biome.cliff_layers);

                chunk.base_layer[idx] = cliff;

            }

        }
    }

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {

            chunk.tile_water();
            chunk.tile_cliffs();
        }
    }

    chunk
}