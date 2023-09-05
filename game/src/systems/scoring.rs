use bevy_ecs::{
    prelude::{Event, EventReader, EventWriter, Query, With, Without},
    schedule::SystemSet,
};
use bevy_log::debug;

use cricket_pong_base::{
    components::{
        ball::Ball,
        boundary::Boundary,
        fielder::Fielder,
        instance::GameInstance,
        phase::GamePhase,
        player::{Identity, PlayerOne, PlayerTwo, Position},
        scoreboard::{BowlResult, BowlScore, Scoreboard},
        wicket::Wicket,
    },
    rapier::{prelude::CollisionEvent, rapier::prelude::CollisionEventFlags},
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct ScoringSet;

#[derive(Event)]
pub(crate) struct ScoreEvent(GameInstance, Position, u8);

type WithWicket = (With<Wicket>, Without<Ball>);
type WithBoundary = (With<Boundary>, Without<Ball>);
type WithFielder = (With<Fielder>, Without<Ball>);
pub(crate) fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut score_writer: EventWriter<ScoreEvent>,
    mut ball_query: Query<(&GameInstance, &mut Ball)>,
    game_phase_query: Query<(&GameInstance, &GamePhase), Without<Ball>>,
    wicket_query: Query<&GameInstance, WithWicket>,
    boundary_query: Query<&GameInstance, WithBoundary>,
    fielder_query: Query<&GameInstance, WithFielder>,
) {
    for event in collision_events.iter() {
        let collision_data = match event {
            CollisionEvent::Started(entity1, entity2, _) => Some((*entity1, *entity2, false)),
            CollisionEvent::Stopped(entity1, entity2, flags) => {
                if flags.contains(CollisionEventFlags::REMOVED) {
                    None
                } else {
                    Some((*entity1, *entity2, true))
                }
            }
        };
        let Some((entity1, entity2, is_stopped_collision)) = collision_data else { continue; };

        let (game_instance, mut ball, other_entity) =
            if let Ok((instance, ball)) = ball_query.get_mut(entity1) {
                (instance, ball, entity2)
            } else if let Ok((instance, ball)) = ball_query.get_mut(entity2) {
                (instance, ball, entity1)
            } else {
                continue;
            };

        let Some(phase) = game_phase_query.iter().find_map(|(instance, phase)| {
            if instance == game_instance {
                Some(phase)
            } else {
                None
            }
        }) else { continue; };

        if phase.is_bowling() {
            continue;
        }

        // score 1 for batter if the ball goes outside the boundary
        if is_stopped_collision {
            if boundary_query
                .get(other_entity)
                .is_ok_and(|instance| instance == game_instance)
            {
                score_writer.send(ScoreEvent(game_instance.clone(), Position::Batter, 1));
                ball.passes = 0;
            }
        } else if wicket_query
            .get(other_entity)
            .is_ok_and(|instance| instance == game_instance)
        {
            // score 3 for fielder if the ball hits the wicket
            score_writer.send(ScoreEvent(game_instance.clone(), Position::Fielder, 3));
            ball.passes = 0;
        } else if fielder_query
            .get(other_entity)
            .is_ok_and(|instance| instance == game_instance)
        {
            // score 1 for fielder if the ball is passed between paddles 5 times
            ball.passes += 1;
            if ball.passes >= 5 {
                score_writer.send(ScoreEvent(game_instance.clone(), Position::Fielder, 1));
                ball.passes = 0;
            }
        }
    }
}

type WithPlayer<Player, NotPlayer> = (With<Player>, Without<NotPlayer>);

pub(crate) fn register_goals(
    mut score_events: EventReader<ScoreEvent>,
    mut player_one_query: Query<(&GameInstance, &mut Position), WithPlayer<PlayerOne, PlayerTwo>>,
    mut player_two_query: Query<(&GameInstance, &mut Position), WithPlayer<PlayerTwo, PlayerOne>>,
    mut scoreboard_query: Query<(&GameInstance, &mut Scoreboard)>,
    mut game_phase_query: Query<(&GameInstance, &mut GamePhase), Without<Ball>>,
) {
    for ScoreEvent(game_instance, scoring_position, scored_points) in score_events.iter() {
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
        let Some(mut phase) = game_phase_query.iter_mut().find_map(|(instance, phase)| {
            if instance == game_instance {
                Some(phase)
            } else {
                None
            }
        }) else { return };

        debug!(
            "Scoring {} points for {} in instance ({:?})",
            scored_points, scoring_position, game_instance,
        );
        let scorer = if *player_one_position == *scoring_position {
            Identity::One
        } else if *player_two_position == *scoring_position {
            Identity::Two
        } else {
            return;
        };

        let bowl_result = scoreboard.push(BowlScore::new(scorer, *scored_points));
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
    }
}
