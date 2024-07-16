use bevy::asset::Assets;
use bevy::input::gamepad::GamepadButtonInput;
use bevy::input::{ButtonInput, ButtonState};
use bevy::prelude::{
    default, Circle, Color, ColorMaterial, Commands, Component, EventReader, GamepadButtonType,
    KeyCode, Mesh, Name, Query, Res, ResMut, Transform, Vec3, With, Without,
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier3d::dynamics::{CoefficientCombineRule, LockedAxes, RigidBody, Velocity};
use bevy_rapier3d::geometry::{Collider, Friction};
use bevy_rapier3d::prelude::Ccd;

use crate::plugin::playground::{MARGIN, WIDTH};

const PADDLE_WIDTH: f32 = 40.0;
const PADDLE_SPEED: f32 = 1000.;

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
    // let mesh = Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT)));
    let mesh = Mesh2dHandle(meshes.add(Circle::new(PADDLE_WIDTH)));
    let material = materials.add(Color::srgb(0.0, 1.0, 0.0));

    commands.spawn((
        Name::new("Left Paddle"),
        Paddle,
        Left,
        MaterialMesh2dBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(-(WIDTH / 2.) + PADDLE_WIDTH + MARGIN, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        // Collider::cuboid(PADDLE_WIDTH/2., PADDLE_HEIGHT/2.),
        Collider::ball(PADDLE_WIDTH),
        Velocity::zero(),
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_X,
        Ccd::enabled(),
    ));
    commands.spawn((
        Name::new("Right Paddle"),
        Paddle,
        Right,
        MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform::from_xyz((WIDTH / 2.) - PADDLE_WIDTH - MARGIN, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        // Collider::cuboid(PADDLE_WIDTH/2., PADDLE_HEIGHT/2.),
        Collider::ball(PADDLE_WIDTH),
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
    mut gamepad_button: EventReader<GamepadButtonInput>,
    //mut gamepad_axis: EventReader<GamepadAxisChangedEvent>,
    keys: Res<ButtonInput<KeyCode>>,
    mut paddle_left: Query<&mut Velocity, (With<Left>, Without<Right>)>,
    mut paddle_right: Query<&mut Velocity, (With<Right>, Without<Left>)>,
) {
    for ev in gamepad_button.read() {
        println!("{:?}", ev);

        let gamepad_id = ev.button.gamepad.id;
        let button_type = ev.button.button_type;
        let state = ev.state;

        // let paddle = match gamepad_id {
        //     0 => &mut paddle_left,
        //     1 => &mut paddle_right,
        //     _ => continue,
        // };

        let direction = match button_type {
            GamepadButtonType::DPadUp => Vec3::Y,
            GamepadButtonType::DPadDown => Vec3::NEG_Y,
            _ => continue,
        };

        match state {
            ButtonState::Pressed => {
                println!("{:?}", button_type);
                // paddle.single_mut().linvel = direction * PADDLE_SPEED;
                match gamepad_id {
                    0 => paddle_left.single_mut().linvel = direction * PADDLE_SPEED,
                    1 => paddle_right.single_mut().linvel = direction * PADDLE_SPEED,
                    _ => continue,
                }
            }
            ButtonState::Released => {
                // paddle.single_mut().linvel = Vec2::ZERO;
                match gamepad_id {
                    0 => paddle_left.single_mut().linvel = Vec3::ZERO,
                    1 => paddle_right.single_mut().linvel = Vec3::ZERO,
                    _ => continue,
                }
            }
        }
    }

    if keys.pressed(KeyCode::ArrowUp) {
        paddle_right.single_mut().linvel = Vec3::Y * PADDLE_SPEED;
    } else if keys.pressed(KeyCode::ArrowDown) {
        paddle_right.single_mut().linvel = Vec3::NEG_Y * PADDLE_SPEED;
    } else if keys.just_released(KeyCode::ArrowUp) || keys.just_released(KeyCode::ArrowDown) {
        paddle_right.single_mut().linvel = Vec3::ZERO;
    }

    if keys.pressed(KeyCode::KeyW) {
        paddle_left.single_mut().linvel = Vec3::Y * PADDLE_SPEED;
    } else if keys.pressed(KeyCode::KeyS) {
        paddle_left.single_mut().linvel = Vec3::NEG_Y * PADDLE_SPEED;
    } else if keys.just_released(KeyCode::KeyW) || keys.just_released(KeyCode::KeyS) {
        paddle_left.single_mut().linvel = Vec3::ZERO;
    }
}
