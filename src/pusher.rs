use crate::environment;
use bevy::prelude::*;

pub const RADIUS: f32 = 0.25;
pub const PUSHER_COLOR: Color = Color::srgb(0., 0.75, 0.75);

#[derive(Component)]
pub struct Pusher;

pub struct PusherPlugin;

impl Plugin for PusherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_reset_pusher);
    }
}
fn handle_reset_pusher(
    mut transform: Single<&mut Transform, With<Pusher>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::KeyR)
        || keyboard.just_pressed(KeyCode::KeyZ)
        || keyboard.just_pressed(KeyCode::KeyC)
    {
        transform.translation = environment::PUSHER_START_POS;
    }
}
