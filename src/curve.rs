use crate::cursor::Cursor;
use bevy::{color::palettes::css, prelude::*};
use bevy_simple_subsecond_system::hot;

#[derive(Resource, Clone, Default)]
struct Curve(Option<CubicCurve<Vec3>>);

#[derive(Resource, Clone)]
struct ControlPoints {
    points: Vec<Vec3>,
}

pub struct CurvePlugin;

impl Plugin for CurvePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Curve::default())
            .add_systems(Startup, setup_curve)
            .add_systems(Update, (update_curve, draw_curve, handle_click));
    }
}

fn setup_curve(mut commands: Commands) {
    // Starting data for [`ControlPoints`]:
    let default_points = vec![vec3(0., 0., 0.)];

    let default_control_data = ControlPoints {
        points: default_points.into_iter().collect(),
    };

    let curve = form_curve(&default_control_data);
    commands.insert_resource(curve);
    commands.insert_resource(default_control_data)
}

#[hot]
fn update_curve(control_points: Res<ControlPoints>, mut curve: ResMut<Curve>) {
    if !control_points.is_changed() {
        return;
    }

    *curve = form_curve(&control_points);
}

/// This system uses gizmos to draw the current [`Curve`] by breaking it up into a large number
/// of line segments.
fn draw_curve(curve: Res<Curve>, mut gizmos: Gizmos) {
    let Some(ref curve) = curve.0 else {
        return;
    };
    // Scale resolution with curve length so it doesn't degrade as the length increases.
    let resolution = 100 * curve.segments().len();
    println!("{:?}", curve.segments());
    gizmos.linestrip(
        curve.iter_positions(resolution).map(|pt| pt),
        Color::srgb(1.0, 1.0, 1.0),
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
        commands.spawn((
            Mesh3d(meshes.add(Sphere { radius: 0.4 })),
            MeshMaterial3d(materials.add(Color::from(css::DARK_SEA_GREEN))),
            Transform::from_translation(cursor.position),
        ));
        control_points.points.push(cursor.position);
    }
}
