use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Level {
    pub value: u32,
    pub is_won: bool,
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {}
}
