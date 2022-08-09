use bevy::prelude::*;
use super::{Player, FacingDirection, FacingDirectionEnum};

pub fn player_sprite_system(
    mut query: Query<(&mut Sprite, &FacingDirection), With<Player>>
) {
    println!("Is this running?");
    let (mut sprite, facing_direction) = query.single_mut();
    let direction = &facing_direction.direction;
    match direction {
        FacingDirectionEnum::Left => {
            sprite.flip_x = true;
        }
        FacingDirectionEnum::Right => {
            sprite.flip_x = false;
        }
        _ => {}
    }
}