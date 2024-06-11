use bevy::app::{Startup, Update};
use bevy::prelude::{App, Plugin};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;

use crate::plugin::ball::add_ball;
use crate::plugin::paddle::{add_paddle, move_paddle};
use crate::plugin::playground::add_playground;

mod ball;
mod paddle;
mod playground;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, add_playground)
            .add_systems(Startup, add_paddle)
            .add_systems(Startup, add_ball)
            .add_systems(Update, move_paddle);
    }
}
