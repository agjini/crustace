use bevy::prelude::{App, Update};
use bevy::window::close_on_esc;
use bevy::DefaultPlugins;

use pong::plugin::PongPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PongPlugin)
        .add_systems(Update, close_on_esc)
        .run();
}
