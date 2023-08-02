use bevy::prelude::{debug, Commands, Entity, EventReader, Name, Query};

use cricket_pong_controls::Controller;
use cricket_pong_graphics::ShouldRender;
use naia_bevy_client::{
    events::{InsertComponentEvents, MessageEvents},
    Client, CommandsExt,
};

use cricket_pong_game::{
    base::{
        components::{
            ball::Ball,
            batter::Batter,
            boundary::Boundary,
            fielder::{Fielder, FielderTrack},
            phase::GamePhase,
            player::{PlayerOne, PlayerTwo, Position},
            scoreboard::Scoreboard,
            wicket::Wicket,
        },
        protocol::{channels::PlayerAssignmentChannel, messages::PlayerAssignmentMessage},
    },
    lobby::components::GameLobby,
    ShouldTick,
};

use super::components::{PredictionOf, SourceOf};

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

// TODO:
// BALL NOT SPAWNING ON CLIENTS
// POSITION INCONSISTENTLY ATTACHED SERVER SIDE

pub fn handle_insert_position(
    mut commands: Commands,
    mut event_reader: EventReader<InsertComponentEvents>,
    query: Query<(&Position, &SourceOf)>,
) {
    for event in event_reader.iter() {
        for entity in event.read::<Position>() {
            match query.get(entity) {
                Ok((position, source)) => {
                    commands.entity(source.0).insert(position.clone());
                }
                Err(error) => {
                    debug!(
                        "Warning: insert component event for non-source entity. {:?}",
                        error
                    );
                }
            }
        }
    }
}

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

pub fn spawn_predictions(
    mut commands: Commands,
    mut event_reader: EventReader<InsertComponentEvents>,
) {
    for event in event_reader.iter() {
        for entity in event.read::<PlayerOne>() {
            spawn_prediction_entity(&mut commands, entity, PlayerOne::name());
        }
        for entity in event.read::<PlayerTwo>() {
            spawn_prediction_entity(&mut commands, entity, PlayerTwo::name());
        }
        for entity in event.read::<Ball>() {
            spawn_prediction_entity(&mut commands, entity, Ball::name());
        }
        for entity in event.read::<Batter>() {
            spawn_prediction_entity(&mut commands, entity, Batter::name());
        }
        for entity in event.read::<Boundary>() {
            spawn_prediction_entity(&mut commands, entity, Boundary::name());
        }
        for entity in event.read::<Fielder>() {
            spawn_prediction_entity(&mut commands, entity, Fielder::name());
        }
        for entity in event.read::<FielderTrack>() {
            spawn_prediction_entity(&mut commands, entity, FielderTrack::name());
        }
        for entity in event.read::<GamePhase>() {
            spawn_prediction_entity(&mut commands, entity, GameLobby::name());
        }
        for entity in event.read::<Wicket>() {
            spawn_prediction_entity(&mut commands, entity, Wicket::name());
        }
        for entity in event.read::<Scoreboard>() {
            spawn_prediction_entity(&mut commands, entity, Scoreboard::name());
        }
    }
}
