use crate::plugin::state::AppState;
use bevy::input::gamepad::GamepadButtonStateChangedEvent;
use bevy::input::ButtonInput;
use bevy::prelude::{
    default, Commands, EventReader, JustifyText, KeyCode, NextState, Node, PositionType, Res,
    ResMut, StateScoped, Text, TextFont, TextLayout, Val,
};

pub fn display_action(mut commands: Commands) {
    commands.spawn((
        StateScoped(AppState::KickOff),
        Text::new("Press space to start"),
        TextFont::default().with_font_size(50.0),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
    ));
}

pub fn kickoff(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut gamepad_button: EventReader<GamepadButtonStateChangedEvent>,
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
