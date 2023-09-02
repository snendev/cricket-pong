use bevy::prelude::{debug, Commands, Entity, EventReader, Name, Query};

use cricket_pong_controls::Controller;
use cricket_pong_game::{
    base::components::{
        ball::Ball,
        batter::Batter,
        boundary::Boundary,
        fielder::{Fielder, FielderTrack},
        instance::GameLobby,
        phase::GamePhase,
        player::{PlayerOne, PlayerTwo, Position},
        scoreboard::{BowlScore, Scoreboard, ScoreboardBundle},
        wicket::Wicket,
    },
    GameInstance, ShouldTick,
};
use cricket_pong_graphics::ShouldRender;

use crate::networking::components::{PredictionOf, SourceOf};

pub fn receive_entity_assignment_message(
    mut event_reader: EventReader<MessageEvents>,
    mut commands: Commands,
    client: Client,
    positions_query: Query<&Position>,
) {
    for event in event_reader.iter() {
        for assignment in event.read::<PlayerAssignmentChannel, PlayerAssignmentMessage>() {
            let entity = assignment.entity.get(&client).unwrap();
            debug!("Local player assigned to entity ({:?})", entity);
            let position = positions_query.get(entity);
            commands.entity(entity).insert(Controller::One);
            if let Ok(position) = position {
                commands.entity(entity).insert(position.clone());
            }
        }
    }
}

pub fn receive_score_message(
    mut event_reader: EventReader<MessageEvents>,
    mut scoreboard_query: Query<&mut Scoreboard>,
) {
    for event in event_reader.iter() {
        for score_message in event.read::<ScoreMessageChannel, ScoreMessage>() {
            if let Ok(mut scoreboard) = scoreboard_query.get_single_mut() {
                // ScoreMessageChannel is OrderedReliable, so we should be able to do this without
                // overriding previous messages
                // however, we do want to force_set in case we have locally predicted a score
                scoreboard.force_set(
                    score_message.index,
                    BowlScore::new(score_message.scorer, score_message.value),
                );
            }
        }
    }
}

// pub fn handle_insert_position(
//     mut commands: Commands,
//     query: Query<(&Position, &SourceOf)>,
// ) {
//     for event in event_reader.iter() {
//         for entity in event.read::<Position>() {
//             match query.get(entity) {
//                 Ok((position, source)) => {
//                     commands.entity(source.0).insert(position.clone());
//                 }
//                 Err(error) => {
//                     debug!(
//                         "Warning: insert component event for non-source entity. {:?}",
//                         error
//                     );
//                 }
//             }
//         }
//     }
// }

fn spawn_prediction_entity(commands: &mut Commands, entity: Entity, name: Name) {
    let prediction_entity = commands
        .entity(entity)
        .duplicate()
        .insert((ShouldRender, ShouldTick, name.clone(), PredictionOf(entity)))
        .id();

    commands
        .entity(entity)
        .insert((name, SourceOf(prediction_entity)));
}
