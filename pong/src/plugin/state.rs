use bevy::prelude::States;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    KickOff,
    InGame,
    Goal,
}
