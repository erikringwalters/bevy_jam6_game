use crate::cursor::Cursor;
use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

#[derive(Component, Resource, Clone, Default)]
struct Curve(Option<CubicCurve<Vec3>>);

#[derive(Resource, Clone)]
struct ControlPoints {
    points: Vec<Vec3>,
}

#[derive(Component)]
struct DominoMarker;

pub struct CurvePlugin;

impl Plugin for CurvePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Curve::default())
            .add_systems(Startup, setup_curve)
            .add_systems(
                Update,
                (handle_click, handle_undo, update_curve, draw_curve).chain(),
            );
    }
}

fn setup_curve(mut commands: Commands) {
    // Starting data for [`ControlPoints`]:
    let default_points = vec![Vec3::ZERO];

    let default_control_data = ControlPoints {
        points: default_points.into_iter().collect(),
    };

    let curve = form_curve(&default_control_data);
    commands.insert_resource(curve);
    commands.insert_resource(default_control_data)
}

#[hot]
fn update_curve(
    commands1: Commands,
    commands2: Commands,
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
    despawn_markers(commands1, query);
    spawn_markers(commands2, meshes, materials, curve.into());
}

/// This system uses gizmos to draw the current [`Curve`] by breaking it up into a large number
/// of line segments.
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    mut control_points: ResMut<ControlPoints>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if control_points.points.len() > 0 && control_points.points[0] == Vec3::ZERO {
            control_points.points[0] = cursor.position;
            commands.spawn((
                Mesh3d(meshes.add(Sphere { radius: 0.25 })),
                MeshMaterial3d(materials.add(Color::srgba(0.75, 0., 0.75, 1.0))),
                Transform::from_translation(cursor.position),
            ));
        } else {
            control_points.points.push(cursor.position);
        }
    }
}

#[hot]
fn handle_undo(keyboard: Res<ButtonInput<KeyCode>>, mut control_points: ResMut<ControlPoints>) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        control_points.points.pop();
    }
}

#[hot]
fn spawn_markers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    curve: Res<Curve>,
) {
    let Some(ref curve) = curve.0 else {
        return;
    };

    let spacing = 1.0;
    let resolution = 1000;
    let mut last_pos = Vec3::ZERO;
    let mut pos: Vec3;

    for segment in curve.segments() {
        if last_pos == Vec3::ZERO {
            last_pos = segment.position(0.0);
        }
        let mut dist_accum = 0.0;
        let mut next_dist = spacing;

        for i in 1..=resolution {
            let t = i as f32 / resolution as f32;
            pos = segment.position(t);
            let step = pos.distance(last_pos);
            dist_accum += step;

            if dist_accum >= next_dist {
                commands.spawn((
                    Name::new("Domino Marker"),
                    Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 0.2))),
                    MeshMaterial3d(materials.add(Color::srgba(0.9, 0.9, 0.9, 1.0))),
                    DominoMarker,
                    Transform::from_translation(pos).looking_at(last_pos, Dir3::Y),
                ));
                next_dist += spacing;
            }

            last_pos = pos;
        }
    }
}

fn despawn_markers(mut commands: Commands, mut query: Query<Entity, With<DominoMarker>>) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
