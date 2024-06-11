use bevy::asset::Assets;
use bevy::math::Vec2;
use bevy::prelude::{
    default, Camera2dBundle, Color, ColorMaterial, Commands, Component, Mesh, Rectangle, ResMut,
    Transform, TransformBundle,
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::plugin::RapierConfiguration;

#[derive(Component)]
struct Playground;

#[derive(Component)]
struct Wall;

enum Position {
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
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Playground,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(WIDTH, HEIGHT))),
            material: materials.add(Color::rgb(0.0, 0.0, 0.0)),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        },
    ));

    spawn_wall(Position::Top, &mut commands);
    spawn_wall(Position::Right, &mut commands);
    spawn_wall(Position::Bottom, &mut commands);
    spawn_wall(Position::Left, &mut commands);
}

fn spawn_wall(position: Position, commands: &mut Commands) {
    let (width, height, x, y) = match position {
        Position::Top => (
            WIDTH + WALL_WIDTH * 2.,
            WALL_WIDTH,
            0.0,
            HEIGHT / 2. + WALL_WIDTH / 2.,
        ),
        Position::Right => (WALL_WIDTH, HEIGHT, WIDTH / 2. + WALL_WIDTH / 2., 0.0),
        Position::Bottom => (
            WIDTH + WALL_WIDTH * 2.,
            WALL_WIDTH,
            0.0,
            -HEIGHT / 2. - WALL_WIDTH / 2.,
        ),
        Position::Left => (WALL_WIDTH, HEIGHT, -WIDTH / 2. - WALL_WIDTH / 2., 0.0),
    };

    commands.spawn((
        Collider::cuboid(width / 2., height / 2.),
        Wall,
        TransformBundle::from(Transform::from_xyz(x, y, 0.0)),
        RigidBody::Fixed,
    ));
}
