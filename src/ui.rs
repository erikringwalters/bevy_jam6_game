use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

use crate::level::*;

pub struct UIPlugin;

#[derive(Component)]
pub struct WinText;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_instructions, spawn_win_text))
            .add_systems(Update, display_win);
    }
}

#[hot]
fn spawn_instructions(mut commands: Commands) {
    commands.spawn((
        Text::new(
            "Left-Click to add dominoes.\n\nRight-Click or Middle Mouse to orbit.\n\nSpace to start physics.\n\nR or Z to undo.\n\nC to clear all.",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
    commands.spawn((
        Text::new(
            "Create a path to the goal!\n\nBe sure no markers are red\nor the physics won't start!",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            right: Val::Px(12.0),
            ..default()
        },
    ));
}

#[hot]
fn spawn_win_text(mut commands: Commands) {
    commands.spawn((
        WinText,
        Text::new("You Win!\n\n(N)ext Level"),
        TextFont {
            font_size: 100.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 1., 1.)),
        Visibility::Hidden,
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            top: Val::Percent(15.0),
            justify_self: JustifySelf::Center,
            ..default()
        },
    ));
}

#[hot]
fn display_win(level: Res<Level>, mut vis: Single<&mut Visibility, With<WinText>>) {
    **vis = if level.is_won {
        Visibility::Visible
    } else {
        Visibility::Hidden
    }
}
