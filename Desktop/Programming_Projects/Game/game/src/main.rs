use macroquad::prelude::*;

// ==========================================
// GAME DATA STRUCTURES
// ==========================================

#[derive(PartialEq)]
enum GameMode {
    Play,
    InventoryMenu,
    SettingsMenu, // --- NEW: Settings mode ---
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Bush,
    Water,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction { Down, Left, Up, Right }

// --- INVENTORY SYSTEM ---
#[derive(Clone, Copy, PartialEq)]
enum Item {
    Berry,
}

impl Item {
    fn weight(&self) -> u32 { match self { Item::Berry => 1, } }
    fn name(&self) -> &str { match self { Item::Berry => "Fresh Berry", } }
    
    // How much hunger this item restores when eaten
    fn nutrition(&self) -> f32 { match self { Item::Berry => 15.0, } }
}

struct Inventory {
    slots: Vec<(Item, u32)>, 
    max_capacity: u32,
}

impl Inventory {
    fn new() -> Self { Inventory { slots: Vec::new(), max_capacity: 30 } }
    
    fn current_weight(&self) -> u32 {
        let mut total = 0;
        for (item, count) in &self.slots { total += item.weight() * count; }
        total
    }

    fn try_add_item(&mut self, item: Item) -> bool {
        if self.current_weight() + item.weight() <= self.max_capacity {
            for slot in &mut self.slots {
                if slot.0 == item { slot.1 += 1; return true; }
            }
            self.slots.push((item, 1));
            true
        } else { false }
    }
    
    // Updated to return the item so the game knows what we just ate
    fn consume_item(&mut self, index: usize) -> Option<Item> {
        if index < self.slots.len() {
            let item = self.slots[index].0;
            self.slots[index].1 -= 1;
            if self.slots[index].1 == 0 { self.slots.remove(index); }
            Some(item)
        } else { None }
    }
}

struct GameState {
    mode: GameMode,
    inventory: Inventory,
    inv_selection: usize, 
    settings_selection: usize, // Tracks cursor in settings menu
    
    // --- SURVIVAL STATS ---
    hunger: f32, 
    thirst: f32,
    sleep: f32,
    energy: f32,
    
    // --- SETTINGS TOGGLES ---
    drain_hunger: bool,
    drain_thirst: bool,
    drain_sleep: bool,
    
    grid: [[Tile; 20]; 20],
    player_grid_x: usize,
    player_grid_y: usize,
    player_pixel_x: f32,
    player_pixel_y: f32,
    player_dir: Direction,
    is_moving: bool, 
    movement_cooldown: f64, 
    last_move_time: f64, 
    left_foot: bool, 
    anim_timer: f32, 
}

impl GameState {
    fn new() -> Self {
        let mut state = GameState {
            mode: GameMode::Play,
            inventory: Inventory::new(),
            inv_selection: 0,
            settings_selection: 0,
            
            // Start the player healthy and full of energy
            hunger: 0.0,
            thirst: 0.0,
            sleep: 0.0,
            energy: 100.0,
            
            // Features default to ON for realism
            drain_hunger: true,
            drain_thirst: true,
            drain_sleep: true,
            
            grid: [[Tile::Empty; 20]; 20],
            player_grid_x: 10,
            player_grid_y: 10,
            player_pixel_x: 640.0, 
            player_pixel_y: 640.0,
            player_dir: Direction::Down,
            is_moving: false,
            movement_cooldown: 0.0,
            last_move_time: 0.0,
            left_foot: true,
            anim_timer: 0.0,
        };

        for x in 0..20 {
            for y in 0..20 {
                if x > 8 && x < 12 && y > 8 && y < 12 { continue; }
                let chance = macroquad::rand::gen_range(0, 100);
                if chance < 10 { state.grid[x][y] = Tile::Water; } 
                else if chance < 30 { state.grid[x][y] = Tile::Bush; }
            }
        }
        state 
    }

