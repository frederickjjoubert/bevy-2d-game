use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::{
    Map,
    TileType,
};

pub const TILE_SIZE: i32 = 16;

pub fn draw_map_system(mut commands: Commands, asset_server: Res<AssetServer>, map: Res<Map>) {
    let tile_map = commands.spawn().id();
    commands.entity(tile_map).insert(Name::new("TileMap"));

    let mut tiles: Vec<Entity> = Vec::new();

    for j in 0..map.height {
        for i in 0..map.width {
            let index = map.xy_idx(i, j);

            let x_offset: f32 = ((i - (map.width / 2)) * TILE_SIZE) as f32;
            let y_offset: f32 = ((j - (map.height / 2)) * TILE_SIZE) as f32;

            let tile_entity = commands.spawn().id();
            commands.entity(tile_entity).insert(Name::new("Tile"));

            let tile_type = map.tiles[index];
            match tile_type {
                TileType::Floor => {
                    commands.entity(tile_entity)
                        .insert_bundle(
                            SpriteBundle
                            {
                                texture: asset_server.load("sprites/floor_1.png"),
                                transform: Transform::from_xyz(x_offset, y_offset, 0.),
                                ..Default::default()
                            }
                        );
                }
                TileType::Wall => {
                    commands.entity(tile_entity)
                        .insert_bundle(
                            SpriteBundle
                            {
                                texture: asset_server.load("sprites/wall_mid.png"),
                                transform: Transform::from_xyz(x_offset, y_offset, 0.),
                                ..Default::default()
                            }
                        )
                        .insert(RigidBody::Fixed)
                        .insert(Collider::cuboid((TILE_SIZE / 2) as f32, (TILE_SIZE / 2) as f32));
                }
                _ => {}
            }

            tiles.push(tile_entity);

            // println!("{},{}", i, j);
        }
    }

    commands.entity(tile_map).push_children(&tiles);
}
