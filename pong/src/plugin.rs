use bevy::app::{Startup, Update};
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{
    default, App, Assets, Camera2dBundle, Circle, Color, ColorMaterial, Commands, Component,
    KeyCode, Mesh, Plugin, Query, Rectangle, Res, ResMut, Time, Transform, TransformBundle, With,
    Without,
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::{Collider, RapierConfiguration, RapierDebugRenderPlugin, RigidBody};

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        let mut configuration = RapierConfiguration::new(100.0);
        configuration.gravity = Vec2::new(0.0, 0.0);
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .insert_resource(configuration)
            .add_systems(Startup, add_playground)
            .add_systems(Startup, add_paddle)
            .add_systems(Startup, add_ball)
            .add_systems(Update, move_paddle);
    }
}

#[derive(Component)]
struct Playground;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Right;

#[derive(Component)]
struct Left;

const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 768.0;
const WALL_WIDTH: f32 = 2.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SPEED: f32 = 600.;
const MARGIN: f32 = 10.;

pub fn add_playground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

    commands.spawn((
        Collider::cuboid(WIDTH / 2., WALL_WIDTH),
        Wall,
        TransformBundle::from(Transform::from_xyz(
            0.,
            HEIGHT / 2. + (WALL_WIDTH / 2.),
            0.0,
        )),
        RigidBody::Fixed,
    ));

    commands.spawn((
        Collider::cuboid(WIDTH / 2., WALL_WIDTH),
        Wall,
        TransformBundle::from(Transform::from_xyz(
            0.,
            -HEIGHT / 2. - (WALL_WIDTH / 2.),
            0.0,
        )),
        RigidBody::Fixed,
    ));

    commands.spawn((
        Collider::cuboid(WALL_WIDTH, HEIGHT / 2.),
        Wall,
        TransformBundle::from(Transform::from_xyz(
            -WIDTH / 2. - (WALL_WIDTH / 2.),
            0.,
            0.0,
        )),
        RigidBody::Fixed,
    ));
    commands.spawn((
        Collider::cuboid(WALL_WIDTH, HEIGHT / 2.),
        Wall,
        TransformBundle::from(Transform::from_xyz(WIDTH / 2. + (WALL_WIDTH / 2.), 0., 0.0)),
        RigidBody::Fixed,
    ));
}

pub fn add_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh2dHandle(meshes.add(Circle::new(10.0)));
    let material = materials.add(Color::rgb(1.0, 0.0, 0.0));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(10.0),
    ));
}

pub fn add_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT)));
    let material = materials.add(Color::rgb(0.0, 1.0, 0.0));

    commands.spawn((
        Paddle,
        Left,
        MaterialMesh2dBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(-(WIDTH / 2.) + PADDLE_WIDTH / 2. + MARGIN, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(10.0, 50.0),
    ));
    commands.spawn((
        Paddle,
        Right,
        MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform::from_xyz((WIDTH / 2.) - PADDLE_WIDTH / 2. - MARGIN, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(10.0, 50.0),
    ));
}

fn move_paddle(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut paddle_left: Query<&mut Transform, (With<Left>, Without<Right>)>,
    mut paddle_right: Query<&mut Transform, (With<Right>, Without<Left>)>,
) {
    if keys.pressed(KeyCode::ArrowUp) {
        paddle_right.single_mut().translation.y += PADDLE_SPEED * time.delta().as_secs_f32();
    }
    if keys.pressed(KeyCode::ArrowDown) {
        paddle_right.single_mut().translation.y -= PADDLE_SPEED * time.delta().as_secs_f32();
    }
    if keys.pressed(KeyCode::KeyW) {
        paddle_left.single_mut().translation.y += PADDLE_SPEED * time.delta().as_secs_f32();
    }
    if keys.pressed(KeyCode::KeyS) {
        paddle_left.single_mut().translation.y -= PADDLE_SPEED * time.delta().as_secs_f32();
    }
}
