mod camera_follow_system;
mod components;
mod coordinate;
mod generate_map_system;
mod player_movement_system;
mod player_sprite_system;

use camera_follow_system::*;
pub use components::*;
pub use coordinate::*;
use generate_map_system::*;
use player_movement_system::*;
use player_sprite_system::*;

use bevy::prelude::*;
use bevy::window::PresentMode;


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy 2D Game".to_string(),
            width: 1600.,
            height: 900.,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(generate_map_system)
        .add_system(player_movement_system.label("player_movement_system"))
        .add_system(player_sprite_system.after("player_movement_system"))
        .add_system(camera_follow_system.after("player_movement_system"))
        .run();
}

// === Systems ===

// Startup Systems
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn Camera
    commands.spawn_bundle(Camera2dBundle::default());
    // Spawn Player
    commands.spawn_bundle(
        SpriteBundle {
            texture: asset_server.load("sprites/elf_m_idle_anim_f0.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        }
    )
        .insert(CameraFollowTarget {})
        .insert(FacingDirection {
            direction: FacingDirectionEnum::Right
        })
        .insert(Player {})
        .insert(Movement { speed: 100.0 });
}




