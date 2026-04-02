/*===========================================================================
                                Player.rs
=============================================================================

    Main file that handles all player state interactions

===========================================================================*/

use macroquad::prelude::*;  //Game Engine
use crate::environment::Tile;
use crate::inventory::{Inventory, Item};

#[derive(Clone, Copy, PartialEq)]
pub enum Direction { Down, Left, Up, Right }

pub struct Player {
    
    pub pixel_x: f32,   // Left-most x_value of sprite
    pub pixel_y: f32,   // Top-most y-value of sprite
    pub dir: Direction,
    
    // Animation tracking
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
            // Start the player in the safe zone (10 * 64 = 640)
            pixel_x: 640.0, pixel_y: 640.0,
            dir: Direction::Down, is_moving: false, 
            left_foot: true, anim_timer: 0.0,
            is_fishing: false, fishing_timer: 0.0,
            hunger: 0.0, thirst: 0.0, sleep: 0.0, energy: 100.0,
        }
    }


    // Uses - Rect(x_coordinate, y_coordinate, width, height) - to create a player hitbox
    pub fn hitbox(&self) -> Rect {
        Rect::new(self.pixel_x + 16.0, self.pixel_y + 32.0, 32.0, 32.0)
    }


    // Helper that checks if a specific Rectangle overlaps with 
    // any solid object on the map.
    // NOTE -- change this with time
    fn collides_with_world(&self, test_rect: Rect, grid: &[[Tile; 20]; 20]) -> bool {
        let tile_size = 64.0;

        for x in 0..20 {
            for y in 0..20 {
                
                if let Some(solid_rect) = grid[x][y].hitbox(x,y) {
                    if test_rect.overlaps(&solid_rect) {
                        return true;
                    }
                }
            }
        }
        
        // Also prevent walking off the absolute edges of the 20x20 map
        if test_rect.x < 0.0 || test_rect.y < 0.0 || 
           test_rect.x + test_rect.w > (20.0 * tile_size) || 
           test_rect.y + test_rect.h > (20.0 * tile_size) {
            return true;
        }

        false
    }


    // Uses ray-casting to see what object is in front of the player,
    // returns the coordinate value of the object. Useful for interactions
    pub fn face_coord(&self) -> (usize, usize) {
        let center_x = self.pixel_x + 32.0; let center_y = self.pixel_y + 32.0;
        let reach = 40.0; // How far in front of the player we check
        
        let (target_x, target_y) = match self.dir {
            Direction::Right => (center_x + reach, center_y),
            Direction::Left => (center_x - reach, center_y),
            Direction::Down => (center_x, center_y + reach),
            Direction::Up => (center_x, center_y - reach),
        };
        
        // Convert that pixel coordinate back into a map array index
        ((target_x / 64.0) as usize, (target_y / 64.0) as usize)
    }


    //  Update:     Updates the state of the world. Called 60 times 
    //  every second from the main game loop.
    //  ------------------------------------------------------------------
    //  dt: 'Delta Time' value that tells amount of time that passed since last frame.
    //  grid:   World map
    //  inventory:  Self Explanitory
    //  drains: 
    //  ------------------------------------------------------------------  
    pub fn update(&mut self, dt: f32, grid: &mut [[Tile; 20]; 20], inventory: &mut Inventory, drains: (bool, bool, bool)) {
        
        //  - Drains the survival statistics 
        //  - We multiply the rate per second by dt
        //      so that the rate is frame-rate independent, and users with faster frame
        //      rates do not have faster drain rates. 
        //  - .min(100) ensures that each stat cannot exceed 100
        if drains.0 { self.hunger = (self.hunger + 0.5 * dt).min(100.0); }
        if drains.1 { self.thirst = (self.thirst + 0.8 * dt).min(100.0); }
        if drains.2 { self.sleep = (self.sleep + 0.2 * dt).min(100.0); }
        
        //  --------------- Regeneration Logic ---------------
        //  - Regenerateion rate is 15 when the player's needs are met,
        //      otherwise, if the player is very hungry, thirsty, or tired,
        //      the regeneration rate becomes much slower because the penatly
        //      grows larger.
        //  - If not moving, and not fishing, then regenerate Energy
        let total_penalty = (self.hunger + self.thirst + self.sleep) / 300.0; 
        let regen_rate = 15.0 * (1.0 - total_penalty).max(0.0); 
        if !self.is_moving && !self.is_fishing { self.energy = (self.energy + regen_rate * dt).min(100.0); }


        // --------------- FISHING LOGIC ---------------
        if self.is_fishing {
            self.fishing_timer -= dt;
            if self.fishing_timer <= 0.0 {
                self.is_fishing = false;
                if macroquad::rand::gen_range(0, 100) < 60 { inventory.try_add_item(Item::Fish); }
            }
        }


        let speed = 250.0 * dt; 
        let mut dx = 0.0;
        let mut dy = 0.0;

        // --- FREE MOVEMENT INPUT ---
        if self.energy > 2.0 {
            if is_key_down(KeyCode::D) { dx += speed; self.dir = Direction::Right; }
            if is_key_down(KeyCode::A) { dx -= speed; self.dir = Direction::Left; }
            if is_key_down(KeyCode::S) { dy += speed; self.dir = Direction::Down; }
            if is_key_down(KeyCode::W) { dy -= speed; self.dir = Direction::Up; }
        }

        self.is_moving = dx != 0.0 || dy != 0.0;

        // --- THE SLIDING COLLISION SYSTEM ---
        if self.is_moving {
            self.is_fishing = false; // Moving breaks the line
            self.anim_timer += dt;
            self.energy = (self.energy - 10.0 * dt).max(0.0);
            
            // Toggle feet for animation every 0.15 seconds
            if self.anim_timer > 0.15 {
                self.left_foot = !self.left_foot;
                self.anim_timer = 0.0;
            }

            // Move X axis independently
            self.pixel_x += dx;
            if self.collides_with_world(self.hitbox(), grid) {
                self.pixel_x -= dx; // We hit a wall, cancel the X movement!
            }

            // Move Y axis independently
            self.pixel_y += dy;
            if self.collides_with_world(self.hitbox(), grid) {
                self.pixel_y -= dy; // We hit a wall, cancel the Y movement!
            }
        }


        // --------------- INTERACTION LOGIC ---------------
        //
        let (face_x, face_y) = self.face_coord();

        // 1. GENERAL INTERACTION (Space or Left Click)
        if (is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Space)) && face_x < 20 && face_y < 20 {
            let faced_tile = grid[face_x][face_y];

            // -> Player Logic: Drinking Water
            if faced_tile == Tile::Water && is_key_pressed(KeyCode::Space) {
                self.thirst = (self.thirst - 25.0).max(0.0);
            } 
            // -> World Logic: Ask the tile to do its own thing (Bushes, Trees, etc.)
            else {
                if let Some(yielded_item) = grid[face_x][face_y].interact() {
                    inventory.try_add_item(yielded_item);
                }
            }
        }

        // 2. TOOL INTERACTION (Right Click for Fishing)
        if is_mouse_button_pressed(MouseButton::Right) && face_x < 20 && face_y < 20 {
            // -> Player Logic: Check for water AND the equipped rod
            if grid[face_x][face_y] == Tile::Water && inventory.equipped == Some(Item::FishingRod) {
                self.is_fishing = true; // Freeze the player
                self.fishing_timer = macroquad::rand::gen_range(5.0, 20.0); // Start the timer
            }
        }
    }
}