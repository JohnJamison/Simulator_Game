use macroquad::prelude::*;

mod world_gen;
mod inventory;
mod player;
mod state;
mod ui;

use world_gen::Tile;
use player::Direction;
use state::{GameState};

pub struct GameAsset { pub tex: Option<Texture2D>, pub name: String }

impl GameAsset {
    pub async fn load(path: &str, display_name: &str) -> Self {
        let tex = load_texture(path).await.ok();
        if let Some(t) = &tex { t.set_filter(FilterMode::Nearest); }
        GameAsset { tex, name: display_name.to_string() }
    }

    pub fn draw(&self, x: f32, y: f32, w: f32, h: f32, source: Option<Rect>) {
        if let Some(t) = &self.tex {
            draw_texture_ex(t, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(w, h)), source, ..Default::default() });
        } else {
            draw_rectangle(x, y, w, h, Color::new(0.2, 0.2, 0.2, 1.0));
            draw_rectangle_lines(x, y, w, h, 2.0, DARKGRAY);
            let tw = measure_text(&self.name, None, 14, 1.0).width;
            draw_text(&self.name, x + (w / 2.0) - (tw / 2.0), y + (h / 2.0), 14.0, WHITE);
        }
    }
}

enum Drawable { Player, Bush(usize, usize, bool), TreeTrunk(usize, usize) }

#[macroquad::main("Grid World")]
async fn main() {
    let tile_size = 64.0; 
    let player_sprite = GameAsset::load("assets/player.png", "Player").await;
    let fishing_sprite = GameAsset::load("assets/player_fishing.png", "Fishing").await; // NEW!
    let bobber_sprite = GameAsset::load("assets/bobber.png", "Bobber").await; // NEW!
    let grass_sprite = GameAsset::load("assets/grass1.png", "Grass").await;
    let water_sprite = GameAsset::load("assets/water1.png", "Water").await;
    let bush_sprite = GameAsset::load("assets/bush1.png", "Bush").await;
    let berry_bush_sprite = GameAsset::load("assets/berry_bush1.png", "BerryBush").await;
    let trunk_sprite = GameAsset::load("assets/trunk1.png", "Trunk").await;
    let tree_top_sprite = GameAsset::load("assets/tree_top1.png", "TreeTop").await;
    let berry_sprite = GameAsset::load("assets/berry1.png", "Berry").await;
    let twig_sprite = GameAsset::load("assets/twig.png", "Twig").await; // NEW!
    let rod_sprite = GameAsset::load("assets/rod.png", "Rod").await; // NEW!
    let fish_sprite = GameAsset::load("assets/fish.png", "Fish").await; // NEW!

    let mut state = GameState::new();

    loop {
        state.update();
        clear_background(BLACK);

        let camera_x = state.player.pixel_x + (tile_size / 2.0) - (screen_width() / 2.0);
        let camera_y = state.player.pixel_y + (tile_size / 2.0) - (screen_height() / 2.0);

        let mut drawables: Vec<(f32, Drawable)> = Vec::new();
        let mut tree_tops: Vec<(usize, usize)> = Vec::new();

        drawables.push((state.player.pixel_y + tile_size, Drawable::Player));

        for x in 0..20 {
            for y in 0..20 {
                let sx = (x as f32 * tile_size) - camera_x;
                let sy = (y as f32 * tile_size) - camera_y;
                
                if sx < -(tile_size * 3.0) || sx > screen_width() + (tile_size * 3.0) || 
                   sy < -(tile_size * 3.0) || sy > screen_height() + (tile_size * 3.0) { continue; }
                
                grass_sprite.draw(sx, sy, tile_size, tile_size, None);
                if state.grid[x][y] == Tile::Water { water_sprite.draw(sx, sy, tile_size, tile_size, None); }

                let sort_y = (y as f32 * tile_size) + tile_size;
                match state.grid[x][y] {
                    Tile::Bush => drawables.push((sort_y, Drawable::Bush(x, y, false))),
                    Tile::BerryBush => drawables.push((sort_y, Drawable::Bush(x, y, true))),
                    Tile::TreeTrunk => { drawables.push((sort_y, Drawable::TreeTrunk(x, y))); tree_tops.push((x, y)); },
                    _ => {} 
                }
            }
        }

        drawables.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for (_, drawable) in drawables {
            match drawable {
                Drawable::Player => {
                    let cx = (screen_width() / 2.0) - (tile_size / 2.0);
                    let cy = (screen_height() / 2.0) - (tile_size / 2.0);
                    
                    if state.player.is_fishing {
                        fishing_sprite.draw(cx, cy, tile_size, tile_size, None);
                        
                        // Calculate where the bobber should land!
                        let (fx, fy) = state.player.face_coord();
                        let bx = (fx as f32 * tile_size) - camera_x;
                        let by = (fy as f32 * tile_size) - camera_y;
                        bobber_sprite.draw(bx, by, tile_size, tile_size, None);
                    } else {
                        let row = match state.player.dir { Direction::Down => 0.0, Direction::Left => 1.0, Direction::Up => 2.0, Direction::Right => 3.0 };
                        let mut col = 1.0; 
                        if state.player.is_moving { if state.player.anim_timer < 0.13 { if state.player.left_foot { col = 0.0; } else { col = 2.0; } } else { col = 1.0; } }
                        player_sprite.draw(cx, cy, tile_size, tile_size, Some(Rect::new(col * 32.0, row * 32.0, 32.0, 32.0)));
                    }
                },
                Drawable::Bush(x, y, is_berry) => {
                    let sx = (x as f32 * tile_size) - camera_x; let sy = (y as f32 * tile_size) - camera_y;
                    let spr = if is_berry { &berry_bush_sprite } else { &bush_sprite };
                    spr.draw(sx, sy, tile_size, tile_size, None);
                },
                Drawable::TreeTrunk(x, y) => {
                    let sx = (x as f32 - 1.0) * tile_size - camera_x; let sy = (y as f32 - 2.0) * tile_size - camera_y;
                    trunk_sprite.draw(sx, sy, tile_size * 3.0, tile_size * 3.0, None);
                }
            }
        }

        for (x, y) in tree_tops {
            let sx = (x as f32 - 1.0) * tile_size - camera_x; let sy = (y as f32 - 2.0) * tile_size - camera_y;
            tree_top_sprite.draw(sx, sy, tile_size * 3.0, tile_size * 3.0, None);
        }

        ui::draw_ui(&state, &berry_sprite, &twig_sprite, &rod_sprite, &fish_sprite);
        next_frame().await
    }
}