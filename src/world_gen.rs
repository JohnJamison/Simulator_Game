// ==========================================================================================================================================
//                                                              world_gen.rs                                                               ||
// ========================================================================================================================================||
//  File responsible for generating authentic terrain for the overworld. Each time the game is loaded, world gen is responsible for        ||
//  generating the overworld. Currently generates bodies of water and cliffs using the Perlin Noise method. Generations are pipelined to   ||
//  ensure no overlapping of tile (like cliffs and water).                                                                                 ||
//                                                                                                                                         ||
//                                  Functions:                                                                                             ||
//  1.  is_active_safe():                                                                                                                  ||
//  2.  is_empty_safe():                                                                                                                   ||
//  3.  cleanup_terrain_layer():    Cleans terrain after initial perlin generation                                                         ||
//  4.  get_water_variant():        Determines specific water Tile identity                                                                ||
//  5.  get_cliff_variant():        Determines specific cliff Tile identity                                                                ||
//  6.  generate_wfc_grid():        Responsible for generating world                                                                       ||
//                                                                                                                                         ||
// ==========================================================================================================================================


use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
use crate::environment::Tile;
use crate::environment::water::WaterVariant;
use crate::environment::cliffs::CliffVariant; // Make sure this path matches your project!
use crate::WORLD_SIZE;

// =====================================================================
//   HELPERS: SAFE GRID CHECKING
// =====================================================================

//  ----------  is_active_safe  ----------
//  Sets the water/cliff binary placement grid to False if a cell is out of bounds
fn is_active_safe(grid: &[[bool; WORLD_SIZE]; WORLD_SIZE], x: isize, y: isize) -> bool {
    if x < 0 || x >= WORLD_SIZE as isize || y < 0 || y >= WORLD_SIZE as isize { return false; }
    grid[x as usize][y as usize]
}

//  ----------  is_empty_safe  ----------
//  If out of bounds, return true, otherwise invert the boolean value
fn is_empty_safe(grid: &[[bool; WORLD_SIZE]; WORLD_SIZE], x: isize, y: isize) -> bool {
    if x < 0 || x >= WORLD_SIZE as isize || y < 0 || y >= WORLD_SIZE as isize { return true; }
    !grid[x as usize][y as usize]
}

// ===============================================================================
//                            cleanup_terrain_layer
// ===============================================================================
// Morphological cleanup engine. Runs the 4 strict rules required for a the 
// auto-tiler to work. Ensures that the following will not be rendered:
//  1.  1-tile thick peninsulas or floating islands
//  2.  Diagonal connections and staircase shapes
//  3.  Tiny puddles smaller than a given size

