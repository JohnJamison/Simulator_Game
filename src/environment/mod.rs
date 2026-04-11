// src/environment/mod.rs
use crate::inventory::Item;
use macroquad::prelude::Rect;
use crate::player::Player;

pub mod bush; 
pub mod tree1;
pub mod water;
pub mod cliffs;
pub mod wfc_rules;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction { North, East, South, West }

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Tile { Grass, Bush, BerryBush, Water(water::WaterVariant), Cliff(cliffs::CliffVariant), TreeTrunk }

pub trait TileBehavior {
    fn hitbox(&self, x: usize, y: usize, tile_size: f32) -> Vec<Rect>;
    fn attack(&mut self) -> Option<Item>;
    fn interact(&mut self, player: &mut Player) -> Option<Item>;
    fn weight(&self) -> i32;
    fn sheet_coords(&self) -> Option<(f32, f32)>;
}

impl TileBehavior for Tile {
    fn hitbox(&self, x: usize, y: usize, tile_size: f32) -> Vec<Rect> {
        match self {
            //Tile::Water(w) => w.hitbox(x, y, tile_size),
            Tile::Bush | Tile::BerryBush => bush::hitbox(x, y, tile_size),
            //Tile::Cliff(c) => c.hitbox(x, y, tile_size),
            _ => vec![]
        }
    }

    fn interact(&mut self, player: &mut Player) -> Option<Item> {
        match self {
            Tile::Water(w) => w.interact(player),
            Tile::Bush | Tile::BerryBush => bush::interact(self), 
            _ => None,
        }
    }

    fn attack(&mut self) -> Option<Item> {
        match self {
            Tile::Bush | Tile::BerryBush => bush::attack(self),
            _ => None,
        }
    }

    fn sheet_coords(&self) -> Option<(f32, f32)> {
        match self {
            Tile::Water(w) => w.sheet_coords(),
            Tile::BerryBush => Some((9.0, 0.0)),
            Tile::Bush => Some((9.0, 1.0)),
            Tile::Cliff(c) => c.sheet_coords(),
            _ => None
        }
    }

    fn weight(&self) -> i32 {
        match self {
            Tile::Water(w) => w.weight(),
            Tile::Grass => 50,
            _ => 1,
        }
    }
}