// src/environment/bush.rs
use crate::inventory::Item;
use crate::environment::Tile;
use macroquad::prelude::Rect;

pub fn interact(tile: &mut Tile) -> Option<Item> {
    if *tile == Tile::BerryBush {
        *tile = Tile::Bush; 
        Some(Item::Berry)
    } else {
        None
    }   
}

pub fn attack(tile: &mut Tile) -> Option<Item> {
    if *tile == Tile::Bush || *tile == Tile::BerryBush {
        *tile = Tile::Grass;
        Some(Item::Twig)
    } else {
        None
    }
}

pub fn hitbox(x: usize, y: usize, tile_size: f32) -> Vec<Rect> {
    vec![Rect::new((x as f32 * tile_size) + 8.0, (y as f32 * tile_size) + 8.0, 48.0, 48.0)]
}