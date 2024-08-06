use avian3d::prelude::{Collider, Friction, LinearVelocity, LockedAxes, Restitution, RigidBody};
use bevy::asset::Assets;
use bevy::input::gamepad::GamepadButtonInput;
use bevy::input::{ButtonInput, ButtonState};
use bevy::pbr::{MaterialMeshBundle, StandardMaterial};
use bevy::prelude::{
    default, Color, Commands, Component, EventReader, Gamepad, GamepadButtonType, KeyCode, Mesh,
    Name, Query, Res, ResMut, Transform, Vec3, With, Without,
};

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
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const PADDLE_HEIGHT: f32 = 10.0;
    const PADDLE_RADIUS: f32 = 20.0;
    let paddle_collider = Collider::cylinder(PADDLE_RADIUS, PADDLE_HEIGHT);
    let mesh = meshes.add(bevy::prelude::Cylinder::new(PADDLE_RADIUS, PADDLE_HEIGHT));
    let material = materials.add(Color::srgb(0.0, 1.0, 0.0));

    commands.spawn((
        Name::new("PADDLE Left"),
        Paddle,
        Left,
        MaterialMeshBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(-(WIDTH / 2.) + PADDLE_WIDTH + MARGIN, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        paddle_collider.clone(),
        Friction::new(0.),
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z()
            .lock_translation_x()
            .lock_translation_y(),
        Restitution::new(0.0).with_combine_rule(avian3d::prelude::CoefficientCombine::Min),
    ));
    commands.spawn((
        Name::new("PADDLE Right"),
        Paddle,
        Right,
        MaterialMeshBundle {
            mesh,
            material,
            transform: Transform::from_xyz((WIDTH / 2.) - PADDLE_WIDTH - MARGIN, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        paddle_collider.clone(),
        Friction::new(0.),
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z()
            .lock_translation_x()
            .lock_translation_y(),
        Restitution::new(0.0).with_combine_rule(avian3d::prelude::CoefficientCombine::Min),
    ));
}

pub fn move_paddle(
    mut gamepad_button: EventReader<GamepadButtonInput>,
    //mut gamepad_axis: EventReader<GamepadAxisChangedEvent>,
    keys: Res<ButtonInput<KeyCode>>,
    mut paddle_left: Query<&mut LinearVelocity, (With<Paddle>, With<Left>, Without<Right>)>,
    mut paddle_right: Query<&mut LinearVelocity, (With<Paddle>, With<Right>, Without<Left>)>,
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
            GamepadButtonType::DPadUp => Vec3::Z,
            GamepadButtonType::DPadDown => Vec3::NEG_Z,
            _ => continue,
        };

        match state {
            ButtonState::Pressed => {
                println!("{:?}", button_type);
                // paddle.single_mut().0 = direction * PADDLE_SPEED;
                match gamepad_id {
                    0 => paddle_left.single_mut().0 = direction * PADDLE_SPEED,
                    1 => paddle_right.single_mut().0 = direction * PADDLE_SPEED,
                    _ => continue,
                }
            }
            ButtonState::Released => {
                // paddle.single_mut().0 = Vec2::ZERO;
                match gamepad_id {
                    0 => paddle_left.single_mut().0 = Vec3::ZERO,
                    1 => paddle_right.single_mut().0 = Vec3::ZERO,
                    _ => continue,
                }
            }
        }
    }

    if keys.just_pressed(KeyCode::ArrowUp) {
        paddle_right.single_mut().0 = Vec3::Z * PADDLE_SPEED;
    } else if keys.just_pressed(KeyCode::ArrowDown) {
        paddle_right.single_mut().0 = Vec3::NEG_Z * PADDLE_SPEED;
    } else if keys.just_released(KeyCode::ArrowUp) || keys.just_released(KeyCode::ArrowDown) {
        paddle_right.single_mut().0 = Vec3::ZERO;
    }

    if keys.just_pressed(KeyCode::KeyW) {
        paddle_left.single_mut().0 = Vec3::Z * PADDLE_SPEED;
    } else if keys.just_pressed(KeyCode::KeyS) {
        paddle_left.single_mut().0 = Vec3::NEG_Z * PADDLE_SPEED;
    } else if keys.just_released(KeyCode::KeyW) || keys.just_released(KeyCode::KeyS) {
        paddle_left.single_mut().0 = Vec3::ZERO;
    }
}