fn cleanup_terrain_layer(grid: &mut [[bool; WORLD_SIZE]; WORLD_SIZE], min_pond_size: usize) {
    
    let mut stable = false;
    let mut iterations = 0;

    //  Since the pruning of one-tile can lead to other issues, we run the loop
    //  every time we make a change until no changes are made / needed. The loop
    //  runs through every cell in the map ensuring that the entire map is
    //  covered each generation, in case one already checked tile becomes affected.
    //  We must run ALL passes inside this single loop so they don't fight!
    while !stable && iterations < 15 {
        stable = true;
        iterations += 1;

        // Pass 1: Strict 2x2 Pruning
        let mut next = *grid;
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                //  Check for various conditions such as if a tile is top-right 'tl'...
                if grid[x][y] {
                    let (xi, yi) = (x as isize, y as isize);
                    let is_bl = is_active_safe(grid, xi, yi) && is_active_safe(grid, xi+1, yi) && is_active_safe(grid, xi, yi+1) && is_active_safe(grid, xi+1, yi+1);
                    let is_br = is_active_safe(grid, xi, yi) && is_active_safe(grid, xi-1, yi) && is_active_safe(grid, xi-1, yi+1) && is_active_safe(grid, xi, yi+1);
                    let is_tl = is_active_safe(grid, xi, yi) && is_active_safe(grid, xi+1, yi-1) && is_active_safe(grid, xi, yi-1) && is_active_safe(grid, xi+1, yi);
                    let is_tr = is_active_safe(grid, xi-1, yi-1) && is_active_safe(grid, xi, yi-1) && is_active_safe(grid, xi-1, yi) && is_active_safe(grid, xi, yi);
                    //  if it isn't a part of any 2x2 square, delete it and flag instability
                    if !(is_tl || is_tr || is_bl || is_br) { next[x][y] = false; stable = false; }
                }
            }
        }
        *grid = next;

        // Pass 2: Land Pruning (Prevents double-inverse glitches)
        let mut next2 = *grid;
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                if !grid[x][y] { 
                    let (xi, yi) = (x as isize, y as isize);
                    let is_tl = is_empty_safe(grid, xi, yi) && is_empty_safe(grid, xi+1, yi) && is_empty_safe(grid, xi, yi+1) && is_empty_safe(grid, xi+1, yi+1);
                    let is_tr = is_empty_safe(grid, xi-1, yi) && is_empty_safe(grid, xi, yi) && is_empty_safe(grid, xi-1, yi+1) && is_empty_safe(grid, xi, yi+1);
                    let is_bl = is_empty_safe(grid, xi, yi-1) && is_empty_safe(grid, xi+1, yi-1) && is_empty_safe(grid, xi, yi) && is_empty_safe(grid, xi+1, yi);
                    let is_br = is_empty_safe(grid, xi-1, yi-1) && is_empty_safe(grid, xi, yi-1) && is_empty_safe(grid, xi-1, yi) && is_empty_safe(grid, xi, yi);
                    if !(is_tl || is_tr || is_bl || is_br) { next2[x][y] = true; stable = false; }
                }
            }
        }
        *grid = next2;

        // Pass 3: Fat Corners (Fixes 1-tile diagonal overlaps)
        let mut next3 = *grid;
        //  for every cell
        for x in 0..(WORLD_SIZE - 1) {
            for y in 0..(WORLD_SIZE - 1) {
                //  check a 2x2 area, x is current cell
                //  [0  0]  and tells us if it is a 
                //  [x  0]  water/cliff  tile or not
                let w_tl = grid[x][y+1]; let w_tr = grid[x+1][y+1];
                let w_bl = grid[x][y]; let w_br = grid[x+1][y];
                let count = (w_tl as u8) + (w_tr as u8) + (w_bl as u8) + (w_br as u8);

                //  if  3 of these cells are water/cliff/...
                if count == 3 {

                    //  Set (px,py) equal to the cell where the inverse corner
                    //  tile should be. Determined by being diagonally opposed
                    //  to the cell that is the outlier
                    let (px, py) = if !w_tl { (x+1, y) } else if !w_tr { (x, y) } else if !w_bl { (x+1, y+1) } else { (x, y+1) };
                    let (pxi, pyi) = (px as isize, py as isize);

                    //  Ensure cell isn't out of bounds
                    let n = is_active_safe(grid, pxi, pyi-1); let s = is_active_safe(grid, pxi, pyi+1);
                    let e = is_active_safe(grid, pxi+1, pyi); let w = is_active_safe(grid, pxi-1, pyi);

                    //  If any are false, change the outlier to true.
                    if !(n && s && e && w) {
                        if !w_tl { next3[x][y+1] = true; } else if !w_tr { next3[x+1][y+1] = true; } else if !w_bl { next3[x][y] = true; } else { next3[x+1][y] = true; }
                        stable = false;
                    }
                }
            }
        }
        *grid = next3;
    }

    // Pass 4: Flood Fill
    let mut visited = [[false; WORLD_SIZE]; WORLD_SIZE];
    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            
            //  If cell is Water/Cliff and is unvisited
            if grid[x][y] && !visited[x][y] {
                let mut stack = vec![(x, y)];
                let mut current_cluster = Vec::new();
                while let Some((cx, cy)) = stack.pop() {
                    if visited[cx][cy] || !grid[cx][cy] { continue; }
                    visited[cx][cy] = true;
                    current_cluster.push((cx, cy));
                    if cy > 0 { stack.push((cx, cy - 1)); }
                    if cy < WORLD_SIZE - 1 { stack.push((cx, cy + 1)); }
                    if cx > 0 { stack.push((cx - 1, cy)); }
                    if cx < WORLD_SIZE - 1 { stack.push((cx + 1, cy)); }
                }
                if current_cluster.len() < min_pond_size {
                    for (px, py) in current_cluster { grid[px][py] = false; }
                }
            }
        }
    }
}



// ===============================================================================
//                                     Auto Tilers
// ===============================================================================
//  Functions responsible for determining what specific water and cliff tiles
//  should be used for a given tile cell. Used in the final procedural generation
//  phase of generate_wfc_grid(). Phase only knows if a cell is water, grass or
//  cliff. These functions determine what specific Water or Cliff tile should be
//  generated based on the surrounding tiles. 

//  ----------  Water Variant ---------

