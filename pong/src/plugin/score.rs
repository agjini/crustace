use avian3d::prelude::CollisionStarted;
use bevy::prelude::{
    default, Commands, Component, Entity, Event, EventReader, EventWriter, JustifyText,
    PositionType, Query, Style, Text, TextBundle, TextStyle, Val, With,
};

use crate::plugin::paddle::Player;
use crate::plugin::playground::Goal;
use crate::plugin::puck::Puck;
use crate::plugin::shake::Shake;

#[derive(Component)]
pub struct Score(pub u8);

pub fn display_score(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Percent(45.0),
            ..default()
        }),
        Score(0),
        Player::Left,
    ));

    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Percent(55.0),
            ..default()
        }),
        Score(0),
        Player::Right,
    ));
}

pub fn update_score(mut query: Query<(&mut Text, &Score)>) {
    for (mut text, score) in query.iter_mut() {
        text.sections[0].value = format!("{}", score.0);
    }
}

#[derive(Event)]
pub struct GoalEvent {
    player: Player,
}

pub fn check_goals(
    puck: Query<Entity, With<Puck>>,
    goals: Query<(Entity, &Goal)>,
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut goal_event_writer: EventWriter<GoalEvent>,
) {
    let puck_entity = puck.get_single().ok();
    if let None = puck_entity {
        return;
    }

    let puck = puck_entity.unwrap();
    for CollisionStarted(e1, e2) in collision_event_reader.read() {
        if *e1 != puck && *e2 != puck {
            continue;
        }

        if let Some((_, goal)) = goals.iter().find(|(e, _)| e == e1 || e == e2) {
            goal_event_writer.send(GoalEvent { player: goal.0 });
            continue;
        }
    }
}

pub fn shake_on_goal(
    mut goals: EventReader<GoalEvent>,
    mut query: Query<(&mut Score, &Player)>,
    mut shake_query: Query<&mut Shake>,
) {
    for goal in goals.read() {
        for (mut score, player) in query.iter_mut() {
            if player == &goal.player {
                score.0 += 1;
                for mut shake in shake_query.iter_mut() {
                    shake.add_time(0.45);
                }
            }
        }
    }
}
