use bevy_ecs::{
    event::EventReader,
    system::{Commands, Query, ResMut},
};
use bevy_log::info;

use naia_bevy_server::{
    events::{ConnectEvent, DisconnectEvent, ErrorEvent},
    CommandsExt, Server,
};

use cricket_pong_game::base::{
    components::player::{PlayerOne, PlayerTwo},
    protocol::{channels::PlayerAssignmentChannel, messages::PlayerAssignmentMessage},
};

use crate::UserEntities;

pub fn connect_events(
    mut commands: Commands,
    mut server: Server,
    mut user_entities: ResMut<UserEntities>,
    mut event_reader: EventReader<ConnectEvent>,
    player_one_query: Query<&PlayerOne>,
    player_two_query: Query<&PlayerTwo>,
) {
    for ConnectEvent(user_key) in event_reader.iter() {
        let room_key = server
            .room_keys()
            .into_iter()
            .next()
            .unwrap_or_else(|| server.make_room().key());

        let address = server.user_mut(user_key).enter_room(&room_key).address();

        info!("Client connected from: {}", address);

        let player_builder = if player_one_query.is_empty() {
            Some(commands.spawn(PlayerOne))
        } else if player_two_query.is_empty() {
            Some(commands.spawn(PlayerTwo))
        } else {
            None
        };

        if let Some(mut player_builder) = player_builder {
            let player_entity = player_builder.enable_replication(&mut server).id();
            server.room_mut(&room_key).add_entity(&player_entity);
            user_entities.insert(*user_key, player_entity);

            let mut assignment_message = PlayerAssignmentMessage::new();
            assignment_message.entity.set(&server, &player_entity);
            server.send_message::<PlayerAssignmentChannel, PlayerAssignmentMessage>(
                user_key,
                &assignment_message,
            );
        }

        // also maybe initialize game
    }
}

pub fn disconnect_events(
    mut commands: Commands,
    mut server: Server,
    mut user_entities: ResMut<UserEntities>,
    mut event_reader: EventReader<DisconnectEvent>,
) {
    for DisconnectEvent(user_key, user) in event_reader.iter() {
        info!("bingo Server disconnected from: {:?}", user.address);

        if let Some(entity) = user_entities.remove(user_key) {
            let room_keys = {
                let user_ref = server.user(user_key);
                user_ref.room_keys().map(|key| *key).collect::<Vec<_>>()
            };
            for room_key in room_keys.into_iter() {
                server.room_mut(&room_key).remove_entity(&entity);
            }
            commands.entity(entity).despawn();
        }
    }
}

pub fn error_events(mut event_reader: EventReader<ErrorEvent>) {
    for ErrorEvent(error) in event_reader.iter() {
        info!("bingo Server Error: {:?}", error);
    }
}
