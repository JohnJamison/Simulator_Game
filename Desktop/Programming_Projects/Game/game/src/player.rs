use macroquad::prelude::*;
use crate::world_gen::Tile;
use crate::inventory::{Inventory, Item};

#[derive(Clone, Copy, PartialEq)]
pub enum Direction { Down, Left, Up, Right }

pub struct Player {
    pub grid_x: usize, pub grid_y: usize,
    pub pixel_x: f32, pub pixel_y: f32,
    pub dir: Direction,
    pub is_moving: bool, 
    pub movement_cooldown: f64, 
    pub last_move_time: f64, 
    pub left_foot: bool, 
    pub anim_timer: f32, 

    pub is_fishing: bool,
    pub fishing_timer: f32,
    
    pub hunger: f32, pub thirst: f32, pub sleep: f32, pub energy: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            grid_x: 10, grid_y: 10, pixel_x: 640.0, pixel_y: 640.0,
            dir: Direction::Down, is_moving: false, movement_cooldown: 0.0,
            last_move_time: 0.0, left_foot: true, anim_timer: 0.0,
            is_fishing: false, fishing_timer: 0.0,
            hunger: 0.0, thirst: 0.0, sleep: 0.0, energy: 100.0,
        }
    }

    pub fn face_coord(&self) -> (usize, usize) {
        let mut face_x = self.grid_x; let mut face_y = self.grid_y;
        match self.dir {
            Direction::Right => face_x += 1, Direction::Left => face_x = face_x.saturating_sub(1), 
            Direction::Down => face_y += 1, Direction::Up => face_y = face_y.saturating_sub(1),
        }
        (face_x, face_y)
    }

    pub fn update(&mut self, dt: f32, grid: &mut [[Tile; 20]; 20], inventory: &mut Inventory, drains: (bool, bool, bool)) {
        if drains.0 { self.hunger = (self.hunger + 0.5 * dt).min(100.0); }
        if drains.1 { self.thirst = (self.thirst + 0.8 * dt).min(100.0); }
        if drains.2 { self.sleep = (self.sleep + 0.2 * dt).min(100.0); }

        let total_penalty = (self.hunger + self.thirst + self.sleep) / 300.0; 
        let regen_rate = 15.0 * (1.0 - total_penalty).max(0.0); 
        
        if !self.is_moving && !self.is_fishing { self.energy = (self.energy + regen_rate * dt).min(100.0); }

        // --- FISHING LOGIC ---
        if self.is_fishing {
            self.fishing_timer -= dt;
            if self.fishing_timer <= 0.0 {
                self.is_fishing = false;
                if macroquad::rand::gen_range(0, 100) < 60 { inventory.try_add_item(Item::Fish); }
            }
        }

        let tile_size = 64.0;
        let speed = 250.0 * dt; 

        if self.is_moving {
            self.anim_timer += dt;
            self.energy = (self.energy - 10.0 * dt).max(0.0);
            let target_x = self.grid_x as f32 * tile_size; let target_y = self.grid_y as f32 * tile_size;
            
            if self.pixel_x < target_x { self.pixel_x += speed; }
            if self.pixel_x > target_x { self.pixel_x -= speed; }
            if self.pixel_y < target_y { self.pixel_y += speed; }
            if self.pixel_y > target_y { self.pixel_y -= speed; }
            
            if (self.pixel_x - target_x).abs() < speed && (self.pixel_y - target_y).abs() < speed {
                self.pixel_x = target_x; self.pixel_y = target_y;
                self.is_moving = false; self.last_move_time = get_time(); 
            }
        } else {
            let (face_x, face_y) = self.face_coord();

            // LEFT CLICK: Punch Bush OR Fish in Water
            if is_mouse_button_pressed(MouseButton::Left) && face_x < 20 && face_y < 20 {
                let faced = grid[face_x][face_y];
                if faced == Tile::Bush || faced == Tile::BerryBush {
                    if inventory.try_add_item(Item::Twig) { grid[face_x][face_y] = Tile::Empty; }
                } else if faced == Tile::Water && inventory.equipped == Some(Item::FishingRod) {
                    self.is_fishing = true;
                    self.fishing_timer = macroquad::rand::gen_range(5.0, 20.0);
                }
            }

            // SPACE: Harvest/Drink
            if is_key_pressed(KeyCode::Space) && face_x < 20 && face_y < 20 {
                if grid[face_x][face_y] == Tile::BerryBush {
                    if inventory.try_add_item(Item::Berry) { grid[face_x][face_y] = Tile::Bush; }
                } else if grid[face_x][face_y] == Tile::Water {
                    self.thirst = (self.thirst - 25.0).max(0.0);
                }
            }

            // --- WASD MOVEMENT ---
            let mut next_x = self.grid_x; let mut next_y = self.grid_y;
            let mut intended_dir = self.dir; let mut wants_to_move = false;

            if is_key_down(KeyCode::D) { intended_dir = Direction::Right; wants_to_move = true; }
            else if is_key_down(KeyCode::A) { intended_dir = Direction::Left; wants_to_move = true; }
            else if is_key_down(KeyCode::S) { intended_dir = Direction::Down; wants_to_move = true; }
            else if is_key_down(KeyCode::W) { intended_dir = Direction::Up; wants_to_move = true; }

            if wants_to_move && self.energy > 2.0 {
                self.is_fishing = false; // Moving breaks the fishing line
                if self.dir != intended_dir {
                    self.dir = intended_dir;
                    if (get_time() - self.last_move_time) >= 0.05 { self.movement_cooldown = get_time() + 0.1; }
                }

                if get_time() >= self.movement_cooldown {
                    match intended_dir {
                        Direction::Right => next_x += 1, Direction::Left => next_x = next_x.saturating_sub(1), 
                        Direction::Down => next_y += 1, Direction::Up => next_y = next_y.saturating_sub(1),
                    }
                    if next_x < 20 && next_y < 20 && grid[next_x][next_y] == Tile::Empty {
                        self.grid_x = next_x; self.grid_y = next_y;
                        self.is_moving = true; self.left_foot = !self.left_foot; self.anim_timer = 0.0; 
                    }
                }
            } else if !wants_to_move {
                self.movement_cooldown = 0.0;
            }
        }
    }
}