    fn update(&mut self) {
        // --- MENU TOGGLES ---
        if is_key_pressed(KeyCode::E) && self.mode != GameMode::SettingsMenu {
            if self.mode == GameMode::Play { self.mode = GameMode::InventoryMenu; self.inv_selection = 0; } 
            else { self.mode = GameMode::Play; }
        }
        if is_key_pressed(KeyCode::Tab) && self.mode != GameMode::InventoryMenu {
            if self.mode == GameMode::Play { self.mode = GameMode::SettingsMenu; self.settings_selection = 0; } 
            else { self.mode = GameMode::Play; }
        }

        // --- INVENTORY MENU LOGIC ---
        if self.mode == GameMode::InventoryMenu {
            let max_idx = self.inventory.slots.len().saturating_sub(1);
            if is_key_pressed(KeyCode::Right) { self.inv_selection = (self.inv_selection + 1).min(max_idx); }
            if is_key_pressed(KeyCode::Left) { self.inv_selection = self.inv_selection.saturating_sub(1); }
            if is_key_pressed(KeyCode::Down) { self.inv_selection = (self.inv_selection + 5).min(max_idx); }
            if is_key_pressed(KeyCode::Up) { self.inv_selection = self.inv_selection.saturating_sub(5); }

            if is_key_pressed(KeyCode::Space) && !self.inventory.slots.is_empty() {
                if let Some(eaten_item) = self.inventory.consume_item(self.inv_selection) {
                    // Reduce hunger when a berry is eaten
                    self.hunger = (self.hunger - eaten_item.nutrition()).max(0.0);
                }
                self.inv_selection = self.inv_selection.min(self.inventory.slots.len().saturating_sub(1));
            }
            return; 
        }

        // --- SETTINGS MENU LOGIC ---
        if self.mode == GameMode::SettingsMenu {
            if is_key_pressed(KeyCode::Down) { self.settings_selection = (self.settings_selection + 1).min(2); }
            if is_key_pressed(KeyCode::Up) { self.settings_selection = self.settings_selection.saturating_sub(1); }
            
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                match self.settings_selection {
                    0 => self.drain_hunger = !self.drain_hunger,
                    1 => self.drain_thirst = !self.drain_thirst,
                    2 => self.drain_sleep = !self.drain_sleep,
                    _ => {}
                }
            }
            return;
        }

        // --- SURVIVAL DRAIN LOGIC (Runs only in Play Mode) ---
        let dt = get_frame_time();
        
        if self.drain_hunger { self.hunger = (self.hunger + 0.5 * dt).min(100.0); }
        if self.drain_thirst { self.thirst = (self.thirst + 0.8 * dt).min(100.0); }
        if self.drain_sleep { self.sleep = (self.sleep + 0.2 * dt).min(100.0); }

        // Energy naturally regenerates if you are standing still, 
        // BUT regeneration gets slower the higher your other stats (needs) are.
        let total_penalty = (self.hunger + self.thirst + self.sleep) / 300.0; // 0.0 to 1.0 penalty
        let regen_rate = 15.0 * (1.0 - total_penalty).max(0.0); // No regen if you are totally dying
        
        if !self.is_moving {
            self.energy = (self.energy + regen_rate * dt).min(100.0);
        }

        // --- PLAY MOVEMENT & INTERACT LOGIC ---
        let tile_size = 64.0;
        let speed = 250.0 * dt; 

