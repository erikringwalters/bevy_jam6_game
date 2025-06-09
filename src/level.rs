use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

use crate::{
    curve::{self, ControlPoints, CurrentSimulation},
    domino::{self, Domino},
    environment,
};

pub const WALL_LENGTH_LONG: f32 = environment::FLOOR_LENGTH * 0.75;
pub const WALL_LENGTH_MEDIUM: f32 = environment::FLOOR_LENGTH * 0.66;
pub const WALL_LENGTH_SHORT: f32 = environment::FLOOR_LENGTH * 0.5;
pub const WALL_THICKNESS: f32 = domino::DOMINO_DISTANCE * 2.;
pub const WALL_HEIGHT: f32 = domino::DOMINO_SIZE.y;
pub const WALL_COLOR: Color = Color::srgba(0.7, 0.7, 0.7, 1.);

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
        // app.add_systems(Startup, initiate_level)
        app.add_systems(Update, handle_next_level);
    }
}

#[hot]
fn handle_next_level(
    mut commands: Commands,
    query: Query<Entity, With<Domino>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut level: ResMut<Level>,
    control_points: ResMut<ControlPoints>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sim: ResMut<CurrentSimulation>,
) {
    if level.is_won && keyboard.just_pressed(KeyCode::KeyN) {
        println!("N pressed");
        curve::despawn_entities(&mut commands, query);
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
    match level.value {
        0 => {}
        1 => {
            commands.spawn((
                Wall,
                Mesh3d(meshes.add(Cuboid::new(WALL_LENGTH_LONG, WALL_HEIGHT, WALL_THICKNESS))),
                MeshMaterial3d(materials.add(WALL_COLOR)),
            ));
        }
        2 => {}
        _ => {}
    }
}
