use crate::plugin::paddle::Player;
use crate::plugin::shake::Shake;
use avian3d::prelude::{
    CoefficientCombine, Collider, ColliderConstructor, ColliderConstructorHierarchy, Friction,
    Restitution, RigidBody, Sensor,
};
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::core_pipeline::Skybox;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_map_camera::{LookTransform, MapCameraBundle};

#[derive(Component)]
pub struct Goal(pub Player);

#[derive(Debug, Eq, PartialEq, Component, Clone, Copy)]
pub enum Position {
    Top,
    Right,
    Bottom,
    Left,
}

pub const WIDTH: f32 = 16.0;
pub const HEIGHT: f32 = 2.0;
pub const WALL_WIDTH: f32 = 0.5;

pub fn add_playground(mut commands: Commands, asset_server: Res<AssetServer>) {
    let skybox = asset_server.load("textures/skybox.ktx2");

    commands.spawn((
        Skybox {
            image: skybox,
            brightness: 1000.,
        },
        MapCameraBundle::new_with_transform(LookTransform::new(
            Vec3 {
                x: 0.,
                y: 11.5,
                z: 13.8,
            },
            Vec3::ZERO,
            Vec3::Y,
        )),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("textures/diffuse_map.ktx2"),
            specular_map: asset_server.load("textures/skybox.ktx2"),
            intensity: 1000.0,
        },
        Shake::new(0., 0.6, 15.),
    ));
    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(1.0, 5.0, 0.0),
    //     point_light: PointLight {
    //         intensity: 1.0,
    //         color: Color::srgb(210.0, 17., 13.),
    //         radius: 0.02,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     ..default()
    // });

    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(1.0, 5.0, -3.0),
    //     point_light: PointLight {
    //         intensity: 2.0,
    //         color: Color::srgb(210.0, 17., 13.),
    //         radius: 0.02,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     ..default()
    // });
    let scene = asset_server.load("blueprints/Playground.glb#Scene0");
    commands.spawn((
        Name::new("Playground"),
        SceneBundle { scene, ..default() },
        Friction::new(0.).with_combine_rule(CoefficientCombine::Min),
        Restitution::new(1.).with_combine_rule(CoefficientCombine::Min),
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
    ));
    spawn_goal(Player::Left, &mut commands);
    spawn_goal(Player::Right, &mut commands);
}

fn spawn_goal(player: Player, commands: &mut Commands) {
    let goal_depth = WALL_WIDTH;
    let goal_width = 2.2;
    let goal_height = 0.5;
    let (width, depth, x, z) = match player {
        Player::Right => (
            goal_depth,
            goal_width,
            goal_depth / 2. + WIDTH / 2. - WALL_WIDTH,
            0.0,
        ), // blue
        Player::Left => (goal_depth, goal_width, goal_depth / 2. - WIDTH / 2., 0.0), // red
    };

    commands.spawn((
        Name::new(format!("GOAL {player:?}")),
        Transform::from_xyz(x, goal_height / 2., z),
        Collider::cuboid(width, goal_height, depth),
        Sensor::default(),
        Goal(player),
        RigidBody::Static,
    ));
}
