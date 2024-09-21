use crate::plugin::playground::{MARGIN, WIDTH};
use avian3d::prelude::{
    CoefficientCombine, Collider, Friction, LinearVelocity, LockedAxes, Restitution, RigidBody,
};
use bevy::asset::Assets;
use bevy::pbr::{MaterialMeshBundle, StandardMaterial};
use bevy::prelude::{
    default, Color, Commands, Component, Cylinder, Gamepad, Mesh, Name, Query, Reflect, ResMut,
    Transform, Vec3, With,
};
use leafwing_input_manager::prelude::{
    ActionState, GamepadStick, InputMap, KeyboardVirtualAxis, KeyboardVirtualDPad,
};
use leafwing_input_manager::{Actionlike, InputControlKind, InputManagerBundle};

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
        InputManagerBundle::with_map(
            InputMap::default()
                .with_dual_axis(Action::Move, GamepadStick::LEFT)
                .with_dual_axis(Action::Move, KeyboardVirtualDPad::WASD)
                .with_gamepad(Gamepad::new(0)),
        ),
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
            .lock_translation_y(),
        Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
    ));
    commands.spawn((
        Name::new("PADDLE Right"),
        Paddle,
        Player::Right,
        InputManagerBundle::with_map(
            InputMap::default()
                .with_dual_axis(Action::Move, GamepadStick::LEFT)
                .with_dual_axis(Action::Move, KeyboardVirtualDPad::ARROW_KEYS)
                .with_gamepad(Gamepad::new(1)),
        ),
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
            .lock_translation_y(),
        Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
    ));
}

pub fn move_paddle(mut paddles: Query<(&mut LinearVelocity, &ActionState<Action>), With<Paddle>>) {
    for (mut velocity, action_state) in paddles.iter_mut() {
        if let Some(dual_axis_data) = action_state.dual_axis_data(&Action::Move) {
            velocity.0 =
                Vec3::new(dual_axis_data.pair.x, 0., -dual_axis_data.pair.y) * PADDLE_SPEED;
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Move,
}

impl Actionlike for Action {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Action::Move => InputControlKind::DualAxis,
        }
    }
}
