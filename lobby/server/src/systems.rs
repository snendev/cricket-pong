use bevy_ecs::prelude::{Commands, Entity, EventReader, In, Query, ResMut};
use bevy_log::{debug, info};

use naia_bevy_server::{
    events::{ConnectEvent, DisconnectEvent},
    CommandsExt, RoomKey, Server,
};
use naia_shared::BigMapKey;

use common_lobby_protocol::components::{GameInstance, GameLobby};

use crate::resources::{QueuedUsers, UserEntities};

pub(crate) fn handle_user_connection(
    mut server: Server,
    mut event_reader: EventReader<ConnectEvent>,
    mut queued_players: ResMut<QueuedUsers>,
) {
    for ConnectEvent(user_key) in event_reader.iter() {
        let address = server.user_mut(user_key).address();
        info!("Queued for matchmaking: Client connected from {}", address);
        queued_players.users.push(*user_key);
    }
}

pub(crate) fn handle_room_cleanup(
    mut commands: Commands,
    mut server: Server,
    lobbies_query: Query<(Entity, &GameLobby, &GameInstance)>,
) {
    for (entity, lobby, instance) in lobbies_query.iter() {
        let room_key = RoomKey::from_u64(*instance.id);
        if lobby.is_unloading() && server.room(&room_key).users_count() == 0 {
            server.room_mut(&room_key).destroy();
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn handle_user_disconnection(
    mut reader: EventReader<DisconnectEvent>,
    mut user_entities: ResMut<UserEntities>,
    mut commands: Commands,
    mut queued_players: ResMut<QueuedUsers>,
) {
    for DisconnectEvent(user_key, _) in reader.iter() {
        let Some(entity) = user_entities.remove(user_key) else { continue; };
        if let Some((index, _)) = queued_players
            .users
            .iter()
            .enumerate()
            .find(|(_, key)| *key == user_key)
        {
            queued_players.users.swap_remove(index);
        }
        commands.entity(entity).despawn();
    }
}

pub fn subscribe_to_game_instances(
    In(entities): In<Vec<(GameInstance, Vec<Entity>)>>,
    mut commands: Commands,
    mut server: Server,
) {
    for (instance, entities) in entities.into_iter() {
        for entity in entities.into_iter() {
            let room_key = RoomKey::from_u64(*instance.id);
            if !server.room(&room_key).has_entity(&entity) {
                debug!(
                    "Adding entity ({:?}) to room {} and enabling replication",
                    entity, instance
                );
                commands.entity(entity).enable_replication(&mut server);
                server.room_mut(&room_key).add_entity(&entity);
            }
        }
    }
}
