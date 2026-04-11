use macroquad::prelude::Rect; 
use crate::environment::{Direction::{self, *}, Tile::{self, *}};
use crate::environment::TileBehavior;
use crate::player::Player;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CliffVariant {
    Center, TopLeft, Top, TopRight, Right, BottomRight, Bottom, BottomLeft, Left,
    InverseCornerTR, InverseCornerTL, InverseCornerBL, InverseCornerBR, LeftOpeningUp,
    LeftOpeningDown, RightOpeningUp, RightOpeningDown, BottomOpeningLeft, BottomOpeningRight,
    TopOpeningLeft, TopOpeningRight
}
use CliffVariant::*;

impl TileBehavior for CliffVariant {

    //  Do Nothing
    fn interact(&mut self, player: &mut Player) -> Option<crate::inventory::Item> {
        None
    }

    //  Do Nothing
    fn attack(&mut self) -> Option<crate::inventory::Item> {
        None
    }

    fn hitbox(&self, x: usize, y: usize, tile_size: f32) -> Vec<Rect> {
        let half = tile_size / 2.0;
        let px = x as f32 * tile_size;
        let py = y as f32 * tile_size;

        match self {
            Center => vec![Rect::new(px, py, 0.0, 0.0)],
            TopLeft => vec![Rect::new(px + half, py + half, half, half)],
            Top => vec![Rect::new(px, py + half, tile_size, half)],
            TopRight => vec![Rect::new(px + half, py, half, half)],
            Right => vec![Rect::new(px, py, half, tile_size)],
            BottomRight => vec![Rect::new(px, py, half, half)],
            Bottom => vec![Rect::new(px, py, tile_size, half)],
            BottomLeft => vec![Rect::new(px + half, py, half, half)],
            Left => vec![Rect::new(px + half, py, half, tile_size)],
            InverseCornerTL => vec![Rect::new(px, py, tile_size, tile_size)],
            InverseCornerTR => vec![Rect::new(px, py, tile_size, tile_size)],
            InverseCornerBL => vec![Rect::new(px, py, tile_size, tile_size)],
            InverseCornerBR => vec![Rect::new(px, py, tile_size, tile_size)],
            LeftOpeningUp => vec![Rect::new(px + half, py, half, half)],
            LeftOpeningDown => vec![Rect::new(px + half, py, half, half)],
            RightOpeningUp => vec![Rect::new(px + half, py, half, half)],
            RightOpeningDown => vec![Rect::new(px + half, py, half, half)],
            BottomOpeningLeft => vec![Rect::new(px + half, py, half, half)],
            BottomOpeningRight => vec![Rect::new(px + half, py, half, half)],
            TopOpeningLeft => vec![Rect::new(px + half, py, half, half)],
            TopOpeningRight => vec![Rect::new(px + half, py, half, half)],
            
        }
    }

    fn weight(&self) -> i32 {
        10
    }

    fn sheet_coords(&self) -> Option<(f32, f32)> {
        match self {
            Center => Some((3.0, 1.0)), 
            TopLeft => Some((2.0, 0.0)), 
            Top => Some((3.0, 0.0)), 
            TopRight => Some((4.0, 0.0)), 
            Right => Some((4.0, 1.0)), 
            BottomRight => Some((4.0, 2.0)), 
            Bottom => Some((3.0, 2.0)), 
            BottomLeft => Some((2.0, 2.0)), 
            Left => Some((2.0, 1.0)),
            InverseCornerTR => Some((4.0, 3.0)), 
            InverseCornerTL => Some((3.0, 3.0)), 
            InverseCornerBL => Some((3.0, 4.0)), 
            InverseCornerBR => Some((4.0, 4.0)), 
            LeftOpeningUp => Some((2.0, 3.0)),
            LeftOpeningDown => Some((2.0, 4.0)), 
            RightOpeningUp => Some((2.0, 5.0)), 
            RightOpeningDown => Some((2.0, 6.0)), 
            BottomOpeningLeft => Some((3.0, 6.0)), 
            BottomOpeningRight => Some((4.0, 6.0)),
            TopOpeningLeft => Some((3.0, 5.0)), 
            TopOpeningRight => Some((4.0, 5.0)),
        }
    }
}
