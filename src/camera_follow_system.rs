use bevy::prelude::*;
use super::{Player, CameraFollowTarget};

pub fn camera_follow_system(
    follow_target_query: Query<&Transform, (With<CameraFollowTarget>, With<Player>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let follow_target_transform = follow_target_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = follow_target_transform.translation.x;
    camera_transform.translation.y = follow_target_transform.translation.y;
}

// I needed to add the Without<Player> to the second query to prevent the below error.

// Query<&mut bevy_transform::components::transform::Transform,
// bevy_ecs::query::filter::With<bevy_render::camera::camera::Camera>>
// in system bevy_2d_game::camera_follow_system::camera_follow_system
// accesses component(s) bevy_transform::components::transform::Transform
// in a way that conflicts with a previous system parameter.
// Consider using `Without<T>` to create disjoint Queries or merging conflicting Queries into a `ParamSet`.'