use bevy::prelude::*;
use bevy_rapier3d::prelude::CollidingEntities;

use crate::environment;

pub const DOMINO_SIZE: Vec3 = Vec3::new(1., 2., 0.2);
pub const DOMINO_HALF_SIZE: Vec3 = Vec3::new(
    DOMINO_SIZE.x * 0.5,
    DOMINO_SIZE.y * 0.5,
    DOMINO_SIZE.z * 0.5,
);
pub const DOMINO_DISTANCE: f32 = DOMINO_SIZE.y * 0.6;
pub const DOMINO_Y_OFFSET: f32 = 0.025;
pub const DOMINO_Y_POS: f32 = environment::FLOOR_HALF_SIZE.y + DOMINO_HALF_SIZE.y + DOMINO_Y_OFFSET;

pub const VALID_COLOR: Color = Color::srgba(0.2, 0.8, 0.2, 0.9);
pub const INVALID_COLOR: Color = Color::srgba(0.5, 0.1, 0.1, 0.9);

#[derive(Component)]
pub struct Domino;

#[derive(Component)]
pub struct DominoMarker;

#[derive(Component)]
pub struct DominoSensor;

#[derive(Resource, Debug, Default)]
pub struct IsAllValid {
    pub value: bool,
}

pub struct DominoPlugin;

impl Plugin for DominoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IsAllValid::default())
            .add_systems(Update, detect_valid_markers);
    }
}

fn detect_valid_markers(
    query: Query<(&CollidingEntities, &mut MeshMaterial3d<StandardMaterial>), With<DominoMarker>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut is_all_valid: ResMut<IsAllValid>,
) {
    is_all_valid.value = true;
    for (colliding, material_handle) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.base_color = if colliding.is_empty() {
                VALID_COLOR
            } else {
                is_all_valid.value = false;
                INVALID_COLOR
            };
        }
    }
}
