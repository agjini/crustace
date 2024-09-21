use crate::plugin::paddle::Player;
use crate::plugin::shake::Shake;
use avian3d::prelude::{Collider, RigidBody, Sensor};
use bevy::asset::Assets;
use bevy::core::Name;
use bevy::math::Vec3;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    default, AmbientLight, Camera3dBundle, Color, Commands, Component, Cuboid, MaterialMeshBundle,
    Mesh, ResMut, Transform,
};
use blenvy::{BlueprintInfo, GameWorldTag, HideUntilReady, SpawnBlueprint};

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

pub fn add_playground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn((
    //     Camera3dBundle {
    //         transform: Transform::from_xyz(0., 1000., 1000.).looking_at(Vec3::ZERO, Vec3::Y),
    //         ..default()
    //     },
    //     Shake::new(0.0, 0.3, 0.2),
    // ));
    // commands.insert_resource(AmbientLight {
    //     color: Color::WHITE,
    //     brightness: 1000.0,
    //     ..default()
    // });

    commands.spawn((
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));

    // let mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::new(WIDTH / 2., HEIGHT / 2.)));
    // commands.spawn((
    //     Name::new("PLAYGROUND"),
    //     MaterialMeshBundle {
    //         mesh,
    //         material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //         ..default()
    //     },
    //     Friction::new(0.).with_combine_rule(CoefficientCombine::Min),
    //     RigidBody::Static,
    //     Collider::cuboid(WIDTH, 0., HEIGHT),
    // ));
    //
    // spawn_wall(Position::Top, &mut commands, &mut meshes, &mut materials);
    // spawn_wall(Position::Bottom, &mut commands, &mut meshes, &mut materials);
    // spawn_wall(Position::Right, &mut commands, &mut meshes, &mut materials);
    // spawn_wall(Position::Left, &mut commands, &mut meshes, &mut materials);
    //
    // spawn_goal(Player::Left, &mut commands, &mut meshes, &mut materials);
    // spawn_goal(Player::Right, &mut commands, &mut meshes, &mut materials);
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
