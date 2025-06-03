mod cursor;
mod curve;
mod domino;
mod environment;
mod floor;

use crate::cursor::*;
use crate::curve::*;
use crate::environment::*;
use bevy_simple_subsecond_system::SimpleSubsecondPlugin;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(EnvironmentPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(CurvePlugin)
        .run();
}
