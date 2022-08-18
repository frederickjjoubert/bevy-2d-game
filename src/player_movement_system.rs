use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{
    Player,
    Movement,
    FacingDirection,
    FacingDirectionEnum,
};

pub fn player_movement_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Player,
        &mut Movement,
        &mut FacingDirection,
        &mut Velocity
    )>,
) {
    for (
        _player,
        movement,
        mut direction,
        mut velocity,
    ) in query.iter_mut() {
        let mut velocity_delta = Vec2::new(0.0, 0.0);
        let mut x_delta = 0.0;
        let mut y_delta = 0.0;
        // Up
        if keyboard.pressed(KeyCode::W) {
            y_delta += 1.0;
        }
        // Down
        else if keyboard.pressed(KeyCode::S) {
            y_delta -= 1.0;
        }
        // Left
        if keyboard.pressed(KeyCode::A) {
            x_delta -= 1.0;
        }
        // Right
        else if keyboard.pressed(KeyCode::D) {
            x_delta += 1.0;
        }

        velocity_delta.x = x_delta;
        velocity_delta.y = y_delta;

        if velocity_delta != Vec2::ZERO {
            // Normalize
            velocity_delta /= velocity_delta.length();
            velocity_delta *= movement.speed;
        }

        // Update facing direction
        if velocity_delta.x != 0.0 {
            if velocity_delta.x > 0.0 {
                direction.direction = FacingDirectionEnum::Right;
            } else {
                direction.direction = FacingDirectionEnum::Left;
            }
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        velocity.linvel = velocity_delta;
    }
}

