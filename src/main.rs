mod components;

use components::*;

use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_movement_system)
        .run();
}

// === Systems ===

// Startup Systems
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(
        SpriteBundle {
            texture: asset_server.load("sprites/elf_m_idle_anim_f0.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        }
    )
        .insert(FacingDirection {
            direction: FacingDirectionEnum::Up
        })
        .insert(Player {})
        .insert(Movement { speed: 100.0 });
}

// Systems
/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
// fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
//     for (mut direction, mut transform) in &mut sprite_position {
//         match *direction {
//             Direction::Up => {
//                 transform.translation.y += 150. * time.delta_seconds();
//             }
//             Direction::Down => {
//                 transform.translation.y -= 150. * time.delta_seconds();
//             }
//             _ => {}
//         }
//
//         if transform.translation.y > 200.0 {
//             *direction = Direction::Down;
//         } else if transform.translation.y < -200.0 {
//             *direction = Direction::Up;
//         }
//     }
// }

fn player_movement_system(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Movement, &mut FacingDirection, &mut Transform)>,
) {
    for (_player, movement, mut direction, mut transform) in query.iter_mut() {
        // Up
        if keyboard.pressed(KeyCode::W) {
            transform.translation.y += movement.speed * time.delta_seconds();
            direction.direction = FacingDirectionEnum::Up;
        }
        // Down
        if keyboard.pressed(KeyCode::S) {
            transform.translation.y -= movement.speed * time.delta_seconds();
            direction.direction = FacingDirectionEnum::Down;
        }
        // Left
        if keyboard.pressed(KeyCode::A) {
            transform.translation.x -= movement.speed * time.delta_seconds();
            direction.direction = FacingDirectionEnum::Left;
        }
        // Right
        if keyboard.pressed(KeyCode::D) {
            transform.translation.x += movement.speed * time.delta_seconds();
            direction.direction = FacingDirectionEnum::Right;
        }
    }
}