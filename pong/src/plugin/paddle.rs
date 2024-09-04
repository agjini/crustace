use avian3d::prelude::{
    CoefficientCombine, Collider, Friction, LinearVelocity, LockedAxes, Restitution, RigidBody,
};
use bevy::asset::Assets;
use bevy::input::ButtonInput;
use bevy::pbr::{MaterialMeshBundle, StandardMaterial};
use bevy::prelude::{
    default, Color, Commands, Component, Cylinder, Gamepad, GamepadButton, GamepadButtonType,
    KeyCode, Mesh, Name, Query, Res, ResMut, Transform, Vec3, With,
};

use crate::plugin::playground::{MARGIN, WIDTH};

const PADDLE_WIDTH: f32 = 40.0;
const PADDLE_SPEED: f32 = 1000.;

#[derive(Component)]
pub struct Paddle;

#[derive(Component, PartialEq, Debug, Copy, Clone)]
pub enum Player {
    Left,
    Right,
}

pub fn add_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const PADDLE_HEIGHT: f32 = 10.0;
    const PADDLE_RADIUS: f32 = 20.0;
    let paddle_collider = Collider::cylinder(PADDLE_RADIUS, PADDLE_HEIGHT);
    let mesh = meshes.add(Cylinder::new(PADDLE_RADIUS, PADDLE_HEIGHT));

    commands.spawn((
        Name::new("PADDLE Left"),
        Paddle,
        Player::Left,
        MaterialMeshBundle {
            mesh: mesh.clone(),
            material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
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
        Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
    ));
    commands.spawn((
        Name::new("PADDLE Right"),
        Paddle,
        Player::Right,
        MaterialMeshBundle {
            mesh,
            material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
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
        Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
    ));
}

pub fn move_paddle(
    key_input: Res<ButtonInput<KeyCode>>,
    gamepad_input: Res<ButtonInput<GamepadButton>>,
    mut paddles: Query<(&mut LinearVelocity, &Player), With<Paddle>>,
) {
    for (mut velocity, player) in paddles.iter_mut() {
        match get_action(player, &key_input, &gamepad_input) {
            Action::Up => velocity.0 = Vec3::NEG_Z * PADDLE_SPEED,
            Action::Down => velocity.0 = Vec3::Z * PADDLE_SPEED,
            Action::Stop => velocity.0 = Vec3::ZERO,
            Action::None => {}
        };
    }
}

enum Action {
    Up,
    Down,
    Stop,
    None,
}

fn get_action(
    player: &Player,
    key_input: &Res<ButtonInput<KeyCode>>,
    gamepad_input: &Res<ButtonInput<GamepadButton>>,
) -> Action {
    let (key_up, key_down, gamepad_id) = get_player_binding(player);
    let gamepad_up = GamepadButton::new(gamepad_id, GamepadButtonType::DPadUp);
    let gamepad_down = GamepadButton::new(gamepad_id, GamepadButtonType::DPadDown);

    if key_input.just_pressed(key_up) || gamepad_input.just_pressed(gamepad_up) {
        Action::Up
    } else if key_input.just_pressed(key_down) || gamepad_input.just_pressed(gamepad_down) {
        Action::Down
    } else if key_input.just_released(key_up)
        || gamepad_input.just_released(gamepad_up)
        || key_input.just_released(key_down)
        || gamepad_input.just_released(gamepad_down)
    {
        Action::Stop
    } else {
        Action::None
    }
}

fn get_player_binding(player: &Player) -> (KeyCode, KeyCode, Gamepad) {
    match player {
        Player::Left => (KeyCode::KeyW, KeyCode::KeyS, Gamepad::new(0)),
        Player::Right => (KeyCode::ArrowUp, KeyCode::ArrowDown, Gamepad::new(1)),
    }
}
