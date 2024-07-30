use bevy::asset::Assets;
use bevy::core::Name;
use bevy::math::{Vec2, Vec3};
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    default, AmbientLight, Camera3dBundle, Color, Commands, Component, Cuboid, Entity, EventReader,
    MaterialMeshBundle, Mesh, NextState, Plane3d, Query, ResMut, Transform, With, Without,
};
use bevy_rapier3d::dynamics::{Ccd, RigidBody};
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::prelude::{CollisionEvent, Friction};

use crate::plugin::paddle::{Left, Right};
use crate::plugin::puck::Puck;
use crate::plugin::score::Score;
use crate::plugin::AppState;

#[derive(Component)]
pub(crate) struct Wall;

#[derive(Debug, Eq, PartialEq, Component, Clone, Copy)]
pub enum Position {
    Top,
    Right,
    Bottom,
    Left,
}

pub const WIDTH: f32 = 1024.0;
pub const HEIGHT: f32 = 768.0;
pub const WALL_WIDTH: f32 = 100.0;
pub const MARGIN: f32 = 10.;

pub fn add_playground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1000., -1000.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
        ..default()
    });

    let mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::new(WIDTH / 2., HEIGHT / 2.)));
    commands.spawn((
        Name::new("PLAYGROUND"),
        MaterialMeshBundle {
            mesh,
            material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Friction::new(0.),
        Ccd::enabled(),
        Collider::cuboid(WIDTH / 2., 0., HEIGHT / 2.),
    ));

    spawn_wall(Position::Top, &mut commands, &mut meshes, &mut materials);
    spawn_wall(Position::Bottom, &mut commands, &mut meshes, &mut materials);
    spawn_wall(Position::Right, &mut commands, &mut meshes, &mut materials);
    spawn_wall(Position::Left, &mut commands, &mut meshes, &mut materials);
}

fn spawn_wall(
    position: Position,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let (width, height, x, z) = match position {
        Position::Top => (
            WIDTH + WALL_WIDTH * 2.,
            WALL_WIDTH,
            0.0,
            HEIGHT / 2. + WALL_WIDTH / 2.,
        ),
        Position::Right => (WALL_WIDTH, HEIGHT, WIDTH / 2. + WALL_WIDTH / 2., 0.0),
        Position::Bottom => (
            WIDTH + WALL_WIDTH * 2.,
            WALL_WIDTH,
            0.0,
            -HEIGHT / 2. - WALL_WIDTH / 2.,
        ),
        Position::Left => (WALL_WIDTH, HEIGHT, -WIDTH / 2. - WALL_WIDTH / 2., 0.0),
    };

    let mesh = meshes.add(Cuboid::new(width, 50., height));
    let mut wall = commands.spawn((
        Name::new(format!("WALL {position:?}")),
        MaterialMeshBundle {
            mesh,
            material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
            transform: Transform::from_xyz(x, 25., z),
            ..default()
        },
        Collider::cuboid(width / 2., 50. / 2., height / 2.),
        Ccd::enabled(),
        Wall,
        RigidBody::Fixed,
    ));
    if position == Position::Left || position == Position::Right {
        //wall.insert((Sensor, ActiveEvents::COLLISION_EVENTS));
        match position {
            Position::Left => wall.insert(Left),
            Position::Right => wall.insert(Right),
            _ => panic!("Invalid position"),
        };
    }
}

pub fn display_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    wall_left: Query<Entity, (With<Wall>, With<Left>, Without<Right>)>,
    wall_right: Query<Entity, (With<Wall>, With<Right>, Without<Left>)>,
    mut score_left: Query<&mut Score, (With<Left>, Without<Right>)>,
    mut score_right: Query<&mut Score, (With<Right>, Without<Left>)>,
    ball: Query<Entity, With<Puck>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for collision_event in collision_events.read() {
        if let &CollisionEvent::Started(e1, e2, _) = collision_event {
            if e1 == wall_left.single() || e2 == wall_left.single() {
                println!("Collision with left wall => e2");
                commands.entity(ball.single()).despawn();
                score_right.single_mut().0 += 1;
                next_state.set(AppState::Goal);
            }
            if e1 == wall_right.single() || e2 == wall_right.single() {
                println!("Collision with right wall => e2");
                commands.entity(ball.single()).despawn();
                score_left.single_mut().0 += 1;
                next_state.set(AppState::Goal);
            }
        }
        // println!("Received collision event: {:?}", collision_event);
    }
}
