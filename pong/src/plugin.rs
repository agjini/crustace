use std::ops::Deref;
use bevy::app::{Startup, Update};
use bevy::prelude::{App, Assets, Camera2dBundle, Color, ColorMaterial, Commands, Component, default, IntoSystemConfigs, Mesh, Plugin, Query, Rectangle, Res, ResMut, Resource, Time, Timer, Transform, With};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::time::TimerMode;
use rand::random;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
            app.add_systems(Startup, add_paddle);
    }
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Component)]
struct Paddle;


// #[derive(Resource)]
// struct GreetTimer(Timer);

pub fn add_paddle(mut commands: Commands,
                  mut meshes: ResMut<Assets<Mesh>>,
                  mut materials: ResMut<Assets<ColorMaterial>>,)
{   commands.spawn(Camera2dBundle::default());
    commands.spawn((Paddle,Position { x: 0.0, y: 0.0 }));
    commands.spawn((
            Paddle,
            Position { x: 100.0, y: 0.0 },
            MaterialMesh2dBundle{
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
                material: materials.add(Color::rgb(0.0, 1.0, 0.0)),
                transform: Transform::from_xyz(
            // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
            0.0,
            0.0,
            0.0,
        ),
        ..default()}));
}

// pub fn greet_people(time: Res<Time>,
//                     mut timer: ResMut<GreetTimer>,
//                     query: Query<&Name, With<Person>>) {
//     // update our timer with the time elapsed since the last update
//     // if that caused the timer to finish, we say hello to everyone
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in &query {
//             println!("hello {}!", name.0);
//         }
//     }
// }