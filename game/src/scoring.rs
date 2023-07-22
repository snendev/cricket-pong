use bevy_ecs::prelude::{EventReader, Local, NextState, Query, ResMut, Resource};

use bevy_rapier2d::{prelude::CollisionEvent, rapier::prelude::CollisionEventFlags};

use cricket_pong_base::{
    ball::Ball,
    batter::Wicket,
    fielder::{Boundary, Fielder},
    Player, Position,
};

use crate::gameplay::GamePhase;

pub(crate) struct BowlScore {
    pub scorer: Position,
    pub value: u16,
}

// AKA an "inning"
#[derive(Resource, Default)]
pub(crate) struct Over(Vec<BowlScore>);

pub(crate) enum BowlResult {
    None,
    ChangePositions,
}

impl Over {
    pub(crate) fn get(&self, index: usize) -> Option<&BowlScore> {
        self.0.get(index)
    }

    pub(crate) fn push(&mut self, score: BowlScore) -> BowlResult {
        if self.0.len() == 6 {
            self.0.clear();
        }
        self.0.push(score);
        if self.0.len() == 6 {
            BowlResult::ChangePositions
        } else {
            BowlResult::None
        }
    }
}

pub(crate) fn register_goals(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(&mut Player, &mut Position)>,
    ball_query: Query<&Ball>,
    wicket_query: Query<&Wicket>,
    boundary_query: Query<&Boundary>,
    fielder_query: Query<&Fielder>,
    mut over: ResMut<Over>,
    mut state: ResMut<NextState<GamePhase>>,
    mut pass_count: Local<u8>,
) {
    let mut score_points = |scored_points: u16, scoring_position: Position| {
        let bowl_result = over.push(BowlScore {
            scorer: scoring_position,
            value: scored_points,
        });
        for (mut player, mut position) in player_query.iter_mut() {
            if *position == scoring_position {
                player.score += scored_points;
            }
            match bowl_result {
                BowlResult::None => {}
                BowlResult::ChangePositions => {
                    *position = !*position;
                }
            }
        }
        state.set(GamePhase::Bowling);
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
