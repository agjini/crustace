use bevy::prelude::{
    default, Commands, Component, JustifyText, PositionType, Query, Style, Text, TextBundle,
    TextStyle, Val,
};

use crate::plugin::paddle::{Left, Right};

#[derive(Component)]
pub struct Score(pub u8);

pub fn display_score(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Percent(45.0),
            ..default()
        }),
        Score(0),
        Left,
    ));

    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Percent(55.0),
            ..default()
        }),
        Score(0),
        Right,
    ));
}

pub fn update_score(mut query: Query<(&mut Text, &Score)>) {
    for (mut text, score) in query.iter_mut() {
        text.sections[0].value = format!("{}", score.0);
    }
}
