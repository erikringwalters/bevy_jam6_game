use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Win {
    pub status: bool,
}
