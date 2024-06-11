use bevy::asset::Assets;
use bevy::math::Vec2;
use bevy::prelude::{default, Circle, Color, ColorMaterial, Commands, Mesh, ResMut, Transform};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::dynamics::{CoefficientCombineRule, ExternalImpulse, RigidBody};
use bevy_rapier2d::geometry::ColliderMassProperties::Mass;
use bevy_rapier2d::geometry::{Collider, Friction, Restitution};
use bevy_rapier2d::prelude::Ccd;

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
            impulse: Vec2::new(100.0, 0.0),
            torque_impulse: 0.0,
        },
        Mass(0.2),
    ));
}
