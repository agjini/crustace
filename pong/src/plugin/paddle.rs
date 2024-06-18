use bevy::asset::Assets;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{
    default, Color, ColorMaterial, Commands, Component, KeyCode, Mesh, Query, Rectangle, Res,
    ResMut, Transform, With, Without,
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::dynamics::{CoefficientCombineRule, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{Collider, Friction};
use bevy_rapier2d::prelude::Ccd;

use crate::plugin::playground::{MARGIN, WIDTH};

const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SPEED: f32 = 600.;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Right;

#[derive(Component)]
pub struct Left;

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
        Velocity::zero(),
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_X,
        Ccd::enabled(),
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
        Velocity::zero(),
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_X,
        Ccd::enabled(),
    ));
}

pub fn move_paddle(
    keys: Res<ButtonInput<KeyCode>>,
    mut paddle_left: Query<&mut Velocity, (With<Left>, Without<Right>)>,
    mut paddle_right: Query<&mut Velocity, (With<Right>, Without<Left>)>,
) {
    if keys.pressed(KeyCode::ArrowUp) {
        paddle_right.single_mut().linvel = Vec2::Y * PADDLE_SPEED;
    } else if keys.pressed(KeyCode::ArrowDown) {
        paddle_right.single_mut().linvel = Vec2::NEG_Y * PADDLE_SPEED;
    } else {
        paddle_right.single_mut().linvel = Vec2::ZERO;
    }

    if keys.pressed(KeyCode::KeyW) {
        paddle_left.single_mut().linvel = Vec2::Y * PADDLE_SPEED;
    } else if keys.pressed(KeyCode::KeyS) {
        paddle_left.single_mut().linvel = Vec2::NEG_Y * PADDLE_SPEED;
    } else {
        paddle_left.single_mut().linvel = Vec2::ZERO;
    }
}
