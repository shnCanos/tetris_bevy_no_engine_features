use std::fmt::Debug;

use bevy::prelude::*;

use super::GAME_SIZE;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(create_board_system)
        .add_system(kill_lines_system);
    }
}

pub struct BoardRes {
    // bool: Whether the block is moving
    pub board: [[Option<BoardEntities>; GAME_SIZE.0]; GAME_SIZE.1],
}

#[derive(Clone, Copy)]
pub struct BoardEntities {
    pub moving: bool,
    pub color: Color, 
    pub spawned: Option<Entity>, 
}
impl Default for BoardEntities {
    fn default() -> Self {
        Self { moving: false, color: Color::Rgba { red: 1., green: 1., blue: 1., alpha: 1. }, spawned: None }
    }
}
impl Debug for BoardEntities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! (f, "moving: {}\ncolor: {:?}\nspawned: {:?}", self.moving, self.color, self.spawned)
    }
}

fn create_board_system (
    mut commands: Commands,
) {
    // Spawn board
    let mut board = [[None; GAME_SIZE.0]; GAME_SIZE.1];
    board[1] = [Some(BoardEntities { moving: true, color: Color::Rgba { red: 1., green: 1., blue: 1., alpha: 1. }, spawned: None }), Some(BoardEntities { moving: true, color: Color::Rgba { red: 1., green: 1., blue: 1., alpha: 1. }, spawned: None } ), None, None, None];
    board[2] = [None, None, None, None, None];
    //board[3] = [Some(BoardEntities { moving: true, color: Color::Rgba { red: 1., green: 1., blue: 1., alpha: 1. }, spawned: None }),Some(BoardEntities { moving: true, color: Color::Rgba { red: 1., green: 1., blue: 1., alpha: 1. }, spawned: None }),Some(BoardEntities { moving: true, color: Color::Rgba { red: 1., green: 1., blue: 1., alpha: 1. }, spawned: None }),Some(BoardEntities { moving: true, color: Color::Rgba { red: 1., green: 1., blue: 1., alpha: 1. }, spawned: None }),Some(BoardEntities { moving: true, color: Color::Rgba { red: 1., green: 1., blue: 1., alpha: 1. }, spawned: None })];
    board[4] = [Some(BoardEntities { moving: false, color: Color::Rgba { red: 1., green: 1., blue: 1., alpha: 1. }, spawned: None }), Some(BoardEntities { moving: false, color: Color::Rgba { red: 1., green: 1., blue: 1., alpha: 1. }, spawned: None }), None, None, None];
    commands.insert_resource(BoardRes { board });
}


// Passed a few tests
fn kill_lines_system(
    mut board: ResMut<BoardRes>,
) {
    let mut tbboard = board.board;
    
    let mut remove_lines = Vec::new();
    let mut index = 0;
    for row in tbboard.iter_mut() {
        if is_full(row){
            remove_lines.push(index);
        }
        index += 1;
    }
    
    if remove_lines.is_empty() {
        return;
    }
    
    for line in remove_lines.iter() {
        let mut last_row = [None; GAME_SIZE.0];
        let mut current_row: [Option<BoardEntities>;GAME_SIZE.0];
        let mut index = -1; // Starts in -1 because the new piece is now the 0
        
        for row in tbboard.iter_mut() {

            current_row = *row;
            *row = last_row;
            last_row = current_row;

            index += 1;

            if line == &index {
                break;
            }
        }

    }

    board.board = tbboard;
}

fn is_full (row: &mut [Option<BoardEntities>; GAME_SIZE.0]) -> bool {
    let mut is_full = true;
    for value in row.iter() {
        if let Some(be) = value {
            if be.moving {
                is_full = false;
                break;
            }
            continue;
        }
        is_full = false;

    }
    return is_full;
}