fn get_water_variant(grid: &[[bool; WORLD_SIZE]; WORLD_SIZE], x: usize, y: usize) -> WaterVariant {
    
    //  Get all 8 surrounding cells and set any 'off-map' tiles to false
    let (xi, yi) = (x as isize, y as isize);
    let n = is_active_safe(grid, xi, yi - 1); let s = is_active_safe(grid, xi, yi + 1);
    let e = is_active_safe(grid, xi + 1, yi); let w = is_active_safe(grid, xi - 1, yi);
    let nw = is_active_safe(grid, xi - 1, yi - 1); let ne = is_active_safe(grid, xi + 1, yi - 1);
    let sw = is_active_safe(grid, xi - 1, yi + 1); let se = is_active_safe(grid, xi + 1, yi + 1);

    //  determine cell identity based on direction
    let mut mask = 0;
    if n { mask += 1; } if s { mask += 2; } if e { mask += 4; } if w { mask += 8; }

    //  return tile based on identity
    match mask {
        1 => WaterVariant::Bottom, 2 => WaterVariant::Top, 4 => WaterVariant::Left, 8 => WaterVariant::Right,        
        5 => WaterVariant::BottomLeft, 6 => WaterVariant::TopLeft, 9 => WaterVariant::BottomRight, 10 => WaterVariant::TopRight,    
        7 => WaterVariant::Left, 11 => WaterVariant::Right, 13 => WaterVariant::Bottom, 14 => WaterVariant::Top,         
        15 => {
            if !nw { WaterVariant::InverseCornerBR } else if !ne { WaterVariant::InverseCornerBL }
            else if !sw { WaterVariant::InverseCornerTR } else if !se { WaterVariant::InverseCornerTL }
            else { WaterVariant::Center }
        }
        _ => WaterVariant::Center, 
    }
}

//  ----------  Cliff Variant ---------

fn get_cliff_variant(grid: &[[bool; WORLD_SIZE]; WORLD_SIZE], x: usize, y: usize) -> CliffVariant {
    let (xi, yi) = (x as isize, y as isize);
    let n = is_active_safe(grid, xi, yi - 1); let s = is_active_safe(grid, xi, yi + 1);
    let e = is_active_safe(grid, xi + 1, yi); let w = is_active_safe(grid, xi - 1, yi);
    let nw = is_active_safe(grid, xi - 1, yi - 1); let ne = is_active_safe(grid, xi + 1, yi - 1);
    let sw = is_active_safe(grid, xi - 1, yi + 1); let se = is_active_safe(grid, xi + 1, yi + 1);

    let mut mask = 0;
    if n { mask += 1; } if s { mask += 2; } if e { mask += 4; } if w { mask += 8; }

    // Cliffs use the exact same bitmask structure as water!
    match mask {
        1 => CliffVariant::Bottom, 2 => CliffVariant::Top, 4 => CliffVariant::Left, 8 => CliffVariant::Right,        
        5 => CliffVariant::BottomLeft, 6 => CliffVariant::TopLeft, 9 => CliffVariant::BottomRight, 10 => CliffVariant::TopRight,    
        7 => CliffVariant::Left, 11 => CliffVariant::Right, 13 => CliffVariant::Bottom, 14 => CliffVariant::Top,         
        15 => {
            if !nw { CliffVariant::InverseCornerBR } else if !ne { CliffVariant::InverseCornerBL }
            else if !sw { CliffVariant::InverseCornerTR } else if !se { CliffVariant::InverseCornerTL }
            else { CliffVariant::Center }
        }
        _ => CliffVariant::Center, 
    }
}



// ===============================================================================
//                          generate_wfc_grid
// ===============================================================================
//  Main generation function that creates and returns a 2D Tile array map.
//  Function uses Perlin Noise method to randomly generate water and cliffs.
//  Method creates unique values for each cell between -1 and 1, and returns
//  values based on noise value. Pipeline used to first determine where water
//  will be placed on the map, and uses output to add cliffs. Cliffs cannot
//  overlay water tiles. Tiles are determined using Boolean maps of world size.

