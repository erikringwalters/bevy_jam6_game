mod cursor;
mod curve;
mod domino;
mod environment;
mod floor;
mod goal;
mod pusher;
mod ui;

use crate::cursor::*;
use crate::curve::*;
use crate::environment::*;
use crate::goal::*;
use bevy_simple_subsecond_system::SimpleSubsecondPlugin;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use pusher::PusherPlugin;
use ui::UIPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(EnvironmentPlugin)
        .add_plugins(PusherPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(CurvePlugin)
        .add_plugins(UIPlugin)
        .add_plugins(GoalPlugin)
        .run();
}
