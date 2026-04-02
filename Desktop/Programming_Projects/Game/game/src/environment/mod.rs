use crate::inventory::Item;
use macroquad::prelude::Rect;

// Tell Rust what files exist in this folder
pub mod bush; 
pub mod tree1;

#[derive(Clone, Copy, PartialEq)]
pub enum Tile { Empty, Bush, BerryBush, Water, TreeTrunk }

impl Tile {

    pub fn interact(&mut self) -> Option<Item> {
        match self {
            Tile::BerryBush | Tile::Bush => {
                bush::interact_with_bush(self)
            },
            _ => None,
        }
    }

    // New switchboard function for physics!
    pub fn hitbox(&self, x: usize, y: usize) -> Option<Rect> {
        let tile_size = 64.0;
        
        match self {
            Tile::BerryBush | Tile::Bush => {
                bush::hitbox(x, y, tile_size)
            },
            Tile::TreeTrunk => {
                tree1::hitbox(x, y, tile_size)
            },
            Tile::Water => {
                // Water takes up the full 64x64 square
                Some(Rect::new(x as f32 * tile_size, y as f32 * tile_size, tile_size, tile_size))
            },
            Tile::Empty => None,
        }
    }
}