use bevy::prelude::Component;
use super::{Coordinate};

#[derive(Component)]
pub struct CameraFollowTarget {}

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

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Tile {
    pub coordinate: Coordinate,
}






