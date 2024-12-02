use avian3d::prelude::{
    CoefficientCombine, Collider, LinearVelocity, LockedAxes, Mass, Restitution, RigidBody,
};
use bevy::asset::{AssetServer, Assets};
use bevy::core::Name;
use bevy::prelude::{
    default, Color, Commands, Component, MaterialMeshBundle, Query, Res, ResMut, StandardMaterial,
    Transform, Vec3, With,
};
use rand::Rng;

const INITIAL_VELOCITY: f32 = 20.0;
const PUCK_RADIUS: f32 = 0.25;
const PUCK_HEIGHT: f32 = 0.08;

#[derive(Component)]
pub(crate) struct Puck;

pub fn add_puck(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mesh = asset_server.load("blueprints/Puck.glb#Mesh0/Primitive0");
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
            transform: Transform::from_xyz(0.0, 0.2, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cylinder(PUCK_RADIUS, PUCK_HEIGHT),
        Mass(1.),
        Restitution::new(1.).with_combine_rule(CoefficientCombine::Min),
        Puck,
    ));
}

pub fn reset_puck(mut puck: Query<(&mut Transform, &mut LinearVelocity), With<Puck>>) {
    let (mut transform, mut velocity) = puck.single_mut();
    transform.translation.x = 0.0;
    transform.translation.z = 0.0;
    velocity.0 = Vec3::ZERO;
}
