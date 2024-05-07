use bevy::DefaultPlugins;
use bevy::prelude::App;
use pong::plugin::PongPlugin;

fn main() {
    App::new()
         .add_plugins(DefaultPlugins)
        .add_plugins(PongPlugin)
        .run();
}


