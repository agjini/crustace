use crate::plugin::kickoff::{display_action, kickoff, to_kickoff};
use crate::plugin::paddle::{add_paddle, move_paddle, Action, Paddle, Player};
use crate::plugin::playground::add_playground;
use crate::plugin::puck::add_puck;
use crate::plugin::score::{
    check_goal, display_score, increment_score_on_goal, shake_on_goal, update_score, GoalEvent,
};
use crate::plugin::shake::ShakePlugin;
use crate::plugin::state::AppState;
use avian3d::math::Vector;
use avian3d::prelude::Gravity;
use avian3d::PhysicsPlugins;
use bevy::app::{Startup, Update};
use bevy::prelude::{in_state, App, AppExtStates, IntoSystemConfigs, OnEnter, Plugin};
use bevy_map_camera::MapCameraPlugin;
use leafwing_input_manager::plugin::InputManagerPlugin;

mod kickoff;
mod paddle;
mod playground;
mod puck;
mod score;
mod shake;
mod state;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .enable_state_scoped_entities::<AppState>()
            .add_event::<GoalEvent>()
            .register_type::<Player>()
            .register_type::<Paddle>()
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_plugins(PhysicsPlugins::default().with_length_unit(1.))
            // .add_plugins(PhysicsDebugPlugin::default())
            .add_plugins(MapCameraPlugin)
            .add_plugins(ShakePlugin)
            .insert_resource(Gravity(Vector::NEG_Y * 9.81 * 1.0))
            .add_systems(Startup, (add_playground, add_paddle).chain())
            .add_systems(Startup, display_score)
            .add_systems(OnEnter(AppState::KickOff), display_action)
            .add_systems(Update, kickoff.run_if(in_state(AppState::KickOff)))
            .add_systems(OnEnter(AppState::InGame), add_puck)
            .add_systems(Update, check_goal)
            .add_systems(Update, increment_score_on_goal)
            .add_systems(Update, shake_on_goal)
            .add_systems(Update, move_paddle)
            .add_systems(Update, update_score)
            .add_systems(OnEnter(AppState::Goal), to_kickoff);
    }
}
