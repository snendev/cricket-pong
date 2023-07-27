use bevy::{
    prelude::{Commands, EventReader},
    sprite::SpriteSheetBundle,
};

use naia_bevy_client::{
    events::{InsertComponentEvents, MessageEvents},
    Client, CommandsExt,
};

use cricket_pong_game::base::{
    components::{
        batter::Batter,
        fielder::{Fielder, FielderPosition, FielderRing},
        player::{PlayerOne, PlayerTwo},
    },
    protocol::{channels::PlayerAssignmentChannel, messages::PlayerAssignmentMessage},
};

use super::{
    components::{PredictionOf, SourceOf},
    MyPlayer,
};

pub fn receive_entity_assignment_message(
    mut event_reader: EventReader<MessageEvents>,
    mut commands: Commands,
    client: Client,
) {
    for event in event_reader.iter() {
        for assignment in event.read::<PlayerAssignmentChannel, PlayerAssignmentMessage>() {
            let entity = assignment.entity.get(&client).unwrap();
            let prediction_entity = commands.entity(entity).insert(MyPlayer).id();
        }
    }
}

pub fn receive_insert_component_events(
    mut commands: Commands,
    mut event_reader: EventReader<InsertComponentEvents>,
) {
    for event in event_reader.iter() {
        // for entity in event.read::<Raft>() {
        //     let prediction_entity = commands
        //         .entity(entity)
        //         .duplicate()
        //         .insert(PredictionOf(entity))
        //         .id();

        //     commands.entity(entity).insert(SourceOf(prediction_entity));
        // }
        // for entity in event.read::<Car>() {
        //     let prediction_entity = commands
        //         .entity(entity)
        //         .duplicate()
        //         .insert(PredictionOf(entity))
        //         .id();

        //     commands.entity(entity).insert(SourceOf(prediction_entity));
        // }
    }
}