        if self.is_moving {
            self.anim_timer += dt;
            // Drain energy while moving
            self.energy = (self.energy - 10.0 * dt).max(0.0);

            let target_x = self.player_grid_x as f32 * tile_size;
            let target_y = self.player_grid_y as f32 * tile_size;
            
            if self.player_pixel_x < target_x { self.player_pixel_x += speed; }
            if self.player_pixel_x > target_x { self.player_pixel_x -= speed; }
            if self.player_pixel_y < target_y { self.player_pixel_y += speed; }
            if self.player_pixel_y > target_y { self.player_pixel_y -= speed; }
            
            if (self.player_pixel_x - target_x).abs() < speed && (self.player_pixel_y - target_y).abs() < speed {
                self.player_pixel_x = target_x;
                self.player_pixel_y = target_y;
                self.is_moving = false;
                self.last_move_time = get_time(); 
            }
        } else {
            if is_key_pressed(KeyCode::Space) {
                let mut face_x = self.player_grid_x;
                let mut face_y = self.player_grid_y;
                
                match self.player_dir {
                    Direction::Right => face_x += 1,
                    Direction::Left => face_x = face_x.saturating_sub(1), 
                    Direction::Down => face_y += 1,
                    Direction::Up => face_y = face_y.saturating_sub(1),
                }
                
                if face_x < 20 && face_y < 20 {
                    let faced_tile = self.grid[face_x][face_y];
                    
                    if faced_tile == Tile::Bush {
                        // Harvest berry
                        if self.inventory.try_add_item(Item::Berry) {
                            self.grid[face_x][face_y] = Tile::Empty;
                        }
                    } else if faced_tile == Tile::Water {
                        // Drink water to instantly reduce thirst
                        self.thirst = (self.thirst - 25.0).max(0.0);
                    }
                }
            }

            let mut next_x = self.player_grid_x;
            let mut next_y = self.player_grid_y;
            let mut intended_dir = self.player_dir;
            let mut wants_to_move = false;

            if is_key_down(KeyCode::Right) { intended_dir = Direction::Right; wants_to_move = true; }
            else if is_key_down(KeyCode::Left) { intended_dir = Direction::Left; wants_to_move = true; }
            else if is_key_down(KeyCode::Down) { intended_dir = Direction::Down; wants_to_move = true; }
            else if is_key_down(KeyCode::Up) { intended_dir = Direction::Up; wants_to_move = true; }

            // You can only start a move if you have enough energy!
            if wants_to_move && self.energy > 2.0 {
                if self.player_dir != intended_dir {
                    self.player_dir = intended_dir;
                    let is_chaining = (get_time() - self.last_move_time) < 0.05;
                    if !is_chaining { self.movement_cooldown = get_time() + 0.1; }
                }

                if get_time() >= self.movement_cooldown {
                    match intended_dir {
                        Direction::Right => next_x += 1,
                        Direction::Left => next_x = next_x.saturating_sub(1), 
                        Direction::Down => next_y += 1,
                        Direction::Up => next_y = next_y.saturating_sub(1),
                    }

                    if next_x < 20 && next_y < 20 {
                        if self.grid[next_x][next_y] == Tile::Empty {
                            self.player_grid_x = next_x;
                            self.player_grid_y = next_y;
                            self.is_moving = true;
                            self.left_foot = !self.left_foot;
                            self.anim_timer = 0.0; 
                        }
                    }
                }
            } else {
                self.movement_cooldown = 0.0;
            }
        }
    }
}

async fn load_texture_or_fallback(path: &str, width: u16, height: u16, fallback_color: Color) -> Texture2D {
    match load_texture(path).await {
        Ok(texture) => { texture.set_filter(FilterMode::Nearest); texture }
        Err(_) => {
            let image = Image::gen_image_color(width, height, fallback_color);
            let texture = Texture2D::from_image(&image);
            texture.set_filter(FilterMode::Nearest);
            texture
        }
    }
}

