use macroquad::prelude::Rect;

pub fn hitbox(x: usize, y:usize, tile_size: f32) -> Option<Rect> {
    Some(Rect::new((x as f32 * tile_size) + 24.0, y as f32 * tile_size, 78.0, 64.0))
}