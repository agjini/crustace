use avian3d::prelude::Restitution;
use avian3d::prelude::{
    CoefficientCombine, Collider, ExternalImpulse, Friction, LockedAxes, Mass, RigidBody,
};
use bevy::asset::Assets;
use bevy::core::Name;
use bevy::prelude::{
    default, Color, Commands, Component, Cylinder, MaterialMeshBundle, Mesh, ResMut,
    StandardMaterial, Transform, Vec3,
};
use rand::Rng;

const INITIAL_VELOCITY: f32 = 10000000.0;
const PUCK_RADIUS: f32 = 20.0;
const PUCK_HEIGHT: f32 = 10.0;

#[derive(Component)]
pub(crate) struct Puck;

pub fn add_puck(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mesh = meshes.add(Cylinder::new(PUCK_RADIUS, PUCK_HEIGHT));
    let material = materials.add(Color::srgb(0.0, 0.0, 0.7));
    let max_angle = std::f32::consts::PI / 4.;
    let mut angle: f32 = rng.gen_range(-max_angle..=max_angle);
    let start_left = rng.gen_bool(0.5);
    if start_left {
        angle += std::f32::consts::PI;
    }
    commands.spawn((
        Name::new("PUCK"),
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z()
            .lock_translation_y(),
        MaterialMeshBundle {
            mesh,
            material,
            transform: Transform::from_xyz(0.0, PUCK_HEIGHT, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Friction::new(0.).with_combine_rule(CoefficientCombine::Min),
        Collider::cylinder(PUCK_RADIUS, PUCK_HEIGHT),
        ExternalImpulse::new(Vec3::new(
            INITIAL_VELOCITY * f32::cos(angle),
            0.0,
            INITIAL_VELOCITY * f32::sin(angle),
        )),
        Mass(1.),
        Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
        Puck,
    ));
}
