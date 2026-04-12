use macroquad::prelude::*;
use crate::environment::Tile;
use crate::inventory::{Inventory, Item};
use crate::player::Player;
use crate::environment::biomes::Biome;
use crate::WORLD_SIZE;

#[derive(PartialEq)]
pub enum GameMode { Play, InventoryMenu, SettingsMenu, CraftingMenu }

pub struct GameState {
    pub mode: GameMode,
    pub inventory: Inventory,
    pub player: Player,
    pub grid: [[Tile; WORLD_SIZE]; WORLD_SIZE],
    pub biome: Biome,
    
    pub drain_hunger: bool, pub drain_thirst: bool, pub drain_sleep: bool,
    
    // UI tracking for double clicks
    pub inv_selection: usize, 
    pub craft_selection: usize,
    pub settings_selection: usize, 
    pub last_click_time: f64,
    pub last_clicked_idx: Option<usize>,
}

impl GameState {
    pub fn new() -> Self {
        let starting_biome = Biome::default();
        let starting_grid = crate::world_gen::generate_wfc_grid(&starting_biome);

        GameState {
            mode: GameMode::Play,
            inventory: Inventory::new(),
            player: Player::new(),
            biome: starting_biome,
            grid: starting_grid,
            drain_hunger: true, drain_thirst: true, drain_sleep: true,
            inv_selection: 0, craft_selection: 0, settings_selection: 0,
            last_click_time: 0.0, last_clicked_idx: None,
        }
    }

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Escape) { self.mode = GameMode::Play; }

        if is_key_pressed(KeyCode::E) {
            match self.mode {
                GameMode::Play => { self.mode = GameMode::InventoryMenu; }
                GameMode::InventoryMenu => { self.mode = GameMode::Play; }
                GameMode::CraftingMenu => { self.mode = GameMode::InventoryMenu; } 
                _ => {}
            }
        }

        if is_key_pressed(KeyCode::Tab) {
            if self.mode == GameMode::Play {
                self.mode = GameMode::SettingsMenu; self.settings_selection = 0;
            } else if self.mode == GameMode::SettingsMenu {
                self.mode = GameMode::Play;
            }
        }

        match self.mode {
            GameMode::Play => {
                let drains = (self.drain_hunger, self.drain_thirst, self.drain_sleep);
                self.player.update(get_frame_time(), &mut self.grid, &mut self.inventory, drains);
            },
            GameMode::InventoryMenu => self.update_inventory_menu(),
            GameMode::CraftingMenu => self.update_crafting_menu(),
            GameMode::SettingsMenu => self.update_settings_menu(),
        }
    }

    fn update_inventory_menu(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        let panel_x = (screen_width() - 400.0) / 2.0; let panel_y = (screen_height() - 350.0) / 2.0;
        
        // BACK BUTTON
        let back_btn = Rect::new(panel_x + 310.0, panel_y + 10.0, 80.0, 30.0);
        if is_mouse_button_pressed(MouseButton::Left) && back_btn.contains(vec2(mouse_x, mouse_y)) {
            self.mode = GameMode::Play; return;
        }

        let craft_rect = Rect::new(panel_x + 220.0, panel_y + 10.0, 80.0, 30.0);
        if is_mouse_button_pressed(MouseButton::Left) && craft_rect.contains(vec2(mouse_x, mouse_y)) {
            self.mode = GameMode::CraftingMenu; self.craft_selection = 0; return;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            for i in 0..self.inventory.slots.len() {
                let slot_x = panel_x + 20.0 + ((i % 5) as f32 * 75.0);
                let slot_y = panel_y + 70.0 + ((i / 5) as f32 * 75.0);
                
                if Rect::new(slot_x, slot_y, 60.0, 60.0).contains(vec2(mouse_x, mouse_y)) {
                    let now = get_time();
                    if self.last_clicked_idx == Some(i) && (now - self.last_click_time) < 0.3 {
                        let current_item = self.inventory.slots[i].0;
                        if current_item.nutrition() > 0.0 {
                            if let Some(eaten) = self.inventory.consume_item(i) {
                                self.player.hunger = (self.player.hunger - eaten.nutrition()).max(0.0);
                            }
                        } else {
                            self.inventory.equipped = Some(current_item);
                        }
                    }
                    self.last_clicked_idx = Some(i); self.last_click_time = now; self.inv_selection = i;
                    break; 
                }
            }
        }
    }

    fn update_crafting_menu(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        let panel_x = (screen_width() - 400.0) / 2.0; let panel_y = (screen_height() - 350.0) / 2.0;
        
        // BACK BUTTON
        let back_btn = Rect::new(panel_x + 310.0, panel_y + 10.0, 80.0, 30.0);
        if is_mouse_button_pressed(MouseButton::Left) && back_btn.contains(vec2(mouse_x, mouse_y)) {
            self.mode = GameMode::InventoryMenu; return;
        }

        let has_twigs = self.inventory.count_item(Item::Twig) >= 2;
        if is_mouse_button_pressed(MouseButton::Left) && Rect::new(panel_x + 40.0, panel_y + 100.0, 300.0, 40.0).contains(vec2(mouse_x, mouse_y)) {
            let now = get_time();
            if self.last_clicked_idx == Some(999) && (now - self.last_click_time) < 0.3 {
                if has_twigs && self.inventory.current_weight() + Item::FishingRod.weight() <= self.inventory.max_capacity {
                    self.inventory.remove_items(Item::Twig, 2); 
                    self.inventory.try_add_item(Item::FishingRod);
                }
            }
            self.last_clicked_idx = Some(999); self.last_click_time = now; self.craft_selection = 0;
        }
    }

    fn update_settings_menu(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        let panel_x = (screen_width() - 400.0) / 2.0; let panel_y = (screen_height() - 250.0) / 2.0;
        
        // BACK BUTTON
        let back_btn = Rect::new(panel_x + 310.0, panel_y + 10.0, 80.0, 30.0);
        if is_mouse_button_pressed(MouseButton::Left) && back_btn.contains(vec2(mouse_x, mouse_y)) {
            self.mode = GameMode::Play; return;
        }

        if is_key_pressed(KeyCode::Down) { self.settings_selection = (self.settings_selection + 1).min(2); }
        if is_key_pressed(KeyCode::Up) { self.settings_selection = self.settings_selection.saturating_sub(1); }
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
            match self.settings_selection {
                0 => self.drain_hunger = !self.drain_hunger, 1 => self.drain_thirst = !self.drain_thirst,
                2 => self.drain_sleep = !self.drain_sleep, _ => {}
            }
        }
    }
}