use bevy::asset::Assets;
use bevy::math::Vec2;
use bevy::prelude::{
    default, Camera2dBundle, Color, ColorMaterial, Commands, Component, Entity, EventReader, Mesh,
    NextState, Query, Rectangle, ResMut, Transform, TransformBundle, With, Without,
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::plugin::RapierConfiguration;
use bevy_rapier2d::prelude::{ActiveEvents, CollisionEvent, Sensor};

use crate::plugin::ball::Ball;
use crate::plugin::paddle::{Left, Right};
use crate::plugin::score::Score;
use crate::plugin::AppState;

#[derive(Component)]
pub(crate) struct Wall;

#[derive(Eq, PartialEq, Component, Clone, Copy)]
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
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;
    commands.spawn(Camera2dBundle::default());

    commands.spawn((MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(WIDTH, HEIGHT))),
        material: materials.add(Color::rgb(0.0, 0.0, 0.0)),
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    },));

    spawn_wall(Position::Top, &mut commands);
    spawn_wall(Position::Right, &mut commands);
    spawn_wall(Position::Bottom, &mut commands);
    spawn_wall(Position::Left, &mut commands);
}

fn spawn_wall(position: Position, commands: &mut Commands) {
    let (width, height, x, y) = match position {
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

    let mut wall = commands.spawn((
        Collider::cuboid(width / 2., height / 2.),
        Wall,
        TransformBundle::from(Transform::from_xyz(x, y, 0.0)),
        RigidBody::Fixed,
    ));
    if position == Position::Left || position == Position::Right {
        wall.insert((Sensor, ActiveEvents::COLLISION_EVENTS));
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
    ball: Query<Entity, With<Ball>>,
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
