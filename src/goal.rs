use crate::environment;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, Sensor};

pub const GOAL_WIDTH: f32 = environment::FLOOR_LENGTH * 0.125;
pub const GOAL_HALF_WIDTH: f32 = GOAL_WIDTH * 0.5;
// pub const GOAL_SIZE: Vec3 = vec3(GOAL_WIDTH, GOAL_WIDTH, GOAL_WIDTH);
pub const GOAL_START_POS: Vec3 = vec3(
    -environment::FLOOR_HALF_SIZE.x * environment::PUSHER_OFFSET,
    environment::FLOOR_HALF_SIZE.x * 0.125,
    environment::FLOOR_HALF_SIZE.z * environment::PUSHER_OFFSET,
);
#[derive(Component)]
pub struct Goal;

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_goal);
    }
}

fn setup_goal(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Goal"),
        Goal,
        Collider::cylinder(GOAL_HALF_WIDTH, GOAL_HALF_WIDTH),
        Sensor,
        Mesh3d(meshes.add(Cylinder::new(GOAL_HALF_WIDTH, GOAL_WIDTH))),
        MeshMaterial3d(materials.add(Color::srgba(0., 0.75, 0.75, 0.3))),
        Transform::from_translation(GOAL_START_POS),
    ));
}
