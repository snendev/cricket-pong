use bevy::prelude::{debug, Commands, Entity, EventReader, Query, With};

use cricket_pong_controls::Controller;
use cricket_pong_game::base::components::player::{Identity, PlayerOne, PlayerTwo};
use network_base::messages::PlayerAssignmentMessageEvent;

pub fn receive_entity_assignment_message(
    mut event_reader: EventReader<PlayerAssignmentMessageEvent>,
    player_one_query: Query<Entity, With<PlayerOne>>,
    player_two_query: Query<Entity, With<PlayerTwo>>,
    mut commands: Commands,
) {
    for event in event_reader.iter() {
        let entity = match event.identity {
            Identity::One => player_one_query.get_single(),
            Identity::Two => player_two_query.get_single(),
        };
        if let Ok(entity) = entity {
            commands.entity(entity).insert(Controller::One);
        } else {
            debug!("Error: Player assignment event received without matching entity");
        }
    }
}

// pub fn receive_score_message(
//     mut event_reader: EventReader<MessageEvents>,
//     mut scoreboard_query: Query<&mut Scoreboard>,
// ) {
//     for event in event_reader.iter() {
//         for score_message in event.read::<ScoreMessageChannel, ScoreMessage>() {
//             if let Ok(mut scoreboard) = scoreboard_query.get_single_mut() {
//                 // ScoreMessageChannel is OrderedReliable, so we should be able to do this without
//                 // overriding previous messages
//                 // however, we do want to force_set in case we have locally predicted a score
//                 scoreboard.force_set(
//                     score_message.index,
//                     BowlScore::new(score_message.scorer, score_message.value),
//                 );
//             }
//         }
//     }
// }
