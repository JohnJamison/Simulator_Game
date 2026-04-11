use macroquad::prelude::*;
use crate::environment::{Tile, Direction};
use crate::environment::water::WaterVariant;
use crate::environment::wfc_rules::RuleRegistry;
use main::WORLD_SIZE;
#[derive(Clone)]


//=========================  Cell Structure  =========================
//  Needed to implement each cell that will be collapsed.
//  Each cell has an (x,y) location value and a vector of
//  possible values that it can collapes into

pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub possibilities: Vec<Tile>,
}

impl Cell {
    
    pub fn entropy(&self) -> usize {
        self.possibilities.len()
    } //  Entropy is just the number of possible states a cell can be
}
//==============================================================


//                      collapse_with_weights
//  --------------------------------------------------------------
//  sums up all possible tiles and their weights and picks one at random

pub fn collapse_with_weights(possibilities: &Vec<Tile>) -> Tile {
    let total_weight: i32 = possibilities.iter().map(|t| t.weight()).sum();
    let mut rng_val = macroquad::rand::gen_range(0, total_weight);
    
    for tile in possibilities {
        rng_val -= tile.weight();
        if rng_val < 0 { 
            return *tile; 
        }
    }
    
    *possibilities.first().unwrap_or(&Tile::Grass)
}


//                       generate_efc_grid   
//  --------------------------------------------------------------
//  Generates the entire starting world map. and returns the
//  fully generated world grid

pub fn generate_wfc_grid() -> [[Tile; WORLD_SIZE]; WORLD_SIZE] {
    
    let registry = RuleRegistry::new();

    // Load all tiles
    let all_tiles = vec![
        Tile::Grass,
        Tile::Water(WaterVariant::Center),
        Tile::Water(WaterVariant::TopLeft),
        Tile::Water(WaterVariant::Top),
        Tile::Water(WaterVariant::TopRright),
        Tile::Water(WaterVariant::Right),
        Tile::Water(WaterVariant::BottomRight),
        Tile::Water(WaterVariant::Bottom),
        Tile::Water(WaterVariant::BottomLeft),
        Tile::Water(WaterVariant::Left),
        Tile::Water(WaterVariant::InverseCornerTR),
        Tile::Water(WaterVariant::InverseCornerTL),
        Tile::Water(WaterVariant::InverseCornerBL),
        Tile::Water(WaterVariant::InverseCornerBR),
    ];


    // --- Initialize the grid: Every cell starts will all possibilities ---
    let mut grid_cells: Vec<Vec<Cell>> = vec![];

    for x in 0..WORLD_SIZE {    
        let mut col = vec![];
        for y in 0..WORLD_SIZE {
            col.push(Cell { x, y, possibilities: all_tiles.clone() });
        }
        grid_cells.push(col);
    }

    //  --- set the player's starting position to a grass block ---
    let middle = WORLD_SIZE / 2;
    grid_cells[middle][middle].possibilities = vec![Tile::Grass];


    // The Main WFC Loop
    loop {
        
        // --- Find the lowest entropy cell(s) on the map ---
        let mut min_entropy = 9999;
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                let e = grid_cells[x][y].entropy(); //  get current cells entropy
                if e > 1 && e < min_entropy { // update minimum entropy if find lower
                    min_entropy = e;
                }
            }
        }
        // If no cell has an entropy greater than 1, the map is finished!
        if min_entropy == 9999 { break; } 


        // --- Gather all the cells tied for the lowest entropy to pick one randomly ---
        let mut lowest_entropy_cells = vec![]; 
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                if grid_cells[x][y].entropy() == min_entropy {
                    lowest_entropy_cells.push((x, y));
                }
            }
        }


        //  --- pick one of the lowest cells, we will collapse it   ---
        let chosen_idx = macroquad::rand::gen_range(0, lowest_entropy_cells.len()); 
        let (cx, cy) = lowest_entropy_cells[chosen_idx];    //  get chosen cell's coordinates
        let mut cell = &mut grid_cells[cx][cy];     //  get the cell's properties
        let chosen_tile = collapse_with_weights(&cell.possibilities); // collapse the tile
        cell.possibilities = vec![chosen_tile]; // set the possibilities (and thus entropy) to 1 tile.


        // ----------   Propagation  ----------

        //  Now that one cell has been collapsed, we need to collapes
        //  all other cells around it as their possibilities change. 

        //  To-do list of coordinates
        let mut stack: Vec<(usize, usize)> = Vec::new();
        stack.push((cx, cy));

        //  While we still have cells to update     (start with collapsed cell)
        while let Some((curr_x, curr_y)) = stack.pop() {

            //  Get cell's current identity
            let current_possibilities = grid_cells[curr_x][curr_y].possibilities.clone();

            // Get cell's neighbors as (x, y, dir) pair
            let neighbors = [
                (curr_x as isize, curr_y as isize - 1, Direction::North),
                (curr_x as isize + 1, curr_y as isize, Direction::East),
                (curr_x as isize, curr_y as isize + 1, Direction::South),
                (curr_x as isize - 1, curr_y as isize, Direction::West),
            ];

            //  for each neighbor
            for (nx, ny, dir) in neighbors.iter() {
                
                // Ignore the neighboring cell if it's an edge
                if *nx >= 0 && *nx < WORLD_SIZE as isize && *ny >= 0 && *ny < WORLD_SIZE as isize {

                    //grab current cell value
                    let nx = *nx as usize;  //  Access the value stored in nx location
                    let ny = *ny as usize;  //  Same for y value
                    
                    //  Direct reference to the current neighbor cell
                    let neighbor_cell = &mut grid_cells[nx][ny];
                    
                    // If the neighbor is already locked in, skip it
                    if neighbor_cell.entropy() <= 1 { continue; }
                    
                    //  get the original entropy
                    let original_count = neighbor_cell.possibilities.len();

                    // List of all potential tiles of current tile
                    let mut allowed_by_current = Vec::new();

                    //  for each tile in the current possibilities check out
                    //  what tiles are allowed to neighbor it in a given direction
                    //  and add those to the enitre list of possibilities for
                    //  the current tile
                    for current_tile in current_possibilities.iter() {

                        if let Some(allowed) = registry.matrix.get(&(*current_tile, *dir)) {
                            allowed_by_current.append(&mut allowed.clone());
                        }
                    }

                    //  All possible cells the considered neighboring tile can be are
                    //  only the cells that can connect to the current cell in the
                    //  given direction
                    neighbor_cell.possibilities.retain(|neighbor_tile| {
                        allowed_by_current.contains(neighbor_tile)
                    });
                    
                    //  If no cells are possible, we have to restart the generation
                    if neighbor_cell.possibilities.is_empty() {
                        panic!("WFC Deadlock at {}, {}! Restart generation.", nx, ny);
                    }

                    //  Otherwise, if the current neighbor changed, then we need to 
                    //  update all of its neighbors as well, thus the cascading 
                    //  propagational changes
                    if neighbor_cell.possibilities.len() < original_count {
                        stack.push((nx, ny));
                    }

                    //  The loop will carry on until every cell's
                    //  new entropy has been established
                }
            }
        }
    }

    // --- Convert the quantum grid back into the final solid map ---
    let mut final_grid = [[Tile::grass_cutout; WORLD_SIZE]; WORLD_SIZE];
    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            // Because the loop finished, every cell is guaranteed to have exactly 1 item left
            final_grid[x][y] = grid_cells[x][y].possibilities[0].clone();
        }
    }
    
    final_grid
}
