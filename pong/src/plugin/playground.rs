use avian3d::prelude::{CoefficientCombine, Collider, Friction, RigidBody};
use bevy::asset::Assets;
use bevy::core::Name;
use bevy::math::{Vec2, Vec3};
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    default, AmbientLight, Camera3dBundle, Color, Commands, Component, Cuboid, MaterialMeshBundle,
    Mesh, Plane3d, ResMut, Transform,
};

use crate::plugin::paddle::Player;

#[derive(Component)]
pub(crate) struct Wall;

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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1000., 1000.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
        ..default()
    });

    let mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::new(WIDTH / 2., HEIGHT / 2.)));
    commands.spawn((
        Name::new("PLAYGROUND"),
        MaterialMeshBundle {
            mesh,
            material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Friction::new(0.).with_combine_rule(CoefficientCombine::Min),
        RigidBody::Static,
        Collider::cuboid(WIDTH, 0., HEIGHT),
    ));

    spawn_wall(Position::Top, &mut commands, &mut meshes, &mut materials);
    spawn_wall(Position::Bottom, &mut commands, &mut meshes, &mut materials);
    spawn_wall(Position::Right, &mut commands, &mut meshes, &mut materials);
    spawn_wall(Position::Left, &mut commands, &mut meshes, &mut materials);
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
    let mut wall = commands.spawn((
        Name::new(format!("WALL {position:?}")),
        MaterialMeshBundle {
            mesh,
            material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
            transform: Transform::from_xyz(x, 25., z),
            ..default()
        },
        Collider::cuboid(width, 50., height),
        Wall,
        RigidBody::Static,
    ));
    if position == Position::Left || position == Position::Right {
        //wall.insert((Sensor, ActiveEvents::COLLISION_EVENTS));
        match position {
            Position::Left => wall.insert(Player::Left),
            Position::Right => wall.insert(Player::Right),
            _ => panic!("Invalid position"),
        };
    }
}
