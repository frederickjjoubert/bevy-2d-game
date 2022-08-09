use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
}

pub enum FacingDirectionEnum {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct FacingDirection {
    pub direction: FacingDirectionEnum,
}