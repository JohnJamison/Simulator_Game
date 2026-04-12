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
use crate::environment::cliffs::CliffVariant;
use crate::environment::biomes::Biome;
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

//  ----------  safe_cliff_opening  ----------
//  Ensure's that a considered opening for a cliff is valid. Achieves this by
//  making sure that the opening does not open up to a wall. 
//  vertical -> 1 = vertical wall, 0 = horizontal wall. This tells the function
//  whether to check vertically (top, bottom ) or horizontally (left, right).
fn safe_cliff_opening(grid: &[[Tile; WORLD_SIZE]; WORLD_SIZE], x: isize, y: isize, vertical: bool) -> bool {
    
    // If the wall is vertical (Left/Right), the player walks HORIZONTALLY through the opening.
    // We must check if the tiles to the Left (x-1) and Right (x+1) are Grass or Center!
    if vertical {
        if x + 1 < WORLD_SIZE as isize && x - 1 >= 0 && y >= 0 && y < WORLD_SIZE as isize {
            let xu_right = (x + 1) as usize;
            let xu_left = (x - 1) as usize;
            let yu = y as usize;

            if (grid[xu_right][yu] == Tile::Grass || grid[xu_right][yu] == Tile::Cliff(CliffVariant::Center)) && 
               (grid[xu_left][yu] == Tile::Grass || grid[xu_left][yu] == Tile::Cliff(CliffVariant::Center)) {
                return true;
            }
        }
        return false;
        
    // If the wall is horizontal (Top/Bottom), the player walks VERTICALLY through the opening.
    // We must check if the tiles Above (y-1) and Below (y+1) are Grass or Center!
    } else {
        if y + 1 < WORLD_SIZE as isize && y - 1 >= 0 && x >= 0 && x < WORLD_SIZE as isize {
            let xu = x as usize;
            let yu_up = (y + 1) as usize;
            let yu_down = (y - 1) as usize;
            
            if (grid[xu][yu_up] == Tile::Grass || grid[xu][yu_up] == Tile::Cliff(CliffVariant::Center)) && 
               (grid[xu][yu_down] == Tile::Grass || grid[xu][yu_down] == Tile::Cliff(CliffVariant::Center)) {
                return true;
            }
        }
        return false;
    }
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

        // Pass 3: Fat Corners (Fixes 1-tile diagonal overlaps & double-inverses)
        let mut next3 = *grid;
        //  for every cell
        for x in 0..(WORLD_SIZE - 1) {
            for y in 0..(WORLD_SIZE - 1) {
                //  check a 2x2 area
                let w_tl = grid[x][y+1]; let w_tr = grid[x+1][y+1];
                let w_bl = grid[x][y]; let w_br = grid[x+1][y];
                let count = (w_tl as u8) + (w_tr as u8) + (w_bl as u8) + (w_br as u8);

                //  if 3 of these cells are active
                if count == 3 {

                    //  Find the Pivot
                    let (px, py) = if !w_tl { (x+1, y) } else if !w_tr { (x, y) } else if !w_bl { (x+1, y+1) } else { (x, y+1) };
                    let (pxi, pyi) = (px as isize, py as isize);

                    //  Check all 4 Cardinals
                    let n = is_active_safe(grid, pxi, pyi-1); let s = is_active_safe(grid, pxi, pyi+1);
                    let e = is_active_safe(grid, pxi+1, pyi); let w = is_active_safe(grid, pxi-1, pyi);
                    
                    //  Check all 4 Diagonals
                    let nw = is_active_safe(grid, pxi-1, pyi-1);
                    let ne = is_active_safe(grid, pxi+1, pyi-1);
                    let sw = is_active_safe(grid, pxi-1, pyi+1);
                    let se = is_active_safe(grid, pxi+1, pyi+1);

                    let diagonals_active = (nw as u8) + (ne as u8) + (sw as u8) + (se as u8);

                    //  THE 8-NEIGHBOR RULE:
                    //  The pivot MUST have all 4 cardinals, AND it cannot have more than 1 empty diagonal.
                    //  If it has less than 3 active diagonals, it is a double-inverse glitch waiting to happen!
                    if !(n && s && e && w && diagonals_active >= 3) {
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
//                                Auto Tilers
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

fn get_cliff_variant(elevation: &[[u8; WORLD_SIZE]; WORLD_SIZE], x: usize, y: usize, current_layer: u8) -> CliffVariant {
    let (xi, yi) = (x as isize, y as isize);

    // A neighbor is considered a solid wall if it is AT LEAST as high as our current layer!
    let check_height = |nx: isize, ny: isize| -> bool {
        if nx < 0 || nx >= WORLD_SIZE as isize || ny < 0 || ny >= WORLD_SIZE as isize { return false; }
        elevation[nx as usize][ny as usize] >= current_layer
    };

    let n = check_height(xi, yi - 1); let s = check_height(xi, yi + 1);
    let e = check_height(xi + 1, yi); let w = check_height(xi - 1, yi);
    let nw = check_height(xi - 1, yi - 1); let ne = check_height(xi + 1, yi - 1);
    let sw = check_height(xi - 1, yi + 1); let se = check_height(xi + 1, yi + 1);

    let mut mask = 0;
    if n { mask += 1; } if s { mask += 2; } if e { mask += 4; } if w { mask += 8; }

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
//                              carve_cliff_openings
// ===============================================================================
// Post-processing pass. Scans the generated map for straight cliff edges.
// If a straight edge meets the minimum size, it rolls a probability check.
// If successful, it replaces a random segment of the wall with Opening/Center tiles.

fn carve_cliff_openings(grid: &mut [[Tile; WORLD_SIZE]; WORLD_SIZE], chance: f32, min_size: usize, max_size: usize) {
    let mut visited = [[false; WORLD_SIZE]; WORLD_SIZE];

    // ---------------------------------------------------------
    //  Scan Vertical Walls
    // ---------------------------------------------------------
    for x in 0..WORLD_SIZE {
        let mut y = 0;
        while y < WORLD_SIZE {

            if let Tile::Cliff(variant) = grid[x][y] {
                let x_ = x as isize;
                let y_ = y as isize;
                if (variant == CliffVariant::Right || variant == CliffVariant::Left) && !visited[x][y] {
                    
                    //  Measure the continuous strip
                    let mut len = 1;
                    while y + len < WORLD_SIZE {
                        if let Tile::Cliff(next_var) = grid[x][y + len] {
                            if (next_var == variant) && safe_cliff_opening(grid, x_, y_+len as isize, true) {
                                len += 1;
                                continue;
                            }
                        }
                        break;
                    }

                    // 2. Mark the entire strip as visited so we don't process its middle later
                    for i in 0..len { visited[x][y + i] = true; }

                    // 3. Roll the Biome Dice!
                    if len >= min_size && macroquad::rand::gen_range(0.0, 1.0) < chance {
                        // Determine random size and starting position within the strip
                        let actual_max = max_size.min(len);
                        let size = macroquad::rand::gen_range(min_size as u32, (actual_max + 1) as u32) as usize;
                        let start_offset = macroquad::rand::gen_range(0, (len - size + 1) as u32) as usize;

                        // 4. Carve the opening
                        for i in 0..size {
                            let target_y = y + start_offset + i;
                            
                            if i == 0 { // Top Tile
                                grid[x][target_y] = if variant == CliffVariant::Right { Tile::Cliff(CliffVariant::RightOpeningUp) } else { Tile::Cliff(CliffVariant::LeftOpeningUp) };
                            } else if i == size - 1 { // Bottom Tile
                                grid[x][target_y] = if variant == CliffVariant::Right { Tile::Cliff(CliffVariant::RightOpeningDown) } else { Tile::Cliff(CliffVariant::LeftOpeningDown) };
                            } else { // Middle Tiles (Flat plateau)
                                grid[x][target_y] = Tile::Cliff(CliffVariant::Center);
                            }
                        }
                    }
                    y += len;
                    continue;
                }
            }
            y += 1;
        }
    }

    // ---------------------------------------------------------
    //  Scan Horizontal Walls
    // ---------------------------------------------------------
    for y in 0..WORLD_SIZE {
        let mut x = 0;
        while x < WORLD_SIZE {
            if let Tile::Cliff(variant) = grid[x][y] {
                if (variant == CliffVariant::Top || variant == CliffVariant::Bottom) && !visited[x][y] {
                    
                    // 1. Measure the continuous strip
                    let mut len = 1;
                    while x + len < WORLD_SIZE {
                        if let Tile::Cliff(next_var) = grid[x + len][y] {
                            let x_ = x as isize;
                            let y_ = y as isize;
                            if (next_var == variant) && safe_cliff_opening(grid, x_ + len as isize, y_, false) {
                                len += 1;
                                continue;
                            }
                        }
                        break;
                    }

                    // 2. Mark as visited
                    for i in 0..len { visited[x + i][y] = true; }

                    // 3. Roll the Biome Dice!
                    if len >= min_size && macroquad::rand::gen_range(0.0, 1.0) < chance {
                        let actual_max = max_size.min(len);
                        let size = macroquad::rand::gen_range(min_size as u32, (actual_max + 1) as u32) as usize;
                        let start_offset = macroquad::rand::gen_range(0, (len - size + 1) as u32) as usize;

                        // 4. Carve the opening
                        for i in 0..size {
                            let target_x = x + start_offset + i;
                            
                            if i == 0 { // Left Tile
                                grid[target_x][y] = if variant == CliffVariant::Top { Tile::Cliff(CliffVariant::TopOpeningLeft) } else { Tile::Cliff(CliffVariant::BottomOpeningLeft) };
                            } else if i == size - 1 { // Right Tile
                                grid[target_x][y] = if variant == CliffVariant::Top { Tile::Cliff(CliffVariant::TopOpeningRight) } else { Tile::Cliff(CliffVariant::BottomOpeningRight) };
                            } else { // Middle Tiles (Flat plateau)
                                grid[target_x][y] = Tile::Cliff(CliffVariant::Center);
                            }
                        }
                    }
                    x += len;
                    continue;
                }
            }
            x += 1;
        }
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

pub fn generate_wfc_grid(biome: &Biome) -> [[Tile; WORLD_SIZE]; WORLD_SIZE] {
    
    // ---------------------------------------------------------
    //              PHASE 1 - Water Generation
    // ---------------------------------------------------------
    let perlin = Perlin::new(macroquad::rand::rand());
    let mut is_water = [[false; WORLD_SIZE]; WORLD_SIZE];  
    
    //  ----------  Generate Noise  ----------
    //  Get the perlin value for every cell and set it to water
    //  if its below -0.3 (35% chance). Noise_scale is mult by
    //  coordinate values to create zoom
    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            if perlin.get([x as f64 * biome.water_scale, y as f64 * biome.water_scale]) < biome.water_threshold {
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
    let mut elevation = [[0u8; WORLD_SIZE]; WORLD_SIZE];

    // 1. Generate Raw Elevation dynamically based on Biome settings
    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            if !is_water[x][y] {
                let noise = cliff_perlin.get([x as f64 * biome.cliff_scale, y as f64 * biome.cliff_scale]);
                let mut h = 0;
                
                // Dynamically check against the requested number of layers!
                for l in 0..biome.cliff_layers {
                    let threshold = biome.cliff_threshold + (l as f64 * biome.cliff_spacing);
                    if noise > threshold { h += 1; }
                }
                elevation[x][y] = h;
            }
        }
    }

    // 2. Carve safe spawn island
    let middle = WORLD_SIZE as f32 / 2.0;
    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            if ((x as f32 - middle).powi(2) + (y as f32 - middle).powi(2)).sqrt() <= 6.5 {
                elevation[x][y] = 0; 
            }
        }
    }

    // 3. The Masking Loop (Process each layer individually!)
    for current_layer in 1..=biome.cliff_layers {
        let mut layer_mask = [[false; WORLD_SIZE]; WORLD_SIZE];

        // Extract boolean mask for just this level
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                if elevation[x][y] >= current_layer { layer_mask[x][y] = true; }
            }
        }

        // CA Smoothing
        for _ in 0..2 {
            let mut next = layer_mask;
            for x in 1..(WORLD_SIZE - 1) {
                for y in 1..(WORLD_SIZE - 1) {
                    if is_water[x][y] { continue; } 
                    let mut neighbors = 0;
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx != 0 || dy != 0 {
                                if layer_mask[(x as isize + dx) as usize][(y as isize + dy) as usize] { neighbors += 1; }
                            }
                        }
                    }
                    if !layer_mask[x][y] && neighbors >= 5 { next[x][y] = true; }
                    else if layer_mask[x][y] && neighbors <= 3 { next[x][y] = false; }
                }
            }
            layer_mask = next;
        }

        // ---------------------------------------------------------
        // THE BUFFER ZONES (Fixes the Overlapping/Shearing!)
        // ---------------------------------------------------------
        if current_layer == 1 {
            // LAYER 1: Water Buffer (Must be 2 tiles away from water)
            for x in 0..WORLD_SIZE {
                for y in 0..WORLD_SIZE {
                    if layer_mask[x][y] {
                        let mut near_water = false;
                        for dx in -2..=2 {
                            for dy in -2..=2 {
                                let nx = x as isize + dx; let ny = y as isize + dy;
                                if nx >= 0 && nx < WORLD_SIZE as isize && ny >= 0 && ny < WORLD_SIZE as isize {
                                    if is_water[nx as usize][ny as usize] { near_water = true; }
                                }
                            }
                        }
                        if near_water { layer_mask[x][y] = false; }
                    }
                }
            }
        } else {
            // LAYER 2+: Plateau Buffer 
            // A 2-tile radius forces a wide terrace and prevents Algorithm Fighting with Pass 2!
            for x in 0..WORLD_SIZE {
                for y in 0..WORLD_SIZE {
                    if layer_mask[x][y] {
                        let mut near_ledge = false;
                        for dx in -2..=2 { // 2-TILE RADIUS!
                            for dy in -2..=2 {
                                let nx = x as isize + dx; let ny = y as isize + dy;
                                if nx >= 0 && nx < WORLD_SIZE as isize && ny >= 0 && ny < WORLD_SIZE as isize {
                                    // If neighbor is a sheer drop (lower than the layer we are sitting on)
                                    if elevation[nx as usize][ny as usize] < current_layer - 1 {
                                        near_ledge = true;
                                    }
                                } else {
                                    near_ledge = true; // Map edges are sheer drops
                                }
                            }
                        }
                        if near_ledge { layer_mask[x][y] = false; }
                    }
                }
            }
        }

        // Clean up the layer!
        cleanup_terrain_layer(&mut layer_mask, 4);

        // Save validated mask back to the 3D elevation map
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                if layer_mask[x][y] {
                    // If cleanup expanded a layer, ensure the 3D map reflects the height boost
                    if elevation[x][y] < current_layer {
                        elevation[x][y] = current_layer;
                    }
                } else if !layer_mask[x][y] && elevation[x][y] >= current_layer {
                    // If pruned by cleanup or buffers, drop all stacked elevation down!
                    elevation[x][y] = current_layer - 1; 
                }
            }
        }
    }
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
            } else if elevation[x][y] > 0 {
                // Pass the 3D elevation map and the specific height level!
                final_grid[x][y] = Tile::Cliff(get_cliff_variant(&elevation, x, y, elevation[x][y]));
            } else {
                final_grid[x][y] = Tile::Grass;
            }
        }
    }

    // ---------------------------------------------------------
    // PHASE 4: BIOME POST-PROCESSING
    // ---------------------------------------------------------
    // Parameters: (grid, chance_of_opening, min_width, max_width)
    // 0.6 = 60% chance a valid straight wall gets a ramp.
    carve_cliff_openings(&mut final_grid, biome.ramp_chance, biome.min_ramp_width, biome.max_ramp_width);
    
    export_map_to_png(&final_grid, biome);

    final_grid
} 

