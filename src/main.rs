use macroquad::prelude::*;

pub const WORLD_SIZE: usize = 41;

mod environment;
mod world_gen;
mod inventory;
mod player;
mod state;
mod ui;


use player::Direction;
use state::GameState;
use crate::environment::Tile;
use crate::environment::TileBehavior;


//==================================================================================
//                               Game Asset
//==================================================================================
//      Wrapper for all game assets. Charged with loading and displaying all assets 
// in the game. If an asset is not available, it's name will be displayed.
//==================================================================================
pub struct GameAsset { pub texture: Option<Texture2D>, pub name: String }
impl GameAsset {

    //      load    ------------------------------------------------------------------
    //  Async (so game not forced to wait) function that loads an asset's sprite
    //  from memory
    pub async fn load(path: &str, display_name: &str) -> Self {
        let texture = load_texture(path).await.ok();
        if let Some(t) = &texture { t.set_filter(FilterMode::Nearest); }
        GameAsset { texture, name: display_name.to_string() }
    }

    //      draw    ------------------------------------------------------------------
    //  Draw's the sprite for a given asset, or the name if sprite doesn't exit
    pub fn draw(&self, x: f32, y: f32, w: f32, h: f32, source: Option<Rect>) {
        if let Some(t) = &self.texture {
            draw_texture_ex(t, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(w, h)), source, ..Default::default() });
        } else {
            draw_rectangle(x, y, w, h, Color::new(0.2, 0.2, 0.2, 1.0));
            draw_rectangle_lines(x, y, w, h, 2.0, DARKGRAY);
            let tw = measure_text(&self.name, None, 14, 1.0).width;
            draw_text(&self.name, x + (w / 2.0) - (tw / 2.0), y + (h / 2.0), 14.0, WHITE);
        }
    }

    // Pass in the column and row from your sprite sheet (starting at 0)
    // Pass in how many tiles wide/tall the object is.
    pub fn sheet_rect(&self, col: f32, row: f32, width_in_tiles: f32, height_in_tiles: f32) -> Option<Rect> {
        let raw_tile_size = 32.0; // The actual pixel size of your PNG's grid
        
        Some(Rect::new(
            col * raw_tile_size, 
            row * raw_tile_size, 
            width_in_tiles * raw_tile_size, 
            height_in_tiles * raw_tile_size
        ))
    }
}

enum Drawable { Player, Bush(usize, usize, bool), TreeTrunk(usize, usize) }

#[macroquad::main("Game")]
async fn main() {

    macroquad::rand::srand(macroquad::miniquad::date::now() as u64);
    let tile_size = 64.0; 
    
    // Load the master environment sheet!
    let env_sheet = GameAsset::load("assets/master_sheet.png", "EnvSheet").await;
    let player_sprite = GameAsset::load("assets/player.png", "Player").await;
    let fishing_sprite = GameAsset::load("assets/player_fishing.png", "Fishing").await; // NEW!
    let bobber_sprite = GameAsset::load("assets/bobber.png", "Bobber").await; // NEW!
    //let bush_sprite = GameAsset::load("assets/bush1.png", "Bush").await;
    //let berry_bush_sprite = GameAsset::load("assets/berry_bush1.png", "BerryBush").await;
    let twig_sprite = GameAsset::load("assets/twig.png", "Twig").await; // NEW!
    let berry_sprite = GameAsset::load("assets/berry1.png", "Berry").await;
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

        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                let sx = ((x as f32 * tile_size) - camera_x).floor();
                let sy = ((y as f32 * tile_size) - camera_y).floor();
                
                if sx < -(tile_size * 3.0) || sx > screen_width() + (tile_size * 3.0) || 
                   sy < -(tile_size * 3.0) || sy > screen_height() + (tile_size * 3.0) { continue; }
                
                let grass_cutout = env_sheet.sheet_rect(3.0, 1.0, 1.0, 1.0);
                env_sheet.draw(sx, sy, tile_size, tile_size, grass_cutout);


                if let Some((col, row)) = state.grid[x][y].sheet_coords() {
                    let cutout = env_sheet.sheet_rect(col, row, 1.0, 1.0);
                    env_sheet.draw(sx, sy, tile_size, tile_size, cutout); 
                }

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

                //Player
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

                //Bush
                Drawable::Bush(x, y, is_berry) => {
                    let sx = (x as f32) * tile_size - camera_x; 
                    let sy = (y as f32 * tile_size) - camera_y;

                    let berry_bush_sprite = env_sheet.sheet_rect(0.0, 9.0, 1.0, 1.0);
                    let bush_sprite = env_sheet.sheet_rect(1.0, 9.0, 1.0, 1.0);

                    let spr = if is_berry { berry_bush_sprite } else { bush_sprite };
                    env_sheet.draw(sx, sy, tile_size, tile_size, spr);
                    
                },
                
                //TreeTrunk
                Drawable::TreeTrunk(x, y) => {
                    let sx = (x as f32) * tile_size - camera_x; 
                    let sy = (y as f32) * tile_size - camera_y;
                    
                    //sheet_rect(col, row, w, h)
                    let trunk_cutout = env_sheet.sheet_rect(0.0, 1.0, 2.0, 1.0);
                    env_sheet.draw(sx, sy, tile_size * 2.0, tile_size * 1.0, trunk_cutout);
                }
            }
        }

        for (x, y) in tree_tops {
            // X stays exactly the same so it lines up with the trunk
            let sx = (x as f32) * tile_size - camera_x; 
            // Y gets shifted UP by 1 full tile so it sits on top of the trunk!
            let sy = (y as f32 - 1.0) * tile_size - camera_y;
            
            // Your Canopy: Starts at Column 1, Row 1. It is 2 wide and 1 tall.
            let canopy_cutout = env_sheet.sheet_rect(0.0, 0.0, 2.0, 1.0);
            
            // Draw it 2 tiles wide and 1 tile tall on the screen
            env_sheet.draw(sx, sy, tile_size * 2.0, tile_size * 1.0, canopy_cutout);
        }

        ui::draw_ui(&state, &berry_sprite, &twig_sprite, &rod_sprite, &fish_sprite);
        next_frame().await
    }
}