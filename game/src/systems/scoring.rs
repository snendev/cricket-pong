use bevy_ecs::{
    prelude::{EventReader, Local, NextState, Query, ResMut, With, Without},
    schedule::SystemSet,
};

use bevy_rapier2d::{prelude::CollisionEvent, rapier::prelude::CollisionEventFlags};

use cricket_pong_base::components::{
    ball::Ball,
    boundary::Boundary,
    fielder::Fielder,
    player::{Identity, PlayerOne, PlayerTwo, Position},
    scoreboard::{BowlResult, BowlScore, Scoreboard},
    wicket::Wicket,
};

use crate::GamePhase;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct ScoringSet;

pub(crate) fn register_goals(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_one_query: Query<&mut Position, (With<PlayerOne>, Without<PlayerTwo>)>,
    mut player_two_query: Query<&mut Position, (With<PlayerTwo>, Without<PlayerOne>)>,
    ball_query: Query<&Ball>,
    wicket_query: Query<&Wicket>,
    boundary_query: Query<&Boundary>,
    fielder_query: Query<&Fielder>,
    mut scoreboard_query: Query<&mut Scoreboard>,
    mut state: ResMut<NextState<GamePhase>>,
    mut pass_count: Local<u8>,
) {
    let mut score_points = move |scored_points: u16, scoring_position: Position| {
        let mut player_one_position = player_one_query.single_mut();
        let mut player_two_position = player_two_query.single_mut();
        let mut scoreboard = scoreboard_query.single_mut();
        let scorer = if *player_one_position == scoring_position {
            Identity::One
        } else if *player_two_position == scoring_position {
            Identity::Two
        } else {
            return;
        };

        let bowl_result = scoreboard.push(BowlScore {
            scorer,
            value: scored_points,
        });
        state.set(GamePhase::Bowling);
        match bowl_result {
            BowlResult::None => {}
            BowlResult::ChangePositions => {
                *player_one_position = !*player_one_position;
                *player_two_position = !*player_two_position;
            }
            BowlResult::GameOver => {
                state.set(GamePhase::GameOver);
            }
        }
    };

    for event in collision_events.iter() {
        // score 1 for batter if the ball goes outside the boundary
        if let CollisionEvent::Stopped(entity1, entity2, flags) = event {
            if flags.contains(CollisionEventFlags::REMOVED) {
                continue;
            };
            let other_entity = if ball_query.contains(*entity1) {
                *entity2
            } else if ball_query.contains(*entity2) {
                *entity1
            } else {
                continue;
            };
            if boundary_query.contains(other_entity) {
                score_points(1, Position::Batter);
                *pass_count = 0;
            }
        }
        if let CollisionEvent::Started(entity1, entity2, _flags) = event {
            let other_entity = if ball_query.contains(*entity1) {
                *entity2
            } else if ball_query.contains(*entity2) {
                *entity1
            } else {
                continue;
            };
            if wicket_query.contains(other_entity) {
                // score 3 for fielder if the ball hits the wicket
                score_points(3, Position::Fielder);
                *pass_count = 0;
            } else if fielder_query.contains(other_entity) {
                // score 1 for fielder if the ball is passed between paddles 5 times
                *pass_count += 1;
                if *pass_count >= 5 {
                    score_points(1, Position::Fielder);
                    *pass_count = 0;
                }
            }
        }
    }
}