// ===============================================================================
//                              export_map_to_png
// ===============================================================================
// Renders the entire generated 2D grid into an Image buffer and saves it as a PNG.
// Extremely useful for debugging biome variables without having to walk around.
pub fn export_map_to_png(grid: &[[Tile; WORLD_SIZE]; WORLD_SIZE], biome: &Biome) {
    // 1 tile = 4 pixels (Makes the output image 4x larger and easier to view)
    let scale = 4; 
    let img_width = (WORLD_SIZE * scale) as u16;
    let img_height = (WORLD_SIZE * scale) as u16;
    
    // Create a blank canvas
    let mut img = Image::gen_image_color(img_width, img_height, WHITE);

    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            
            // Map the tiles to colors. 
            // We use dark brown for cliff edges to draw topological contour lines!
            let color = match &grid[x][y] {
                Tile::Grass => Color::new(0.55, 0.78, 0.45, 1.0),
                Tile::Water(_) => Color::new(0.25, 0.55, 0.90, 1.0),
                Tile::Cliff(variant) => {
                    if *variant == CliffVariant::Center {
                        Color::new(0.70, 0.55, 0.40, 1.0) // Lighter plateaus
                    } else {
                        Color::new(0.45, 0.30, 0.15, 1.0) // Darker sheer edges & ramps
                    }
                },
                _ => Color::new(0.0,0.0,0.0,0.0)
            };

            // Paint the scaled block of pixels for this specific tile
            for dx in 0..scale {
                for dy in 0..scale {
                    img.set_pixel((x * scale + dx) as u32, (y * scale + dy) as u32, color);
                }
            }
        }
    }

    // Generate the dynamic filename using the biome variables
    let filename = format!(
        "map_ws{}_wt{}_cs{}_ct{}_layers{}.png",
        biome.water_scale, biome.water_threshold,
        biome.cliff_scale, biome.cliff_threshold, biome.cliff_layers
    );

    // Save it to the root of your project directory!
    img.export_png(&filename);
    println!("SUCCESS: Exported world map to {}", filename);
}
