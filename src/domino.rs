use bevy::prelude::*;

pub const DOMINO_SIZE: Vec3 = Vec3::new(1., 2., 0.2);
pub const DOMINO_HALF_SIZE: Vec3 = Vec3::new(
    DOMINO_SIZE.x * 0.5,
    DOMINO_SIZE.y * 0.5,
    DOMINO_SIZE.z * 0.5,
);
pub const DOMINO_DISTANCE: f32 = DOMINO_SIZE.y * 0.6;

#[derive(Component)]
pub struct Domino;

#[derive(Component)]
pub struct DominoMarker;
