use bevy_ecs::{
    prelude::{EventReader, Local, Query, With, Without},
    schedule::SystemSet,
};
use bevy_log::debug;

use cricket_pong_base::{
    components::{
        ball::Ball,
        boundary::Boundary,
        fielder::Fielder,
        phase::{GamePhase, GamePhaseKind},
        player::{Identity, PlayerOne, PlayerTwo, Position, PositionKind},
        scoreboard::{BowlResult, BowlScore, Scoreboard},
        wicket::Wicket,
    },
    lobby::components::GameInstance,
    rapier::{prelude::CollisionEvent, rapier::prelude::CollisionEventFlags},
};

use crate::ShouldTick;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct ScoringSet;

pub(crate) fn register_goals(
    mut collision_events: EventReader<CollisionEvent>,
    mut lobby_query: Query<(&GameInstance, &mut GamePhase), With<ShouldTick>>,
    mut player_one_query: Query<
        (&GameInstance, &mut Position),
        (With<PlayerOne>, Without<PlayerTwo>, With<ShouldTick>),
    >,
    mut player_two_query: Query<
        (&GameInstance, &mut Position),
        (With<PlayerTwo>, Without<PlayerOne>, With<ShouldTick>),
    >,
    mut scoreboard_query: Query<(&GameInstance, &mut Scoreboard), With<ShouldTick>>,
    ball_query: Query<&GameInstance, (With<Ball>, With<ShouldTick>)>,
    wicket_query: Query<&GameInstance, (With<Wicket>, With<ShouldTick>)>,
    boundary_query: Query<&GameInstance, (With<Boundary>, With<ShouldTick>)>,
    fielder_query: Query<&GameInstance, (With<Fielder>, With<ShouldTick>)>,
    mut pass_count: Local<u8>,
) {
    for (game_instance, mut phase) in lobby_query.iter_mut() {
        let Some(mut player_one_position) = player_one_query.iter_mut().find_map(|(instance, player)| {
            if instance == game_instance {
                Some(player)
            } else {
                None
            }
        }) else { continue };
        let Some(mut player_two_position) = player_two_query.iter_mut().find_map(|(instance, player)| {
            if instance == game_instance {
                Some(player)
            } else {
                None
            }
        }) else { continue };
        let Some(mut scoreboard) = scoreboard_query.iter_mut().find_map(|(instance, scoreboard)| {
            if instance == game_instance {
                Some(scoreboard)
            } else {
                None
            }
        }) else { return };

        let current_phase = phase.inner();

        let mut score_points = move |scored_points: u16, scoring_position: PositionKind| {
            debug!(
                "Scoring {} points for {} in instance ({})",
                scored_points, scoring_position, game_instance
            );
            let scorer = if player_one_position.is_kind(scoring_position) {
                Identity::One
            } else if player_two_position.is_kind(scoring_position) {
                Identity::Two
            } else {
                return;
            };

            let bowl_result = scoreboard.push(BowlScore {
                scorer,
                value: scored_points,
            });
            phase.set_bowling();
            match bowl_result {
                BowlResult::None => {}
                BowlResult::ChangePositions => {
                    player_one_position.switch();
                    player_two_position.switch();
                }
                BowlResult::GameOver => {
                    phase.set_game_over();
                }
            }
        };

        for event in collision_events.iter() {
            // score 1 for batter if the ball goes outside the boundary
            if let CollisionEvent::Stopped(entity1, entity2, flags) = event {
                if flags.contains(CollisionEventFlags::REMOVED) {
                    continue;
                };
                let other_entity = if ball_query
                    .get(*entity1)
                    .is_ok_and(|instance| instance == game_instance)
                {
                    *entity2
                } else if ball_query.contains(*entity2) {
                    *entity1
                } else {
                    continue;
                };
                if boundary_query
                    .get(other_entity)
                    .is_ok_and(|instance| instance == game_instance)
                {
                    score_points(1, PositionKind::Batter);
                    *pass_count = 0;
                }
            }
            if current_phase == GamePhaseKind::Bowling {
                continue;
            }
            if let CollisionEvent::Started(entity1, entity2, _flags) = event {
                let other_entity = if ball_query
                    .get(*entity1)
                    .is_ok_and(|instance| instance == game_instance)
                {
                    *entity2
                } else if ball_query.contains(*entity2) {
                    *entity1
                } else {
                    continue;
                };
                if wicket_query
                    .get(other_entity)
                    .is_ok_and(|instance| instance == game_instance)
                {
                    // score 3 for fielder if the ball hits the wicket
                    score_points(3, PositionKind::Fielder);
                    *pass_count = 0;
                } else if fielder_query
                    .get(other_entity)
                    .is_ok_and(|instance| instance == game_instance)
                {
                    // score 1 for fielder if the ball is passed between paddles 5 times
                    *pass_count += 1;
                    if *pass_count >= 5 {
                        score_points(1, PositionKind::Fielder);
                        *pass_count = 0;
                    }
                }
            }
        }
    }
}
