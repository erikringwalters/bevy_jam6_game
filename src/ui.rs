use bevy::prelude::*;

use crate::level::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, display_instructions)
            .add_systems(Update, display_win);
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

fn display_win(mut commands: Commands, level: Res<Level>) {
    if level.is_won {
        commands.spawn((
            Text::new("You Win!"),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(15.0),
                left: Val::Percent(50.0),
                ..default()
            },
        ));
    }
}