pub fn generate_wfc_grid() -> [[Tile; WORLD_SIZE]; WORLD_SIZE] {
    
    // ---------------------------------------------------------
    //              PHASE 1 - Water Generation
    // ---------------------------------------------------------
    let perlin = Perlin::new(macroquad::rand::rand());
    let mut is_water = [[false; WORLD_SIZE]; WORLD_SIZE];

    //  Zoom on the noise
    let noise_scale = 0.08;     
    
    //  ----------  Generate Noise  ----------
    //  Get the perlin value for every cell and set it to water
    //  if its below -0.3 (35% chance). Noise_scale is mult by
    //  coordinate values to create zoom
    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            if perlin.get([x as f64 * noise_scale, y as f64 * noise_scale]) < -0.3 {
                is_water[x][y] = true;
            }
        }
    }

    //  Don't put water in the center of the world / spawn point
    let middle = WORLD_SIZE as f32 / 2.0;
    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            if ((x as f32 - middle).powi(2) + (y as f32 - middle).powi(2)).sqrt() <= 4.5 {
                is_water[x][y] = false; 
            }
        }
    }

    // ---------- Smoothing process ----------
    //  Smooths out any ugly terrain the Perlin
    //  generated. Runs Twice. First time takes off
    //  sharp edges, second time rounds the ponds out
    for _ in 0..2 {
        let mut next = is_water;

        //  Go thorugh every cell (except edges)...
        for x in 1..(WORLD_SIZE - 1) {
            for y in 1..(WORLD_SIZE - 1) {

                //  look at surrounding 8 tiles 3x3 grid,
                //  and count how many nieghbors are water tiles
                let mut neighbors = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {

                        // don't check self
                        if dx != 0 || dy != 0 {
                            if is_water[(x as isize + dx) as usize][(y as isize + dy) as usize] { neighbors += 1; }
                        }
                    }
                }
                
                //  if the cell isn't water but many of its neighbros are,
                //  make the cell a water tile too. But if its a water tile,
                //  and not many of its neighbros are, then make it a land cell
                if !is_water[x][y] && neighbors >= 5 { next[x][y] = true; }
                else if is_water[x][y] && neighbors <= 3 { next[x][y] = false; }
            }
        }
        is_water = next;
    }

    cleanup_terrain_layer(&mut is_water, 25);



    // ---------------------------------------------------------
    //              PHASE 2 - Cliff Generation
    // ---------------------------------------------------------
    //  Generates cliffs using the same exact perlin noise methods
    //  as the water generation
    
    //  generate new perlin map
    let cliff_perlin = Perlin::new(macroquad::rand::rand() + 100); 
    let mut is_cliff = [[false; WORLD_SIZE]; WORLD_SIZE];
    
    // 1. Lower the scale slightly so the cliff blobs are wider
    let cliff_scale = 0.09; 


    //  ----------  Generate Noise  ----------
    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            if !is_water[x][y] {
                if cliff_perlin.get([x as f64 * cliff_scale, y as f64 * cliff_scale]) > -0.3 {
                    is_cliff[x][y] = true;
                }
            }
        }
    }

    // Carve an even larger safe spawn island for cliffs
    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            if ((x as f32 - middle).powi(2) + (y as f32 - middle).powi(2)).sqrt() <= 6.5 {
                is_cliff[x][y] = false; 
            }
        }
    }

    // ---------- Smoothing process ----------
    for _ in 0..2 {
        let mut next = is_cliff;
        for x in 1..(WORLD_SIZE - 1) {
            for y in 1..(WORLD_SIZE - 1) {
                // CRITICAL: Prevent cliffs from smoothing into water
                if is_water[x][y] { continue; } 

                let mut neighbors = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx != 0 || dy != 0 {
                            if is_cliff[(x as isize + dx) as usize][(y as isize + dy) as usize] { neighbors += 1; }
                        }
                    }
                }
                if !is_cliff[x][y] && neighbors >= 5 { next[x][y] = true; }
                else if is_cliff[x][y] && neighbors <= 3 { next[x][y] = false; }
            }
        }
        is_cliff = next;
    }

    // --- PHASE 2.5: THE GRASS BUFFER ---
    // Forbid cliffs from touching water directly to prevent sprite shearing
    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            if is_cliff[x][y] {
                let mut near_water = false;

                //  search surrounding 3x3 tiles
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && nx < WORLD_SIZE as isize && ny >= 0 && ny < WORLD_SIZE as isize {
                            if is_water[nx as usize][ny as usize] {
                                near_water = true;
                            }
                        }
                    }
                }
                // If water is within 1 tile, delete the cliff
                if near_water {
                    is_cliff[x][y] = false;
                }
            }
        }
    }

    // Now clean up the remaining cliffs!
    cleanup_terrain_layer(&mut is_cliff, 4);

    // ---------------------------------------------------------
    //              PHASE 3 - COMPOSITING & PAINTING
    // ---------------------------------------------------------
    //  Generate a complete grass-based world, and overly every
    //  water and cliff tile in the game. Uses the get_x_variant()
    //  functions to determine what tiles need to be placed.
    //  Returns 2D Tile array.

    let mut final_grid = [[Tile::Grass; WORLD_SIZE]; WORLD_SIZE]; 

    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            if is_water[x][y] {
                final_grid[x][y] = Tile::Water(get_water_variant(&is_water, x, y));
            } else if is_cliff[x][y] {
                final_grid[x][y] = Tile::Cliff(get_cliff_variant(&is_cliff, x, y));
            } else {
                final_grid[x][y] = Tile::Grass;
            }
        }
    }
    
    final_grid
}