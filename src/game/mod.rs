use bevy::prelude::*;
pub mod block;
mod board;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(board::BoardPlugin)
        .add_plugin(block::BlockPlugin)
        .insert_resource(ScoreRes::default())
        .add_startup_system(startup_system);
    }
}

pub const BLOCK_SIZE: f32 = 20.;
pub const GAME_SIZE: (usize, usize) = (5,5); // (5, 25);
pub const GAME_SPEED: f32 = 1.;
pub const DOWN_KEY_MULTIPLIER: f32 = 3.;

pub struct ScoreRes (usize);

impl Default for ScoreRes {
    fn default() -> Self {
        Self(0)
    }
}

fn startup_system(
    mut window: ResMut<Windows>
) {
    // Set window size
    let window = window.get_primary_mut().unwrap();
    window.set_resolution(GAME_SIZE.0 as f32, GAME_SIZE.1 as f32);
    window.set_title("Tetris GamPlah".to_string());

}

