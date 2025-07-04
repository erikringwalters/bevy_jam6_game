mod camera;
mod cursor;
mod curve;
mod domino;
mod environment;
mod floor;
mod goal;
mod level;
mod pusher;
mod ui;

use crate::camera::*;
use crate::cursor::*;
use crate::curve::*;
use crate::environment::*;
use crate::goal::*;
// use bevy_simple_subsecond_system::SimpleSubsecondPlugin;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use domino::DominoPlugin;
use level::LevelPlugin;
use pusher::PusherPlugin;
use ui::UIPlugin;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#canvas".into()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(EnvironmentPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DominoPlugin)
        .add_plugins(PusherPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(CurvePlugin)
        .add_plugins(UIPlugin)
        .add_plugins(GoalPlugin)
        .add_plugins(LevelPlugin)
        .run();
}
