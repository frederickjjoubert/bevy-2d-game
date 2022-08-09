use bevy::prelude::*;
use super::{Player, Movement, FacingDirection, FacingDirectionEnum};

pub fn player_movement_system(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Movement, &mut FacingDirection, &mut Transform)>,
) {
    for (_player, movement, mut direction, mut transform) in query.iter_mut() {
        let mut x_delta = 0.0;
        let mut y_delta = 0.0;
        // Up
        if keyboard.pressed(KeyCode::W) {
            y_delta += movement.speed * time.delta_seconds();
        }
        // Down
        else if keyboard.pressed(KeyCode::S) {
            y_delta -= movement.speed * time.delta_seconds();
        }
        // Left
        if keyboard.pressed(KeyCode::A) {
            x_delta -= movement.speed * time.delta_seconds();
        }
        // Right
        else if keyboard.pressed(KeyCode::D) {
            x_delta += movement.speed * time.delta_seconds();
        }

        if x_delta != 0.0 {
            transform.translation.x += x_delta;
            if x_delta > 0.0 {
                direction.direction = FacingDirectionEnum::Right;
            } else {
                direction.direction = FacingDirectionEnum::Left;
            }
        }
        if y_delta != 0.0 {
            transform.translation.y += y_delta;
        }
    }
}