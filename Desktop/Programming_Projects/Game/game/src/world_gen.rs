use macroquad::prelude::*;
use crate::environment::Tile;

pub fn generate_grid() -> [[Tile; 20]; 20] {
    let mut grid = [[Tile::Empty; 20]; 20];
    let num_ponds = macroquad::rand::gen_range(2, 4); 
    
    for _ in 0..num_ponds {
        let start_x = macroquad::rand::gen_range(2, 18) as i32;
        let start_y = macroquad::rand::gen_range(2, 18) as i32;
        let pond_size = macroquad::rand::gen_range(10, 25); 

        let mut water_tiles = vec![(start_x, start_y)];
        grid[start_x as usize][start_y as usize] = Tile::Water;

        for _ in 0..pond_size {
            let idx = macroquad::rand::gen_range(0, water_tiles.len());
            let (cx, cy) = water_tiles[idx];
            let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            let (dx, dy) = dirs[macroquad::rand::gen_range(0, 4)];
            let nx = cx + dx;
            let ny = cy + dy;

            if nx >= 0 && nx < 20 && ny >= 0 && ny < 20 && grid[nx as usize][ny as usize] != Tile::Water {
                grid[nx as usize][ny as usize] = Tile::Water;
                water_tiles.push((nx, ny));
            }
        }
    }

    for _ in 0..macroquad::rand::gen_range(5, 10) {
        let tx = macroquad::rand::gen_range(2, 18) as usize;
        let ty = macroquad::rand::gen_range(2, 18) as usize;
        if grid[tx][ty] == Tile::Empty { grid[tx][ty] = Tile::TreeTrunk; }
    }

    for x in 0..20 {
        for y in 0..20 {
            if grid[x][y] == Tile::Empty && macroquad::rand::gen_range(0, 100) < 20 { 
                grid[x][y] = Tile::BerryBush;
            }
        }
    }

    for x in 9..12 { for y in 9..12 { grid[x][y] = Tile::Empty; } }
    grid
}