use macroquad::prelude::*;
use crate::state::{GameState, GameMode};
use crate::inventory::Item;
use crate::GameAsset;

pub fn draw_ui(state: &GameState, berry_spr: &GameAsset, twig_spr: &GameAsset, rod_spr: &GameAsset, fish_spr: &GameAsset) {
    if state.mode == GameMode::Play {
        draw_rectangle(10.0, 10.0, 220.0, 120.0, Color::new(0.0, 0.0, 0.0, 0.7));
        draw_text("Stats (TAB: Settings, E: Inv)", 15.0, 25.0, 16.0, LIGHTGRAY);
        
        let draw_bar = |y: f32, label: &str, val: f32, color: Color, invert_good: bool| {
            draw_text(label, 15.0, y + 12.0, 18.0, WHITE);
            draw_rectangle(80.0, y, 100.0, 15.0, DARKGRAY);
            let bar_color = if invert_good { if val > 80.0 { RED } else { color } } else { if val < 20.0 { RED } else { color } };
            draw_rectangle(80.0, y, val, 15.0, bar_color);
        };
        draw_bar(40.0, "Hunger:", state.player.hunger, ORANGE, true);
        draw_bar(60.0, "Thirst:", state.player.thirst, SKYBLUE, true);
        draw_bar(80.0, "Sleep:", state.player.sleep, PURPLE, true);
        draw_bar(100.0, "Energy:", state.player.energy, GREEN, false);
    }

    if state.mode != GameMode::Play {
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.8));
        
        // Dynamic Panel Sizing based on menu
        let panel_w = 400.0; 
        let panel_h = if state.mode == GameMode::SettingsMenu { 250.0 } else { 350.0 };
        let panel_x = (screen_width() - panel_w) / 2.0; 
        let panel_y = (screen_height() - panel_h) / 2.0;
        
        draw_rectangle(panel_x, panel_y, panel_w, panel_h, DARKGRAY);
        draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 4.0, LIGHTGRAY);

        // --- DRAW BACK BUTTON ---
        draw_rectangle(panel_x + 310.0, panel_y + 10.0, 80.0, 30.0, MAROON);
        draw_text("BACK", panel_x + 325.0, panel_y + 32.0, 20.0, WHITE);

        if state.mode == GameMode::SettingsMenu {
            draw_text("Settings", panel_x + 20.0, panel_y + 40.0, 40.0, WHITE);
            let options = [("Drain Hunger", state.drain_hunger), ("Drain Thirst", state.drain_thirst), ("Drain Sleep", state.drain_sleep)];
            for (i, (label, is_on)) in options.iter().enumerate() {
                let y_pos = panel_y + 120.0 + (i as f32 * 40.0);
                draw_text(label, panel_x + 40.0, y_pos, 30.0, if state.settings_selection == i { YELLOW } else { WHITE });
                draw_text(if *is_on { "[ ON ]" } else { "[ OFF ]" }, panel_x + 260.0, y_pos, 30.0, if *is_on { GREEN } else { RED });
                if state.settings_selection == i { draw_text(">", panel_x + 15.0, y_pos, 30.0, YELLOW); }
            }
        } else if state.mode == GameMode::CraftingMenu {
            draw_text("Crafting", panel_x + 20.0, panel_y + 40.0, 40.0, WHITE);
            draw_text("Double-Click to Craft", panel_x + 20.0, panel_y + 70.0, 18.0, LIGHTGRAY);
            let has_twigs = state.inventory.count_item(Item::Twig) >= 2;
            let availability = if has_twigs { "(Available)" } else { "(Need 2 Twigs)" };
            draw_text(&format!("Fishing Rod {}", availability), panel_x + 40.0, panel_y + 120.0, 25.0, if state.craft_selection == 0 { YELLOW } else { WHITE });
            if state.craft_selection == 0 { draw_text(">", panel_x + 15.0, panel_y + 120.0, 25.0, YELLOW); }
        } else if state.mode == GameMode::InventoryMenu {
            draw_text("Inventory", panel_x + 20.0, panel_y + 40.0, 40.0, WHITE);
            draw_text(&format!("Weight: {}/{}", state.inventory.current_weight(), state.inventory.max_capacity), panel_x + 20.0, panel_y + 65.0, 20.0, YELLOW);
            if let Some(eq) = state.inventory.equipped { draw_text(&format!("Equipped: {}", eq.name()), panel_x + 20.0, panel_y + panel_h - 40.0, 20.0, SKYBLUE); }
            
            // Craft Button
            draw_rectangle(panel_x + 220.0, panel_y + 10.0, 80.0, 30.0, GRAY);
            draw_text("CRAFT", panel_x + 230.0, panel_y + 32.0, 20.0, WHITE);

            let (mouse_x, mouse_y) = mouse_position();
            let mut hovered_item_name = "";

            for (i, (item, count)) in state.inventory.slots.iter().enumerate() {
                let slot_x = panel_x + 20.0 + ((i % 5) as f32 * 75.0);
                let slot_y = panel_y + 70.0 + ((i / 5) as f32 * 75.0);
                let is_sel = state.inv_selection == i;
                
                let slot_rect = Rect::new(slot_x, slot_y, 60.0, 60.0);
                if slot_rect.contains(vec2(mouse_x, mouse_y)) { hovered_item_name = item.name(); }

                draw_rectangle(slot_x, slot_y, 60.0, 60.0, if is_sel { YELLOW } else { GRAY });
                draw_rectangle(slot_x + 2.0, slot_y + 2.0, 56.0, 56.0, BLACK);
                
                let spr = match item { Item::Berry => berry_spr, Item::Twig => twig_spr, Item::FishingRod => rod_spr, Item::Fish => fish_spr };
                spr.draw(slot_x + 5.0, slot_y + 5.0, 50.0, 50.0, None);
                if *count > 1 { draw_text(&format!("x{}", count), slot_x + 35.0, slot_y + 55.0, 20.0, WHITE); }
            }
            if !hovered_item_name.is_empty() {
                draw_text(hovered_item_name, panel_x + 20.0, panel_y + panel_h - 15.0, 20.0, WHITE);
            }
        }
    }
}