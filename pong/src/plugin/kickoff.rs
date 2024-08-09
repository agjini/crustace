use bevy::input::gamepad::GamepadButtonInput;
use bevy::input::ButtonInput;
use bevy::prelude::{
    default, Commands, EventReader, JustifyText, KeyCode, NextState, PositionType, Res, ResMut,
    StateScoped, Style, TextBundle, TextStyle, Val,
};

use crate::plugin::state::AppState;

pub fn display_action(mut commands: Commands) {
    commands.spawn((
        StateScoped(AppState::KickOff),
        TextBundle::from_section(
            "press space to start",
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
    ));
}

pub fn kickoff(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut gamepad_button: EventReader<GamepadButtonInput>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
    for _ in gamepad_button.read() {
        next_state.set(AppState::InGame);
    }
}

pub fn to_kickoff(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::KickOff);
}
