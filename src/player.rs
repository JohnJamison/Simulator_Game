use macroquad::prelude::*;  
use crate::environment::Tile;
use crate::environment::TileBehavior;
use crate::inventory::{Inventory, Item};
use crate::WORLD_SIZE;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction { Down, Left, Up, Right }

pub struct Player {
    pub pixel_x: f32,
    pub pixel_y: f32,
    pub dir: Direction,
    pub is_moving: bool, 
    pub left_foot: bool, 
    pub anim_timer: f32, 
    pub is_fishing: bool,
    pub fishing_timer: f32,
    pub hunger: f32, 
    pub thirst: f32, 
    pub sleep: f32, 
    pub energy: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pixel_x: (WORLD_SIZE as f32 / 2.0) * 64.0, pixel_y: (WORLD_SIZE as f32 / 2.0) * 64.0,
            dir: Direction::Down, is_moving: false, 
            left_foot: true, anim_timer: 0.0,
            is_fishing: false, fishing_timer: 0.0,
            hunger: 0.0, thirst: 0.0, sleep: 0.0, energy: 500.0,
        }
    }

    pub fn hitbox(&self) -> Rect {
        Rect::new(self.pixel_x + 16.0, self.pixel_y + 32.0, 32.0, 32.0)
    }

    fn collides_with_world(&self, test_rect: Rect, grid: &[[Tile; WORLD_SIZE]; WORLD_SIZE]) -> bool {
        let tile_size = 64.0;

        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                for solid_rect in grid[x][y].hitbox(x, y, tile_size) {
                    if test_rect.overlaps(&solid_rect) {
                        return true;
                    }
                }
            }
        }
        let worldsize = WORLD_SIZE as f32;
        if test_rect.x < 0.0 || test_rect.y < 0.0 || 
           test_rect.x + test_rect.w > (worldsize * tile_size) || 
           test_rect.y + test_rect.h > (worldsize * tile_size) {
            return true;
        }

        false
    }

    pub fn face_coord(&self) -> (usize, usize) {
        let center_x = self.pixel_x + 32.0; let center_y = self.pixel_y + 32.0;
        let reach = 40.0; 
        
        let (target_x, target_y) = match self.dir {
            Direction::Right => (center_x + reach, center_y),
            Direction::Left => (center_x - reach, center_y),
            Direction::Down => (center_x, center_y + reach),
            Direction::Up => (center_x, center_y - reach),
        };
        
        ((target_x / 64.0) as usize, (target_y / 64.0) as usize)
    }

    pub fn update(&mut self, dt: f32, grid: &mut [[Tile; WORLD_SIZE]; WORLD_SIZE], inventory: &mut Inventory, drains: (bool, bool, bool)) {
        if drains.0 { self.hunger = (self.hunger + 0.5 * dt).min(100.0); }
        if drains.1 { self.thirst = (self.thirst + 0.8 * dt).min(100.0); }
        if drains.2 { self.sleep = (self.sleep + 0.2 * dt).min(100.0); }
        
        let total_penalty = (self.hunger + self.thirst + self.sleep) / 300.0; 
        let regen_rate = 15.0 * (1.0 - total_penalty).max(0.0); 
        if !self.is_moving && !self.is_fishing { self.energy = (self.energy + regen_rate * dt).min(100.0); }

        if self.is_fishing {
            self.fishing_timer -= dt;
            if self.fishing_timer <= 0.0 {
                self.is_fishing = false;
                if macroquad::rand::gen_range(0, 100) < 60 { inventory.try_add_item(Item::Fish); }
            }
        }

        let speed = 500.0 * dt; 
        let mut dx = 0.0;
        let mut dy = 0.0;

        if self.energy > 2.0 {
            if is_key_down(KeyCode::D) { dx += speed; self.dir = Direction::Right; }
            if is_key_down(KeyCode::A) { dx -= speed; self.dir = Direction::Left; }
            if is_key_down(KeyCode::S) { dy += speed; self.dir = Direction::Down; }
            if is_key_down(KeyCode::W) { dy -= speed; self.dir = Direction::Up; }
        }

        self.is_moving = dx != 0.0 || dy != 0.0;

        if self.is_moving {
            self.is_fishing = false; 
            self.anim_timer += dt;
            self.energy = (self.energy - 10.0 * dt).max(0.0);
            
            if self.anim_timer > 0.15 {
                self.left_foot = !self.left_foot;
                self.anim_timer = 0.0;
            }

            self.pixel_x += dx;
            if self.collides_with_world(self.hitbox(), grid) {
                self.pixel_x -= dx; 
            }

            self.pixel_y += dy;
            if self.collides_with_world(self.hitbox(), grid) {
                self.pixel_y -= dy; 
            }
        }

        let (face_x, face_y) = self.face_coord();

        if (is_mouse_button_pressed(MouseButton::Right) || is_key_pressed(KeyCode::Space)) && face_x < WORLD_SIZE && face_y < WORLD_SIZE {
            let faced_tile = grid[face_x][face_y];

            if matches!(faced_tile, Tile::Water(_)) && is_key_pressed(KeyCode::Space) {
                self.thirst = (self.thirst - 25.0).max(0.0);
            } 
            else {
                if let Some(yielded_item) = grid[face_x][face_y].interact(self) {
                    inventory.try_add_item(yielded_item);
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) && face_x < WORLD_SIZE && face_y < WORLD_SIZE {
            if matches!(grid[face_x][face_y], Tile::Water(_)) && inventory.equipped == Some(Item::FishingRod) {
                self.is_fishing = true; 
                self.fishing_timer = macroquad::rand::gen_range(5.0, 20.0); 
            }
            else {
                if let Some(yielded_item) = grid[face_x][face_y].attack() {
                    inventory.try_add_item(yielded_item);
                }
            }
        }
    }
}