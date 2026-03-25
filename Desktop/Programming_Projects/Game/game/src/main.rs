use macroquad::prelude::*;

// ==========================================
// GAME DATA STRUCTURES
// ==========================================

#[derive(PartialEq)]
enum GameMode {
    Play,
    InventoryMenu,
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
    fn weight(&self) -> u32 {
        match self {
            Item::Berry => 1,
        }
    }
    
    fn name(&self) -> &str {
        match self {
            Item::Berry => "Fresh Berry",
        }
    }
}

struct Inventory {
    slots: Vec<(Item, u32)>, 
    max_capacity: u32,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            slots: Vec::new(),
            max_capacity: 30, 
        }
    }

    fn current_weight(&self) -> u32 {
        let mut total = 0;
        for (item, count) in &self.slots {
            total += item.weight() * count;
        }
        total
    }

    fn try_add_item(&mut self, item: Item) -> bool {
        if self.current_weight() + item.weight() <= self.max_capacity {
            for slot in &mut self.slots {
                if slot.0 == item {
                    slot.1 += 1;
                    return true;
                }
            }
            self.slots.push((item, 1));
            true
        } else {
            false
        }
    }
    
    fn consume_item(&mut self, index: usize) {
        if index < self.slots.len() {
            self.slots[index].1 -= 1;
            if self.slots[index].1 == 0 {
                self.slots.remove(index);
            }
        }
    }
}

struct GameState {
    mode: GameMode,
    inventory: Inventory,
    inv_selection: usize, 
    
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
            
            grid: [[Tile::Empty; 20]; 20],
            player_grid_x: 10,
            player_grid_y: 10,
            player_pixel_x: 800.0, 
            player_pixel_y: 800.0,
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
        if is_key_pressed(KeyCode::E) {
            if self.mode == GameMode::Play {
                self.mode = GameMode::InventoryMenu;
                self.inv_selection = 0; 
            } else {
                self.mode = GameMode::Play;
            }
        }

        if self.mode == GameMode::InventoryMenu {
            let max_idx = self.inventory.slots.len().saturating_sub(1);
            
            if is_key_pressed(KeyCode::Right) {
                self.inv_selection = (self.inv_selection + 1).min(max_idx);
            }
            if is_key_pressed(KeyCode::Left) {
                self.inv_selection = self.inv_selection.saturating_sub(1);
            }
            if is_key_pressed(KeyCode::Down) {
                self.inv_selection = (self.inv_selection + 5).min(max_idx);
            }
            if is_key_pressed(KeyCode::Up) {
                self.inv_selection = self.inv_selection.saturating_sub(5);
            }

            if is_key_pressed(KeyCode::Space) && !self.inventory.slots.is_empty() {
                self.inventory.consume_item(self.inv_selection);
                self.inv_selection = self.inv_selection.min(self.inventory.slots.len().saturating_sub(1));
            }
            
            return; 
        }

        let tile_size = 80.0;
        let speed = 300.0 * get_frame_time(); 

        if self.is_moving {
            self.anim_timer += get_frame_time();
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
                
                if face_x < 20 && face_y < 20 && self.grid[face_x][face_y] == Tile::Bush {
                    if self.inventory.try_add_item(Item::Berry) {
                        self.grid[face_x][face_y] = Tile::Empty;
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

            if wants_to_move {
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
                        let target_tile = self.grid[next_x][next_y];
                        if target_tile == Tile::Empty || target_tile == Tile::Bush {
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

// --- NEW FALLBACK TEXTURE HELPER ---
// We match the result of the load attempt. If it fails, we quickly draw a colored rectangle in memory 
// to use as our texture instead, preventing the panic unwrap error.
async fn load_texture_or_fallback(path: &str, width: u16, height: u16, fallback_color: Color) -> Texture2D {
    match load_texture(path).await {
        Ok(texture) => {
            texture.set_filter(FilterMode::Nearest);
            texture
        }
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
    let tile_size = 80.0; 
    
    // We pass the expected dimensions of the sprite so the fallback rectangle is the right size.
    // The player needs to be 120x160 to fit all the animation frames without crashing the crop math.
    let player_sprite = load_texture_or_fallback("assets/player.png", 120, 160, MAGENTA).await;
    let grass_sprite = load_texture_or_fallback("assets/grass1.png", 40, 40, DARKGREEN).await;
    let water_sprite = load_texture_or_fallback("assets/water1.png", 40, 40, BLUE).await;
    let bush_sprite = load_texture_or_fallback("assets/bush1.png", 40, 40, GREEN).await;
    let berry_sprite = load_texture_or_fallback("assets/berry1.png", 40, 40, RED).await;

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

        let source_rect = Rect::new(frame_col * 40.0, frame_row * 40.0, 40.0, 40.0);
        let center_screen_x = (screen_width() / 2.0) - (tile_size / 2.0);
        let center_screen_y = (screen_height() / 2.0) - (tile_size / 2.0);

        draw_texture_ex(&player_sprite, center_screen_x, center_screen_y, WHITE, DrawTextureParams { dest_size: Some(vec2(tile_size, tile_size)), source: Some(source_rect), ..Default::default() });

        // --- RENDER INVENTORY OVERLAY ---
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

                if slot_rect.contains(vec2(mouse_x, mouse_y)) {
                    state.inv_selection = i; 
                }
                
                let is_selected = state.inv_selection == i;

                let box_color = if is_selected { YELLOW } else { GRAY };
                draw_rectangle(slot_x, slot_y, slot_size, slot_size, box_color);
                draw_rectangle(slot_x + 2.0, slot_y + 2.0, slot_size - 4.0, slot_size - 4.0, BLACK);
                
                if is_selected {
                    hovered_item_name = item.name();
                }

                match item {
                    Item::Berry => {
                        draw_texture_ex(&berry_sprite, slot_x + 5.0, slot_y + 5.0, WHITE, 
                            DrawTextureParams { dest_size: Some(vec2(50.0, 50.0)), ..Default::default() });
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