use crate::cursor::Cursor;
use crate::domino::{self, DOMINO_DISTANCE, Domino, DominoMarker};
use crate::environment;
use crate::pusher::Pusher;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_simple_subsecond_system::hot;

#[derive(Default, PartialEq, Debug)]
pub enum SimulationState {
    #[default]
    Draw,
    Physics,
}

#[derive(Resource, Default, Debug)]
pub struct CurrentSimulation {
    state: SimulationState,
}

#[derive(Component, Resource, Clone, Default)]
struct Curve(Option<CubicCurve<Vec3>>);

#[derive(Resource, Clone, Default)]
pub struct ControlPoints {
    points: Vec<Vec3>,
}

pub struct CurvePlugin;

impl Plugin for CurvePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Curve::default())
            .insert_resource(CurrentSimulation::default())
            .insert_resource(ControlPoints::default())
            .add_systems(PostStartup, setup_curve)
            .add_systems(FixedUpdate, animate_bump)
            .add_systems(
                Update,
                (
                    handle_click,
                    handle_undo,
                    handle_start_sim,
                    update_curve,
                    // draw_curve,
                ),
            );
    }
}

#[hot]
fn setup_curve(
    mut control_points: ResMut<ControlPoints>,
    transform: Single<&Transform, With<Pusher>>,
) {
    control_points.points.clear();
    control_points.points.push(transform.translation);
}

#[hot]
fn update_curve(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    control_points: ResMut<ControlPoints>,
    mut curve: ResMut<Curve>,
    query: Query<Entity, With<DominoMarker>>,
) {
    if !control_points.is_changed() {
        return;
    }

    *curve = form_curve(&control_points);
    despawn_entities(&mut commands, query);
    spawn_markers(&mut commands, meshes, materials, curve.into());
}

#[hot]
fn draw_curve(curve: Res<Curve>, mut gizmos: Gizmos) {
    let Some(ref curve) = curve.0 else {
        return;
    };
    // Scale resolution with curve length so it doesn't degrade as the length increases.
    let resolution = 100 * curve.segments().len();
    // println!("{:?}\n", curve.segments());
    gizmos.linestrip(
        curve.iter_positions(resolution).map(|pt| pt),
        Color::srgba(1.0, 1.0, 1.0, 1.0),
    );
}

#[hot]
fn form_curve(control_points: &ControlPoints) -> Curve {
    let points: Vec<Vec3> = control_points.points.iter().copied().collect();
    let spline = CubicCardinalSpline::new_catmull_rom(points);
    Curve(spline.to_curve().ok())
}

#[hot]
fn handle_click(
    mut commands: Commands,
    mut sim: ResMut<CurrentSimulation>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    mut control_points: ResMut<ControlPoints>,
    query: Query<Entity, With<Domino>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        sim.state = SimulationState::Draw;
        despawn_entities(&mut commands, query);
        let mut pos = cursor.position;
        pos.y = 1.7;
        if control_points.points.len() > 0 && control_points.points[0] == Vec3::ZERO {
            control_points.points[0] = pos;
        } else {
            control_points.points.push(pos);
        }
    }
}

#[hot]
fn handle_undo(
    mut commands: Commands,
    mut sim: ResMut<CurrentSimulation>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut control_points: ResMut<ControlPoints>,
    query: Query<Entity, With<Domino>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) || keyboard.just_pressed(KeyCode::KeyZ) {
        despawn_entities(&mut commands, query);
        control_points.points.pop();
        if control_points.points.is_empty() {
            control_points.points.push(environment::PUSHER_START_POS);
        }
        sim.state = SimulationState::Draw;
    }
    if keyboard.just_pressed(KeyCode::KeyC) {
        despawn_entities(&mut commands, query);
        control_points.points.clear();
        control_points.points.push(environment::PUSHER_START_POS);
        sim.state = SimulationState::Draw;
    }
}

#[hot]
fn handle_start_sim(
    mut commands: Commands,
    mut sim: ResMut<CurrentSimulation>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &Transform), With<DominoMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for (marker, transform) in query.iter_mut() {
            let pos = transform.translation;
            let rot = transform.rotation;
            commands.entity(marker).despawn();
            commands.spawn((
                Domino,
                RigidBody::Dynamic,
                Collider::cuboid(
                    domino::DOMINO_HALF_SIZE.x,
                    domino::DOMINO_HALF_SIZE.y,
                    domino::DOMINO_HALF_SIZE.z,
                ),
                Mesh3d(meshes.add(Cuboid::from_size(domino::DOMINO_SIZE))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Transform::from_translation(pos).with_rotation(rot),
            ));
        }
        sim.state = SimulationState::Physics;
    }
}

#[hot]
fn animate_bump(
    control_points: Res<ControlPoints>,
    sim: ResMut<CurrentSimulation>,
    mut pusher_transform: Single<&mut Transform, With<Pusher>>,
    time: Res<Time>,
) {
    // println!("{:?}", sim.state);
    if sim.state == SimulationState::Physics {
        let distance = DOMINO_DISTANCE * 1.75;
        let toward = if control_points.points.len() > 1 {
            control_points.points[1]
        } else {
            pusher_transform.translation
        };

        if pusher_transform
            .translation
            .distance(control_points.points[0])
            < distance
        {
            pusher_transform
                .translation
                .smooth_nudge(&toward, 0.04, time.delta_secs());
        }
    }
}

#[hot]
fn spawn_markers(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    curve: Res<Curve>,
) {
    let Some(ref curve) = curve.0 else {
        return;
    };

    let spacing = domino::DOMINO_DISTANCE;
    let resolution = 1000;
    let mut last_pos = Vec3::ZERO;
    let mut pos: Vec3;
    let mut dist_accum = 0.0;
    let mut next_dist = spacing;

    for segment in curve.segments() {
        if last_pos == Vec3::ZERO {
            last_pos = segment.position(0.0);
        }
        for i in 1..=resolution {
            let t = i as f32 / resolution as f32;
            pos = segment.position(t);
            let step = pos.distance(last_pos);
            dist_accum += step;
            if step > spacing {}

            if dist_accum >= next_dist {
                commands.spawn((
                    Name::new("Domino Marker"),
                    DominoMarker,
                    Collider::cuboid(
                        domino::DOMINO_HALF_SIZE.x,
                        domino::DOMINO_HALF_SIZE.y,
                        domino::DOMINO_HALF_SIZE.z,
                    ),
                    Sensor,
                    Mesh3d(meshes.add(Cuboid::from_size(domino::DOMINO_SIZE))),
                    MeshMaterial3d(materials.add(Color::srgba(0.2, 0.8, 0.2, 0.9))),
                    Transform::from_translation(pos).looking_at(last_pos, Dir3::Y),
                ));
                next_dist += spacing;
            }
            last_pos = pos;
        }
    }
}

#[hot]
fn despawn_entities<T: Component>(commands: &mut Commands, mut query: Query<Entity, With<T>>) {
    for marker in query.iter_mut() {
        commands.entity(marker).despawn();
    }
}
