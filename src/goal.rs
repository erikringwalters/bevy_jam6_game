use crate::{
    curve::{CurrentSimulation, SimulationState},
    environment,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    ActiveCollisionTypes, ActiveEvents, Collider, CollidingEntities, Sensor,
};

pub const GOAL_WIDTH: f32 = environment::FLOOR_LENGTH * 0.125;
pub const GOAL_HALF_WIDTH: f32 = GOAL_WIDTH * 0.5;
// pub const GOAL_SIZE: Vec3 = vec3(GOAL_WIDTH, GOAL_WIDTH, GOAL_WIDTH);
pub const GOAL_START_POS: Vec3 = vec3(
    -environment::FLOOR_HALF_SIZE.x * environment::PUSHER_OFFSET,
    environment::FLOOR_HALF_SIZE.y + GOAL_HALF_WIDTH + 0.1,
    environment::FLOOR_HALF_SIZE.z * environment::PUSHER_OFFSET,
);
const WIN_COLOR: Color = Color::srgba(0., 0.75, 0.75, 0.3);
const DEFAULT_COLOR: Color = Color::srgba(0.7, 0.7, 0.2, 0.3);

#[derive(Component)]
pub struct Goal;

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_goal)
            .add_systems(Update, detect_dominos);
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
        CollidingEntities::default(),
        ActiveCollisionTypes::all(),
        ActiveEvents::COLLISION_EVENTS,
        Mesh3d(meshes.add(Cylinder::new(GOAL_HALF_WIDTH, GOAL_WIDTH))),
        MeshMaterial3d(materials.add(DEFAULT_COLOR)),
        Transform::from_translation(GOAL_START_POS),
    ));
}

fn detect_dominos(
    query: Query<
        (
            &CollidingEntities,
            // &DominoSensor,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        With<Goal>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sim: Res<CurrentSimulation>,
) {
    for (colliding, material_handle) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.base_color = if colliding.is_empty() || sim.state != SimulationState::Physics {
                DEFAULT_COLOR
            } else {
                WIN_COLOR
            }
        }
    }
}
