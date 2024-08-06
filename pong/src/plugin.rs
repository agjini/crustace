use avian3d::math::Vector;
use avian3d::prelude::{Gravity, PhysicsDebugPlugin};
use avian3d::PhysicsPlugins;
use bevy::app::{Startup, Update};
use bevy::prelude::{
    in_state, App, AppExtStates, IntoSystemConfigs, OnEnter, OnExit, Plugin, States,
};

use crate::plugin::kickoff::{clean_action, display_action, kickoff, to_kickoff};
use crate::plugin::paddle::{add_paddle, move_paddle};
use crate::plugin::playground::add_playground;
use crate::plugin::puck::add_puck;
use crate::plugin::score::{display_score, update_score};

mod kickoff;
mod paddle;
mod playground;
mod puck;
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
            .add_plugins(PhysicsPlugins::default().with_length_unit(100.))
            .add_plugins(PhysicsDebugPlugin::default())
            .insert_resource(Gravity(Vector::NEG_Y * 9.81 * 100.0))
            .add_systems(Startup, add_playground)
            .add_systems(Startup, add_paddle)
            .add_systems(Startup, display_score)
            .add_systems(OnEnter(AppState::KickOff), display_action)
            .add_systems(OnExit(AppState::KickOff), clean_action)
            .add_systems(Update, kickoff.run_if(in_state(AppState::KickOff)))
            .add_systems(OnEnter(AppState::InGame), add_puck)
            .add_systems(Update, move_paddle)
            // .add_systems(Update, display_events)
            .add_systems(Update, update_score)
            .add_systems(OnEnter(AppState::Goal), to_kickoff);
    }
}
