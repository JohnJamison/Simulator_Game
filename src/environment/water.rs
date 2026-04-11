// src/environment/water.rs
use macroquad::prelude::Rect; 
use crate::environment::{Direction::{self, *}, Tile::{self, *}};
use crate::environment::TileBehavior;
use crate::player::Player;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum WaterVariant {
    Center, TopLeft, Top, TopRight, Right, BottomRight, Bottom, BottomLeft, Left,
    InverseCornerTR, InverseCornerTL, InverseCornerBL, InverseCornerBR
}
use WaterVariant::*;

impl TileBehavior for WaterVariant {
    fn interact(&mut self, player: &mut Player) -> Option<crate::inventory::Item> {
        player.thirst = 100.0;
        None
    }

    fn attack(&mut self) -> Option<crate::inventory::Item> {
        None
    }

    fn hitbox(&self, x: usize, y: usize, tile_size: f32) -> Vec<Rect> {
        let half = tile_size / 2.0;
        let px = x as f32 * tile_size;
        let py = y as f32 * tile_size;

        match self {
            Center => vec![],
            TopLeft => vec![Rect::new(px + half, py + half, half, half)],
            Top => vec![Rect::new(px, py + half, tile_size, half)],
            TopRight => vec![Rect::new(px + half, py, half, half)],
            Right => vec![Rect::new(px, py, half, tile_size)],
            BottomRight => vec![Rect::new(px, py, half, half)],
            Bottom => vec![Rect::new(px, py, tile_size, half)],
            BottomLeft => vec![Rect::new(px + half, py, half, half)],
            Left => vec![Rect::new(px + half, py, half, tile_size)],
            InverseCornerTL => vec![
                Rect::new(px + half, py, half, half),          // Block Top-Right
                Rect::new(px, py + half, tile_size, half)      // Block Bottom-Half
            ],
            // Inverse TR: Land is top-right. Water blocks the Top-Left and Bottom-Half.
            InverseCornerTR => vec![
                Rect::new(px, py, half, half),                 // Block Top-Left
                Rect::new(px, py + half, tile_size, half)      // Block Bottom-Half
            ],
            // Inverse BL: Land is bottom-left. Water blocks the Top-Half and Bottom-Right.
            InverseCornerBL => vec![
                Rect::new(px, py, tile_size, half),            // Block Top-Half
                Rect::new(px + half, py + half, half, half)    // Block Bottom-Right
            ],
            // Inverse BR: Land is bottom-right. Water blocks the Top-Half and Bottom-Left.
            InverseCornerBR => vec![
                Rect::new(px, py, tile_size, half),            // Block Top-Half
                Rect::new(px, py + half, half, half)           // Block Bottom-Left
            ],
        }
    }


    //  function to update the water's weight based on the amount of water
    //  in the environment so far
    fn weight(&self) -> i32 {
        match self {
            Center => 40,
            Top | Bottom | Left | Right => 10,
            _ => 10
        }
    }

    fn sheet_coords(&self) -> Option<(f32, f32)> {
        match self {
            Center => Some((3.0, 8.0)),
            Top => Some((3.0, 7.0)),
            Bottom => Some((3.0, 9.0)),
            Left => Some((2.0, 8.0)),
            Right => Some((4.0, 8.0)),
            TopLeft => Some((2.0, 7.0)),
            TopRight => Some((4.0, 7.0)),
            BottomLeft => Some((2.0, 9.0)),
            BottomRight => Some((4.0, 9.0)),
            InverseCornerTL => Some((3.0, 10.0)),
            InverseCornerTR => Some((4.0, 10.0)),
            InverseCornerBL => Some((3.0, 11.0)),
            InverseCornerBR => Some((4.0, 11.0)),
        }
    }
}