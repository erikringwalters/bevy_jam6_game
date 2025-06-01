use bevy::prelude::*;

mod environment;

use crate::environment::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EnvironmentPlugin)
        .add_systems(Update, draw_cursor)
        .run();
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    floor: Single<&GlobalTransform, With<Floor>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let Ok(windows) = windows.single() else {
        return;
    };

    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the floor plane.
    let Some(distance) = ray.intersect_plane(
        floor.translation() + vec3(0.0, 1.0, 0.0),
        InfinitePlane3d::new(floor.up()),
    ) else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the floor plane at that position.
    gizmos.circle(
        Isometry3d::new(
            point + floor.up() * 0.01,
            Quat::from_rotation_arc(Vec3::Z, floor.up().as_vec3()),
        ),
        0.2,
        Color::WHITE,
    );
}
