use bevy::prelude::*;
use super::{Tile, Coordinate};

const OFFSET: i32 = 50;
const MAP_WIDTH: i32 = 100;
const MAP_HEIGHT: i32 = 100;
const TILE_WIDTH: i32 = 16;

pub fn generate_map_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    for j in 0 - OFFSET..MAP_HEIGHT - OFFSET {
        for i in 0 - OFFSET..MAP_WIDTH - OFFSET {
            let x_offset: f32 = (i * TILE_WIDTH) as f32;
            let y_offset: f32 = (j * TILE_WIDTH) as f32;
            commands.spawn_bundle(
                SpriteBundle
                {
                    texture: asset_server.load("sprites/floor_1.png"),
                    transform: Transform::from_xyz(x_offset, y_offset, 0.),
                    ..Default::default()
                }
            )
                .insert(Tile {
                    coordinate: Coordinate {
                        x: i,
                        y: j,
                    }
                });
        }
    }
}