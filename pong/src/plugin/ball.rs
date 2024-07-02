use bevy::asset::Assets;
use bevy::math::Vec2;
use bevy::prelude::{
    default, Circle, Color, ColorMaterial, Commands, Component, Mesh, ResMut, Transform,
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::dynamics::{CoefficientCombineRule, ExternalImpulse, RigidBody};
use bevy_rapier2d::geometry::ColliderMassProperties::Mass;
use bevy_rapier2d::geometry::{Collider, Friction, Restitution};
use bevy_rapier2d::prelude::Ccd;
use rand::Rng;

const INITIAL_VELOCITY: f32 = 100.0;

#[derive(Component)]
pub(crate) struct Ball;

pub fn add_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mesh = Mesh2dHandle(meshes.add(Circle::new(10.0)));
    let material = materials.add(Color::rgb(1.0, 0.0, 0.0));
    let max_angle = std::f32::consts::PI / 4.;
    let mut angle: f32 = rng.gen_range(-max_angle..=max_angle);
    let start_left = rng.gen_bool(0.5);
    if start_left {
        angle += std::f32::consts::PI;
    }
    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Ccd::enabled(),
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        Collider::ball(10.0),
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        ExternalImpulse {
            impulse: Vec2::new(
                INITIAL_VELOCITY * f32::cos(angle),
                INITIAL_VELOCITY * f32::sin(angle),
            ),
            torque_impulse: 0.0,
        },
        Mass(0.2),
        Ball,
    ));
}
