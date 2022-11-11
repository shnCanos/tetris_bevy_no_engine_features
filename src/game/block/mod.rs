use std::time::Duration;

use bevy::prelude::*;

use super::{board::{BoardRes, self}, DOWN_KEY_MULTIPLIER, GAME_SIZE, BLOCK_SIZE};
use crate::game::board::BoardEntities;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ShouldSpawn (true))
        .add_system(block_physics);
    }
}

const BLOCK_TYPES: [[[bool; 3]; 3]; 2] = [
    [
        [false, true, false],
        [false, true, false],
        [false, true, false]
    ],
    [
        [true, true, false],
        [false, true, false],
        [false, true,  true]
    ]
];

pub struct ShouldSpawn (bool);

fn block_physics (
    mut boardres: ResMut<BoardRes>,
    mut timer: Local<Timer>,
    mut should_spawn: ResMut<ShouldSpawn>,
    time: Res<Time>,
    kb: Res<Input<KeyCode>>
) {
    let mut board = boardres.board;
    let mut should_move = true;

    let mut row_index = 0;
    for row in board.iter() {
        
        let mut value_index = 0;
        for value in row.iter() {
            if let Some(be) = value {
                if row_index == 0 && !be.moving {
                    todo!(); // Game over
                }
                if be.moving {
                    let valuebelow = match board.get(row_index + 1) {
                        Some(a) => a,
                        None => {dbg!("GET YO BANANA! THIS IS WAR!"); should_move = false; break;},
                    };
                    if let Some(bebelow) = valuebelow[value_index] {
                        if !bebelow.moving {
                            should_move = false;
                        }
                        
                    }
                }
            }
            
            value_index += 1;
        }

        row_index += 1;
    }

    if !should_move {
        should_spawn.0 = true;

        // Clean up the board
        for row in board.iter_mut() {
            for value in row.iter_mut() {
                if let Some(be) = value {
                    if be.moving {
                        be.moving = false;
                    }
                }
                
                
            }
        }

        boardres.board = board;
        
        timer.reset();
        *timer = Timer::from_seconds(super::GAME_SPEED, false);

        return;
    }

    if timer.finished() {
        // Move the block
        let mut row_index = 0;
        let boardclone = board;
        for row in boardclone.iter() {
        
            let mut value_index = 0;
            for value in row.iter() {
                if let Some(be) = value {
                    if be.moving {
                        board[row_index][value_index] = None;
                        board[row_index+1][value_index] = *value;
                    }

                }
            value_index += 1;
            }

        row_index += 1;
        }

    *timer = Timer::from_seconds(super::GAME_SPEED, false);
    boardres.board = board;
    

    } else if kb.pressed(KeyCode::Down){
        timer.tick(
            Duration::from_millis(
                ((time.delta_seconds() * DOWN_KEY_MULTIPLIER) * 1000. ) as u64)
        );

    } else {
        timer.tick(time.delta());
    }
}

#[derive(Component)]
struct Block;

fn show_blocks_system (
    mut commands: Commands,
    mut boardres: ResMut<BoardRes>,
    mut query: Query<(&Entity, &mut Transform), With<Block>>
) {
    let board = boardres.board;
    for (entity, mut block) in query.iter_mut() {
        
    }
}

fn find_entity_index (
    entity: Entity,
    board: [[Option<BoardEntities>; GAME_SIZE.0]; GAME_SIZE.1]
) -> (usize, usize) {
    for y in 0..GAME_SIZE.1 {
        for x in 0..GAME_SIZE.0 {
            if let Some(be) = board[y][x] {
                if let Some(entity) = be.spawned {
                    
                }
            }
        }
    }
}