// ==========================================
// MAIN GAME LOOP & GRAPHICS
// ==========================================
#[macroquad::main("Grid World")]
async fn main() {
    let tile_size = 64.0; 
    let player_sprite = load_texture_or_fallback("assets/player.png", 96, 128, MAGENTA).await;
    let grass_sprite = load_texture_or_fallback("assets/grass1.png", 32, 32, DARKGREEN).await;
    let water_sprite = load_texture_or_fallback("assets/water1.png", 32, 32, BLUE).await;
    let bush_sprite = load_texture_or_fallback("assets/bush1.png", 32, 32, GREEN).await;
    let berry_sprite = load_texture_or_fallback("assets/berry1.png", 32, 32, RED).await;

    let mut state = GameState::new();

    loop {
        state.update();
        clear_background(BLACK);

        // --- RENDER WORLD ---
        let camera_offset_x = state.player_pixel_x + (tile_size / 2.0) - (screen_width() / 2.0);
        let camera_offset_y = state.player_pixel_y + (tile_size / 2.0) - (screen_height() / 2.0);

        for x in 0..20 {
            for y in 0..20 {
                let screen_x = (x as f32 * tile_size) - camera_offset_x;
                let screen_y = (y as f32 * tile_size) - camera_offset_y;
                
                if screen_x < -tile_size || screen_x > screen_width() || screen_y < -tile_size || screen_y > screen_height() { continue; }
                
                draw_texture_ex(&grass_sprite, screen_x, screen_y, WHITE, DrawTextureParams { dest_size: Some(vec2(tile_size, tile_size)), ..Default::default() });
                
                match state.grid[x][y] {
                    Tile::Bush => draw_texture_ex(&bush_sprite, screen_x, screen_y, WHITE, DrawTextureParams { dest_size: Some(vec2(tile_size, tile_size)), ..Default::default() }),
                    Tile::Water => draw_texture_ex(&water_sprite, screen_x, screen_y, WHITE, DrawTextureParams { dest_size: Some(vec2(tile_size, tile_size)), ..Default::default() }),
                    Tile::Empty => {} 
                }
            }
        }

        // --- RENDER PLAYER ---
        let frame_row = match state.player_dir { Direction::Down => 0.0, Direction::Left => 1.0, Direction::Up => 2.0, Direction::Right => 3.0 };
        let mut frame_col = 1.0; 
        if state.is_moving {
            if state.anim_timer < 0.13 { if state.left_foot { frame_col = 0.0; } else { frame_col = 2.0; } } 
            else { frame_col = 1.0; }
        }

        let source_rect = Rect::new(frame_col * 32.0, frame_row * 32.0, 32.0, 32.0);
        let center_screen_x = (screen_width() / 2.0) - (tile_size / 2.0);
        let center_screen_y = (screen_height() / 2.0) - (tile_size / 2.0);
        draw_texture_ex(&player_sprite, center_screen_x, center_screen_y, WHITE, DrawTextureParams { dest_size: Some(vec2(tile_size, tile_size)), source: Some(source_rect), ..Default::default() });

        // --- RENDER HUD (SURVIVAL STATS) ---
        if state.mode == GameMode::Play {
            draw_rectangle(10.0, 10.0, 220.0, 120.0, Color::new(0.0, 0.0, 0.0, 0.7));
            draw_text("Survival Stats (TAB for Settings)", 15.0, 25.0, 16.0, LIGHTGRAY);
            
            // Helper function for drawing bars
            let draw_bar = |y: f32, label: &str, val: f32, color: Color, invert_good: bool| {
                draw_text(label, 15.0, y + 12.0, 18.0, WHITE);
                draw_rectangle(80.0, y, 100.0, 15.0, DARKGRAY);
                
                // If it's energy, high is good. If it's hunger, low is good.
                let bar_color = if invert_good { if val > 80.0 { RED } else { color } } 
                                else { if val < 20.0 { RED } else { color } };
                                
                draw_rectangle(80.0, y, val, 15.0, bar_color);
            };

            draw_bar(40.0, "Hunger:", state.hunger, ORANGE, true);
            draw_bar(60.0, "Thirst:", state.thirst, SKYBLUE, true);
            draw_bar(80.0, "Sleep:", state.sleep, PURPLE, true);
            draw_bar(100.0, "Energy:", state.energy, GREEN, false);
        }

        // --- RENDER SETTINGS MENU ---
        if state.mode == GameMode::SettingsMenu {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.8));
            
            let panel_w = 400.0;
            let panel_h = 250.0;
            let panel_x = (screen_width() - panel_w) / 2.0;
            let panel_y = (screen_height() - panel_h) / 2.0;
            
            draw_rectangle(panel_x, panel_y, panel_w, panel_h, DARKGRAY);
            draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 4.0, LIGHTGRAY);
            
            draw_text("Simulation Settings", panel_x + 20.0, panel_y + 40.0, 40.0, WHITE);
            draw_text("Use Arrows & Space to Toggle", panel_x + 20.0, panel_y + 70.0, 20.0, LIGHTGRAY);

            let options = [
                ("Drain Hunger", state.drain_hunger),
                ("Drain Thirst", state.drain_thirst),
                ("Drain Sleep", state.drain_sleep),
            ];

            for (i, (label, is_on)) in options.iter().enumerate() {
                let y_pos = panel_y + 120.0 + (i as f32 * 40.0);
                
                let text_color = if state.settings_selection == i { YELLOW } else { WHITE };
                let status_text = if *is_on { "[ ON ]" } else { "[ OFF ]" };
                let status_color = if *is_on { GREEN } else { RED };

                draw_text(label, panel_x + 40.0, y_pos, 30.0, text_color);
                draw_text(status_text, panel_x + 260.0, y_pos, 30.0, status_color);
                
                if state.settings_selection == i {
                    draw_text(">", panel_x + 15.0, y_pos, 30.0, YELLOW);
                }
            }
        }

        // --- RENDER INVENTORY MENU ---
        if state.mode == GameMode::InventoryMenu {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.8));
            
            let panel_w = 400.0;
            let panel_h = 350.0;
            let panel_x = (screen_width() - panel_w) / 2.0;
            let panel_y = (screen_height() - panel_h) / 2.0;
            
            draw_rectangle(panel_x, panel_y, panel_w, panel_h, DARKGRAY);
            draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 4.0, LIGHTGRAY);
            
            draw_text("Inventory", panel_x + 20.0, panel_y + 40.0, 40.0, WHITE);
            let cap_text = format!("Weight: {}/{}", state.inventory.current_weight(), state.inventory.max_capacity);
            draw_text(&cap_text, panel_x + 220.0, panel_y + 35.0, 25.0, YELLOW);
            
            let (mouse_x, mouse_y) = mouse_position();
            let mut hovered_item_name = "";

            let cols = 5;
            let slot_size = 60.0;
            let padding = 15.0;
            let start_x = panel_x + 20.0;
            let start_y = panel_y + 70.0;

            for (i, (item, count)) in state.inventory.slots.iter().enumerate() {
                let col = i % cols;
                let row = i / cols;
                let slot_x = start_x + (col as f32 * (slot_size + padding));
                let slot_y = start_y + (row as f32 * (slot_size + padding));
                let slot_rect = Rect::new(slot_x, slot_y, slot_size, slot_size);

                if slot_rect.contains(vec2(mouse_x, mouse_y)) { state.inv_selection = i; }
                let is_selected = state.inv_selection == i;

                let box_color = if is_selected { YELLOW } else { GRAY };
                draw_rectangle(slot_x, slot_y, slot_size, slot_size, box_color);
                draw_rectangle(slot_x + 2.0, slot_y + 2.0, slot_size - 4.0, slot_size - 4.0, BLACK);
                
                if is_selected { hovered_item_name = item.name(); }

                match item {
                    Item::Berry => {
                        draw_texture_ex(&berry_sprite, slot_x + 5.0, slot_y + 5.0, WHITE, DrawTextureParams { dest_size: Some(vec2(50.0, 50.0)), ..Default::default() });
                    }
                }

                if *count > 1 {
                    let count_text = format!("x{}", count);
                    draw_text(&count_text, slot_x + 35.0, slot_y + 55.0, 20.0, WHITE);
                }
            }

            if !hovered_item_name.is_empty() {
                draw_text(hovered_item_name, panel_x + 20.0, panel_y + panel_h - 20.0, 30.0, WHITE);
                draw_text("Press SPACE to eat", panel_x + 200.0, panel_y + panel_h - 20.0, 20.0, LIGHTGRAY);
            }
        }

        next_frame().await
    }
}