use bevy::prelude::*;

pub const DOMINO_SIZE: Vec3 = Vec3::new(1., 2., 0.2);
pub const DOMINO_HALF_SIZE: Vec3 = Vec3::new(
    DOMINO_SIZE.x * 0.5,
    DOMINO_SIZE.y * 0.5,
    DOMINO_SIZE.z * 0.5,
);
pub const DOMINO_DISTANCE: f32 = DOMINO_SIZE.y * 0.4;

#[derive(Component)]
pub struct Domino;

#[derive(Component)]
pub struct DominoMarker;

// pub struct DominoMarkerPlugin;

// impl Plugin for DominoMarkerPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update, check_marker_validity);
//     }
// }
// fn check_marker_validity(
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut material_handles: Query<&MeshMaterial3d<StandardMaterial>, With<DominoMarker>>,
// ) {
//     for mut marker_material in query.iter() {
//         marker_material.add(Color::srgba(0.1, 0.75, 0.1, 0.8))
//     }
// }
