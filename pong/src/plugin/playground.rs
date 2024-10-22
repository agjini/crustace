use crate::plugin::paddle::Player;
use crate::plugin::shake::Shake;
use avian3d::prelude::{
    CoefficientCombine, Collider, ColliderConstructor, Friction, Restitution, RigidBody, Sensor,
};
use bevy::asset::{AssetServer, Assets};
use bevy::color::palettes::css::WHITE;
use bevy::core::Name;
use bevy::core_pipeline::Skybox;
use bevy::math::Vec3;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{
    default, AmbientLight, Camera, Camera3dBundle, Color, Commands, Component, Cuboid,
    MaterialMeshBundle, Mesh, PointLight, PointLightBundle, Res, ResMut, Transform,
};

#[derive(Component)]
pub struct Goal(pub Player);

#[derive(Debug, Eq, PartialEq, Component, Clone, Copy)]
pub enum Position {
    Top,
    Right,
    Bottom,
    Left,
}

pub const WIDTH: f32 = 1024.0;
pub const HEIGHT: f32 = 768.0;
pub const WALL_WIDTH: f32 = 100.0;
pub const MARGIN: f32 = 10.;

pub fn add_playground(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let skybox = asset_server.load("kloofendal_48d_partly_cloudy_puresky_2k.hdr");

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 11.5, 13.8).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        // Skybox {
        //     image: skybox,
        //     brightness: 1000.0,
        // },
        Shake::new(0., 0.6, 15.),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.,
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(1.0, 5.0, 0.0),
        point_light: PointLight {
            intensity: 600_000.0,
            color: WHITE.into(),
            shadows_enabled: true,
            ..default()
        },

        ..default()
    });

    let mesh = asset_server.load("blueprints/Playground.glb#Mesh0/Primitive0");
    let material = asset_server.load("materials/PlaygroundMaterial.glb#Material0");
    commands.spawn((
        Name::new("Playground"),
        PbrBundle {
            mesh,
            material,
            ..default()
        },
        Friction::new(0.).with_combine_rule(CoefficientCombine::Min),
        Restitution::new(1.).with_combine_rule(CoefficientCombine::Min),
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
    ));
}

fn spawn_wall(
    position: Position,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let (width, height, x, z) = match position {
        Position::Top => (
            WIDTH + WALL_WIDTH * 2.,
            WALL_WIDTH,
            0.0,
            -HEIGHT / 2. - WALL_WIDTH / 2.,
        ),
        Position::Right => (WALL_WIDTH, HEIGHT, WIDTH / 2. + WALL_WIDTH / 2., 0.0),
        Position::Bottom => (
            WIDTH + WALL_WIDTH * 2.,
            WALL_WIDTH,
            0.0,
            HEIGHT / 2. + WALL_WIDTH / 2.,
        ),
        Position::Left => (WALL_WIDTH, HEIGHT, -WIDTH / 2. - WALL_WIDTH / 2., 0.0),
    };

    let mesh = meshes.add(Cuboid::new(width, 50., height));
    commands.spawn((
        Name::new(format!("WALL {position:?}")),
        MaterialMeshBundle {
            mesh,
            material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
            transform: Transform::from_xyz(x, 25., z),
            ..default()
        },
        Collider::cuboid(width, 50., height),
        RigidBody::Static,
    ));
}

fn spawn_goal(
    player: Player,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let goal_width = WALL_WIDTH;
    let goal_height = HEIGHT / 4.;
    let (width, height, x, z) = match player {
        Player::Left => (
            goal_width,
            goal_height,
            -WIDTH / 2. - goal_width / 2. + 5.,
            0.0,
        ),
        Player::Right => (
            goal_width,
            goal_height,
            WIDTH / 2. + goal_width / 2. - 5.,
            0.0,
        ),
    };

    let mesh = meshes.add(Cuboid::new(width, 60., height));
    commands.spawn((
        Name::new(format!("GOAL {player:?}")),
        MaterialMeshBundle {
            mesh,
            material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
            transform: Transform::from_xyz(x, 25., z),
            ..default()
        },
        Collider::cuboid(width, 60., height),
        Sensor::default(),
        Goal(player),
        RigidBody::Static,
    ));
}
