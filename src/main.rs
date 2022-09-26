mod camera_follow_system;
mod components;
mod coordinate;
mod map;
mod player_movement_system;
mod player_sprite_system;
mod rect;
mod draw_map_system;

use camera_follow_system::*;
pub use components::*;
pub use coordinate::*;
pub use draw_map_system::*;
pub use map::*;
use player_movement_system::*;
use player_sprite_system::*;
pub use rect::*;

use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::window::PresentMode;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;


fn main() {
    let map = Map::new_map_rooms_and_corridors(1);

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Bevy 2D Game".to_string(),
            width: 1600.0,
            height: 900.0,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(map)
        .add_plugins(DefaultPlugins)
        // Add the bevy_inspector_egui Plugin
        .add_plugin(WorldInspectorPlugin::new())
        // Add the bevy_rapier2d Plugin
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default()) // Comment this in and out to debug colliders.
        .add_startup_system(setup_physics)
        .add_startup_system(setup_camera)
        .add_startup_system(draw_map_system.label("draw_map_system"))
        .add_startup_system(setup_player.after("draw_map_system"))
        .add_startup_system(setup_music)
        .add_system(player_movement_system.label("player_movement_system"))
        .add_system(player_sprite_system.after("player_movement_system"))
        .add_system(camera_follow_system.after("player_movement_system"))
        .run();
}

// === Systems ===

// Startup Systems
fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Name::new("Camera"));
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, map: Res<Map>) {
    let sprite_size = 8.0;

    let (position_x, position_y) = map.rooms[0].center();
    let x = (position_x - (map.width / 2)) * TILE_SIZE;
    let y = (position_y - (map.height / 2)) * TILE_SIZE;

    commands.spawn_bundle(
        SpriteBundle {
            texture: asset_server.load("sprites/elf_m_idle_anim_f0.png"),
            transform: Transform::from_xyz(x as f32, y as f32, 1.0),
            ..Default::default()
        }
    )
        .insert(CameraFollowTarget {})
        .insert(FacingDirection {
            direction: FacingDirectionEnum::Right
        })
        .insert(Player {})
        .insert(Name::new("Player"))
        .insert(Movement { speed: 100.0 })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        // .insert(Collider::ball(sprite_size))
        .insert(Velocity::zero())
        .with_children(|parent| {
            parent.spawn_bundle(
                TransformBundle::from(Transform::from_xyz(0.0, -8.0, 0.0))
            )
                .insert(Collider::ball(sprite_size));
        });
}

fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    // Set Gravity to 0.0
    rapier_config.gravity = Vec2::ZERO;
}

fn setup_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("audio/Abstraction-LudumDare28-FourthTrack.ogg");
    audio.play(music);
}

// References
// 1. Rigid-bodies
// https://rapier.rs/docs/user_guides/bevy_plugin/rigid_bodies
// 2. Colliders
// https://rapier.rs/docs/user_guides/bevy_plugin/colliders
// 3. Transforms (Parenting, Childing)
// https://bevy-cheatbook.github.io/features/transforms.html