use avian3d::prelude::CollisionStarted;
use bevy::prelude::{
    default, Commands, Component, Entity, Event, EventReader, EventWriter, JustifyText, NextState,
    Node, PositionType, Query, ResMut, Text, TextFont, TextLayout, Val, With,
};

use crate::plugin::paddle::Player;
use crate::plugin::playground::Goal;
use crate::plugin::puck::Puck;
use crate::plugin::shake::Shake;
use crate::plugin::state::AppState;

#[derive(Component)]
pub struct Score(pub u8);

pub fn display_score(mut commands: Commands) {
    commands.spawn((
        Text::new("0"),
        TextFont::default().with_font_size(50.0),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Percent(45.0),
            ..default()
        },
        Score(0),
        Player::Left,
    ));

    commands.spawn((
        Text::new("0"),
        TextFont::default().with_font_size(50.0),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Percent(55.0),
            ..default()
        },
        Score(0),
        Player::Right,
    ));
}

pub fn update_score(mut query: Query<(&mut Text, &Score)>) {
    for (mut text, score) in query.iter_mut() {
        text.0 = format!("{}", score.0);
    }
}

#[derive(Event)]
pub struct GoalEvent {
    player: Player,
}

pub fn check_goal(
    puck: Query<Entity, With<Puck>>,
    goals: Query<(Entity, &Goal)>,
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut goal_event_writer: EventWriter<GoalEvent>,
) {
    let puck = puck.single();
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

pub fn increment_score_on_goal(
    mut goals: EventReader<GoalEvent>,
    mut query: Query<(&mut Score, &Player)>,
) {
    for goal in goals.read() {
        for (mut score, player) in query.iter_mut() {
            if player != &goal.player {
                score.0 += 1;
            }
        }
    }
}

pub fn shake_on_goal(mut goals: EventReader<GoalEvent>, mut shake_query: Query<&mut Shake>) {
    if goals.read().len() == 0 {
        return;
    }

    for mut shake in shake_query.iter_mut() {
        shake.add_time(0.45);
    }
}

pub fn goal_state_on_goal(
    mut goals: EventReader<GoalEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if goals.read().len() == 0 {
        return;
    }
    next_state.set(AppState::Goal);
}
