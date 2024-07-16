use bevy::asset::Assets;
use bevy::prelude::{
    default, Color, ColorMaterial, Commands, Component, Cylinder, MaterialMeshBundle, Mesh, ResMut,
    StandardMaterial, Transform, Vec3,
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier3d::dynamics::{CoefficientCombineRule, ExternalImpulse, RigidBody};
use bevy_rapier3d::geometry::ColliderMassProperties::Mass;
use bevy_rapier3d::geometry::{Collider, Friction, Restitution};
use bevy_rapier3d::prelude::{Ccd, Vect};
use rand::Rng;

const INITIAL_VELOCITY: f32 = 100.0;
const PUCK_RADIUS: f32 = 20.0;
const PUCK_HEIGHT: f32 = 5.0;

#[derive(Component)]
pub(crate) struct Puck;

pub fn add_puck(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mesh = meshes.add(Cylinder::new(PUCK_RADIUS, PUCK_HEIGHT));
    let material = materials.add(Color::srgb(1.0, 0.0, 0.0));
    let max_angle = std::f32::consts::PI / 4.;
    let mut angle: f32 = rng.gen_range(-max_angle..=max_angle);
    let start_left = rng.gen_bool(0.5);
    if start_left {
        angle += std::f32::consts::PI;
    }
    commands.spawn((
        MaterialMeshBundle {
            mesh,
            material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Ccd::enabled(),
        Friction {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        Collider::cylinder(PUCK_HEIGHT / 2., PUCK_RADIUS),
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        ExternalImpulse {
            impulse: Vec3::new(
                INITIAL_VELOCITY * f32::cos(angle),
                INITIAL_VELOCITY * f32::sin(angle),
                0.0,
            ),
            torque_impulse: Vect::ZERO,
        },
        Mass(0.2),
        Puck,
    ));
}
