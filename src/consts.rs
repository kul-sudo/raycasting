use macroquad::prelude::*;
use std::f32::consts::PI;

pub const ROWS: usize = 15;
pub const COLUMNS: usize = 20;
pub const TILE_SIZE: f32 = 64.0;

pub const TILE_CHANCE: f32 = 0.1;
pub const PLAYER_STEP: f32 = 5.0;
pub const MOUSE_SENSITIVITY: f32 = 0.0008;
pub const VISIBILITY: f32 = 1500.0;
pub static FIELD: Rect = Rect::new(
    0.0,
    0.0,
    COLUMNS as f32 * TILE_SIZE,
    ROWS as f32 * TILE_SIZE,
);

pub const FOV: f32 = PI / 2.0;
