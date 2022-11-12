use std::time::Duration;

use bevy::prelude::*;

use super::{board::{BoardRes, self}, DOWN_KEY_MULTIPLIER, GAME_SIZE, BLOCK_SIZE};
use crate::game::board::BoardEntities;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(blocks_system);
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

const BLOCK_LEN: (usize, usize) = (3, 3);

fn blocks_system (
    mut boardres: ResMut<BoardRes>,
    mut timer: Local<Timer>,
    time: Res<Time>,
    kb: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    let mut board = boardres.board;
    let mut should_move = true;

    let mut row_index = 0;
    for row in board.iter() {
        
        let mut value_index = 0;
        for value in row.iter() {
            if let Some(be) = value {
                // if row_index ==GAME_SIZE.1-1 && !be.moving {
                //     todo!(); // Game over
                // }
                if be.moving {
                    let valuebelow = match board.get(row_index + 1) {
                        Some(a) => a,
                        None => {dbg!("GET YO BANANA! THIS IS WAR!"); should_move = false; continue;},
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

        // Clean up the board
        for row in board.iter_mut() {
            for value in row.iter_mut() {
                if let Some(be) = value.as_mut() {
                    if be.moving {
                        be.moving = false;
                    }
                }
                
                
            }
        }

        spawn_blocks(&mut board);

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

        show_blocks(commands, boardres);

        dbg!(board);
    

    } else if kb.pressed(KeyCode::Down){
        timer.tick(
            Duration::from_millis(
                ((time.delta_seconds() * DOWN_KEY_MULTIPLIER) * 1000. ) as u64)
        );

    } else {
        timer.tick(time.delta());
    }
}
fn show_blocks (
    mut commands: Commands,
    mut boardres: ResMut<BoardRes>,
) {
    let board = &mut boardres.board;

    // Despawn all entities
    for y in 0..GAME_SIZE.1 {
        for x in 0..GAME_SIZE.0 {
            if let Some(be) = board[y][x] {
                if let Some(entity) = be.spawned {
                    commands.entity(entity).despawn();
                    board[y][x].as_mut().unwrap().spawned = None;
                }
            }
        }
    }

    // Spawn new ones
    for y in 0..GAME_SIZE.1 {
        for x in 0..GAME_SIZE.0 {
            if let Some(be) = board[y][x] {
                if be.spawned.is_some() {
                    panic!("The block at {} {} is still spawned!", x, y);
                }
                let entity = commands.spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::from((x as f32 * BLOCK_SIZE, y as f32 * -BLOCK_SIZE, 0f32)), // The y is upside down
                        // rotation: Quat::from_rotation_z(2f32),
                        scale: Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 0f32),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: board[y][x].unwrap().color,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id();

                board[y][x].as_mut().unwrap().spawned = Some(entity);
            }
        }
    }
}

pub fn spawn_blocks (
    board: &mut [[Option<BoardEntities>; GAME_SIZE.0]; GAME_SIZE.1]
) {

    dbg!("Will spawn!");
    let default = // TODO! Make this random
    BoardEntities { moving: true, color: Color::rgb(1., 1., 1.), spawned: None };
    let block = BLOCK_TYPES[0];

    let upperleft = GAME_SIZE.0 / 2 - 1; // The place where the blocks spawn

    for y in 0..BLOCK_LEN.0 {
        for x in 0..BLOCK_LEN.1 {
            if block[y][x] {
                board[y][x + upperleft] = Some(default);
            }
        }
    }

    dbg!(board);
}