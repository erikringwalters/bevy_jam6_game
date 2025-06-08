use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

#[derive(Resource, Default)]
pub struct Level {
    pub value: u32,
    pub is_won: bool,
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initiate_level);
    }
}

#[hot]
fn initiate_level(mut commands: Commands, level: Res<Level>) {
    match level.value {
        0 => {}
        1 => {
            commands.spawn(());
        }
        _ => {}
    }
}
