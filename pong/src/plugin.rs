use bevy::app::{Startup, Update};
use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, States};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;

use crate::plugin::ball::add_ball;
use crate::plugin::kickoff::{clean_action, display_action, kickoff, to_kickoff};
use crate::plugin::paddle::{add_paddle, move_paddle};
use crate::plugin::playground::{add_playground, display_events};
use crate::plugin::score::{display_score, update_score};

mod ball;
mod kickoff;
mod paddle;
mod playground;
mod score;

pub struct PongPlugin;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    KickOff,
    InGame,
    Goal,
}

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, add_playground)
            .add_systems(Startup, add_paddle)
            .add_systems(Startup, display_score)
            .add_systems(OnEnter(AppState::KickOff), display_action)
            .add_systems(OnExit(AppState::KickOff), clean_action)
            .add_systems(Update, kickoff.run_if(in_state(AppState::KickOff)))
            .add_systems(OnEnter(AppState::InGame), add_ball)
            .add_systems(Update, move_paddle)
            .add_systems(Update, display_events)
            .add_systems(Update, update_score)
            .add_systems(OnEnter(AppState::Goal), to_kickoff);
    }
}
