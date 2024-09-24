use avian3d::prelude::{
    CoefficientCombine, Collider, Friction, LinearVelocity, LockedAxes, Restitution, RigidBody,
};
use bevy::asset::AssetServer;
use bevy::prelude::{
    default, Commands, Component, Gamepad, Name, PbrBundle, Query, Reflect, Res, Transform, Vec3,
    With,
};
use leafwing_input_manager::prelude::{ActionState, GamepadStick, InputMap, KeyboardVirtualDPad};
use leafwing_input_manager::{Actionlike, InputControlKind, InputManagerBundle};

const PADDLE_HEIGHT: f32 = 0.38;
const PADDLE_RADIUS: f32 = 0.4;
const PADDLE_SPEED: f32 = 15.;

#[derive(Component, Reflect)]
pub struct Paddle;

#[derive(Component, Reflect, PartialEq, Debug, Copy, Clone)]
pub enum Player {
    Left,
    Right,
}

pub fn add_paddle(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_paddle(&mut commands, &asset_server, Player::Left);
    spawn_paddle(&mut commands, &asset_server, Player::Right);
}

fn spawn_paddle(commands: &mut Commands, asset_server: &Res<AssetServer>, player: Player) {
    let paddle_collider = Collider::cylinder(PADDLE_RADIUS, PADDLE_HEIGHT);
    let mesh = asset_server.load("blueprints/Paddle.glb#Mesh0/Primitive0");
    let material = asset_server.load(format!("materials/Material{player:?}.glb#Material0"));
    commands.spawn((
        Name::new(format!("Paddle {player:?}")),
        PbrBundle {
            mesh,
            transform: match player {
                Player::Left => Transform::from_xyz(-7., 0.25, 0.),
                Player::Right => Transform::from_xyz(7., 0.25, 0.),
            },
            material,
            ..default()
        },
        Paddle,
        player,
        get_input_map(&player),
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

fn get_input_map(player: &Player) -> InputManagerBundle<Action> {
    match player {
        Player::Left => InputManagerBundle::with_map(
            InputMap::default()
                .with_dual_axis(Action::Move, GamepadStick::LEFT)
                .with_dual_axis(Action::Move, KeyboardVirtualDPad::WASD)
                .with_gamepad(Gamepad::new(0)),
        ),
        Player::Right => InputManagerBundle::with_map(
            InputMap::default()
                .with_dual_axis(Action::Move, GamepadStick::LEFT)
                .with_dual_axis(Action::Move, KeyboardVirtualDPad::ARROW_KEYS)
                .with_gamepad(Gamepad::new(1)),
        ),
    }
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
