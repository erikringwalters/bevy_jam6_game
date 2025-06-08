use std::{f32::consts::FRAC_PI_2, ops::Range};

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

const CAMERA_DISTANCE: f32 = 30.;

#[derive(Component, Default)]
pub struct CameraPivot;

#[derive(Debug, Resource)]
struct CameraSettings {
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    // Clamp pitch to this range
    pub pitch_range: Range<f32>,
    pub yaw_speed: f32,
}
impl Default for CameraSettings {
    fn default() -> Self {
        // Limiting pitch stops some unexpected rotation past 90° up or down.
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            // These values are completely arbitrary, chosen because they seem to produce
            // "sensible" results for this example. Adjust as required.
            orbit_distance: CAMERA_DISTANCE * 1.5,
            pitch_speed: 0.01,
            pitch_range: -pitch_limit..0.,
            yaw_speed: 0.0075,
        }
    }
}
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings::default())
            .add_systems(Startup, setup_camera_rig)
            .add_systems(Update, handle_camera_movement);
    }
}

fn setup_camera_rig(mut commands: Commands) {
    commands.spawn((
        Name::new("CameraRig"),
        Transform::from_translation(Vec3::ZERO),
        Children::spawn(Spawn((
            Name::new("Camera"),
            Camera3d::default(),
            Transform::from_xyz(-CAMERA_DISTANCE, CAMERA_DISTANCE * 0.5, -CAMERA_DISTANCE)
                .looking_at(
                    Vec3 {
                        x: 0.,
                        y: -1.,
                        z: 0.,
                    },
                    Dir3::Y,
                ),
        ))),
    ));
}

fn handle_camera_movement(
    mut camera_pivot: Single<&mut Transform, With<Camera>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera_settings: Res<CameraSettings>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    if mouse_button_input.pressed(MouseButton::Right) {
        let delta = mouse_motion.delta;
        // Mouse motion is one of the few inputs that should not be multiplied by delta time,
        // as we are already receiving the full movement since the last frame was rendered. Multiplying
        // by delta time here would make the movement slower that it should be.
        let delta_pitch = delta.y * camera_settings.pitch_speed;
        let delta_yaw = delta.x * camera_settings.yaw_speed;

        // Obtain the existing pitch, yaw, and roll values from the transform.
        let (yaw, pitch, roll) = camera_pivot.rotation.to_euler(EulerRot::YXZ);

        // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
        let pitch = (pitch - delta_pitch).clamp(
            camera_settings.pitch_range.start,
            camera_settings.pitch_range.end,
        );
        let yaw = yaw - delta_yaw;
        camera_pivot.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

        // Adjust the translation to maintain the correct orientation toward the orbit target.
        // In our example it's a static target, but this could easily be customized.
        let target = Vec3::ZERO;
        camera_pivot.translation = target - camera_pivot.forward() * camera_settings.orbit_distance;
    }
}
