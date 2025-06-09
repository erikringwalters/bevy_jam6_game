use std::f32::consts::PI;

use bevy::{prelude::*, transform};
use bevy_rapier3d::prelude::Collider;
use bevy_simple_subsecond_system::hot;

use crate::{
    curve::{self, ControlPoints, CurrentSimulation},
    domino::{self, Domino},
    environment::{self, FLOOR_HALF_SIZE},
};

pub const WALL_LENGTH_LONG: f32 = environment::FLOOR_LENGTH * 0.75;
pub const WALL_LENGTH_MEDIUM: f32 = environment::FLOOR_LENGTH * 0.66;
pub const WALL_LENGTH_SHORT: f32 = environment::FLOOR_LENGTH * 0.5;
pub const WALL_THICKNESS: f32 = domino::DOMINO_DISTANCE;
pub const WALL_HEIGHT: f32 = domino::DOMINO_SIZE.y * 0.9;
pub const WALL_COLOR: Color = Color::srgb(0.3, 0.2, 0.1);
pub const ROTATION_90Y: Quat = Quat::from_array([0., 0.7071068, 0., 0.7071068]);

#[derive(Resource, Default)]
pub struct Level {
    pub value: u32,
    pub is_won: bool,
}

#[derive(Component)]
pub struct Wall;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initiate_level)
            .add_systems(Update, handle_next_level);
    }
}

#[hot]
fn handle_next_level(
    mut commands: Commands,
    marker_query: Query<Entity, With<Domino>>,
    wall_query: Query<Entity, With<Wall>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut level: ResMut<Level>,
    control_points: ResMut<ControlPoints>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    sim: ResMut<CurrentSimulation>,
) {
    if level.is_won && keyboard.just_pressed(KeyCode::KeyN) {
        curve::despawn_entities(&mut commands, marker_query);
        curve::despawn_entities(&mut commands, wall_query);
        curve::clear_curve(control_points, sim);
        level.is_won = false;
        level.value += 1;
        initiate_level(commands, meshes, materials, level.into());
    }
}

#[hot]
fn initiate_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    level: Res<Level>,
) {
    let y_pos = environment::FLOOR_HALF_SIZE.y + WALL_HEIGHT * 0.5;
    match level.value {
        0 => {}
        1 => {
            commands.spawn((
                Wall,
                Mesh3d(meshes.add(Cuboid::new(WALL_LENGTH_LONG, WALL_HEIGHT, WALL_THICKNESS))),
                Collider::cuboid(
                    WALL_LENGTH_LONG * 0.5,
                    WALL_HEIGHT * 0.5,
                    WALL_THICKNESS * 0.5,
                ),
                MeshMaterial3d(materials.add(WALL_COLOR)),
                Transform::from_xyz(
                    FLOOR_HALF_SIZE.x * 0.5 - WALL_THICKNESS * 0.5,
                    y_pos,
                    -FLOOR_HALF_SIZE.z + WALL_LENGTH_LONG * 0.5,
                )
                .with_rotation(ROTATION_90Y),
            ));
            commands.spawn((
                Wall,
                Mesh3d(meshes.add(Cuboid::new(WALL_LENGTH_LONG, WALL_HEIGHT, WALL_THICKNESS))),
                Collider::cuboid(
                    WALL_LENGTH_LONG * 0.5,
                    WALL_HEIGHT * 0.5,
                    WALL_THICKNESS * 0.5,
                ),
                MeshMaterial3d(materials.add(WALL_COLOR)),
                Transform::from_xyz(
                    -FLOOR_HALF_SIZE.x + WALL_LENGTH_LONG * 0.5,
                    y_pos,
                    FLOOR_HALF_SIZE.z * 0.5 - WALL_THICKNESS * 0.5,
                ),
            ));
        }
        2 => {
            commands.spawn((
                Wall,
                Mesh3d(meshes.add(Cuboid::new(WALL_LENGTH_LONG, WALL_HEIGHT, WALL_THICKNESS))),
                Collider::cuboid(
                    WALL_LENGTH_LONG * 0.5,
                    WALL_HEIGHT * 0.5,
                    WALL_THICKNESS * 0.5,
                ),
                MeshMaterial3d(materials.add(WALL_COLOR)),
                Transform::from_xyz(
                    FLOOR_HALF_SIZE.x - WALL_LENGTH_LONG * 0.5,
                    y_pos,
                    -FLOOR_HALF_SIZE.z * 0.33,
                ),
            ));
            commands.spawn((
                Wall,
                Mesh3d(meshes.add(Cuboid::new(WALL_LENGTH_LONG, WALL_HEIGHT, WALL_THICKNESS))),
                Collider::cuboid(
                    WALL_LENGTH_LONG * 0.5,
                    WALL_HEIGHT * 0.5,
                    WALL_THICKNESS * 0.5,
                ),
                MeshMaterial3d(materials.add(WALL_COLOR)),
                Transform::from_xyz(
                    -FLOOR_HALF_SIZE.x + WALL_LENGTH_LONG * 0.5,
                    y_pos,
                    FLOOR_HALF_SIZE.z * 0.33,
                ),
            ));
        }
        _ => {}
    }
}
