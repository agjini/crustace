use bevy::app::{Startup, Update};
use bevy::prelude::{App, Commands,IntoSystemConfigs, Component, Plugin, Query, With};
use rand::random;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
        .add_systems(Update, (add_people, update_people, greet_people).chain());
    }
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(String, i32);


pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string(), random())));
    commands.spawn((Person, Name("Renzo Hume".to_string(), random())));
    commands.spawn((Person, Name("Zayna Nieves".to_string(), random())));
    println!("people added");
}

pub fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            println!("changing name");
            name.0 = "Elaina Hume".to_string();
        }
    }
}

pub fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}