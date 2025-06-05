use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, display_instructions);
    }
}

fn display_instructions(mut commands: Commands) {
    commands.spawn((
        Text::new(
            "Click to add dominoes.\n\nSpace to start physics.\n\nR or Z to undo.\n\nC to clear all dominoes.",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}
