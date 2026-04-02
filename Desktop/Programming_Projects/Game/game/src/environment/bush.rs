
use crate::inventory::Item;
use super::Tile;
use macroquad::prelude::Rect;

//  Bush Interaction:
//  - if spacebar is pressed, take berries from bush
//  - if attacked, destory the bush 
//--------------------------------------------------------
pub fn interact_with_bush(tile: &mut Tile) -> Option<Item> {
    match tile {
        Tile::BerryBush => {
            *tile = Tile::Bush; // Downgrade to an empty bush
            Some(Item::Berry)
        },
        Tile::Bush => {
            *tile = Tile::Empty; // Destroy the bush
            Some(Item::Twig)
        },
        _ => None,
    }
}

pub fn hitbox(x:usize, y: usize, tile_size: f32) -> Option<Rect> {
    Some(Rect::new((x as f32 * tile_size) + 8.0, (y as f32 * tile_size) + 8.0, 48.0, 48.0))
}