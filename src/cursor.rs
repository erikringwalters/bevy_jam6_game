use crate::floor::Floor;
use bevy::prelude::*;
// use bevy_simple_subsecond_system::hot;

#[derive(Resource, Default)]
pub struct Cursor {
    pub position: Vec3,
}
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cursor::default())
            .add_systems(Update, draw_cursor);
    }
}

// #[hot]
fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    floor: Single<&GlobalTransform, With<Floor>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
    mut cursor: ResMut<Cursor>,
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
    cursor.position = ray.get_point(distance);

    // Draw a circle just above the floor plane at that position.
    gizmos.circle(
        Isometry3d::new(
            cursor.position + floor.up() * 0.01,
            Quat::from_rotation_arc(Vec3::Z, floor.up().as_vec3()),
        ),
        0.75,
        Color::WHITE,
    );
}
