use itertools::Itertools;

use bevy_ecs::{
    entity::Entity,
    prelude::{EventReader, ResMut, Resource},
    system::{Commands, Local, Query},
};
use bevy_log::debug;
use bevy_utils::HashMap;

use bevy_replicon::renet::ServerEvent;

use cricket_pong_game::{
    base::components::{
        instance::{GameInstance, GameLobby},
        player::{PlayerOne, PlayerTwo},
    },
    ShouldTick,
};

#[derive(Clone, Debug, Default, Resource)]
pub(crate) struct QueuedUsers(Vec<u64>);

#[derive(Clone, Debug, Default, Resource)]
pub(crate) struct UserEntities {
    user_to_entity_map: HashMap<u64, Entity>,
    entity_to_user_map: HashMap<Entity, u64>,
}

impl UserEntities {
    pub fn insert(&mut self, user_key: u64, entity: Entity) {
        self.user_to_entity_map.insert(user_key, entity);
        self.entity_to_user_map.insert(entity, user_key);
    }

    pub fn remove(&mut self, user: &u64) -> Option<Entity> {
        self.user_to_entity_map.remove(user).map(|entity| {
            self.entity_to_user_map.remove(&entity);
            entity
        })
    }
}

pub(crate) fn handle_user_connections(
    mut commands: Commands,
    mut server_events: EventReader<ServerEvent>,
    mut queued_players: ResMut<QueuedUsers>,
    mut user_entities: ResMut<UserEntities>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                debug!("Queued for matchmaking: New client {} connected", client_id);
                queued_players.0.push(*client_id);
                let player_entity = commands.spawn_empty().id();
                user_entities.insert(*client_id, player_entity);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                debug!("Disconnected user {}: {}", client_id, reason);
                if let Some((index, _)) = queued_players
                    .0
                    .iter()
                    .enumerate()
                    .find(|(_, key)| *key == client_id)
                {
                    queued_players.0.swap_remove(index);
                }
                if let Some(entity) = user_entities.remove(client_id) {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

pub(crate) fn handle_room_cleanup(
    mut commands: Commands,
    lobbies_query: Query<(Entity, &GameLobby)>,
) {
    for (entity, lobby) in lobbies_query.iter() {
        if lobby.is_unloading() {
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn pair_queued_users(
    mut commands: Commands,
    mut user_entities: ResMut<UserEntities>,
    mut queued_players: ResMut<QueuedUsers>,
    mut next_room_key: Local<u64>,
) {
    let num_queued = queued_players.0.len();
    let num_matched = num_queued - (num_queued % 2);
    for (user_one, user_two) in queued_players.0.drain(..).tuples() {
        let room_key = *next_room_key;
        *next_room_key += 1;

        debug!(
            "Game starting in room {} with users {} (one) and {} (two)",
            room_key, user_one, user_two,
        );
        let instance = GameInstance::new(room_key);

        // player one
        let player_one_entity = commands
            .spawn((PlayerOne, PlayerOne::name(), instance.clone(), ShouldTick))
            .id();

        user_entities.insert(user_one, player_one_entity);
        // let mut assignment_message = PlayerAssignmentMessage::new();
        // assignment_message.entity.set(&server, &player_one_entity);
        // server.send_message::<PlayerAssignmentChannel, PlayerAssignmentMessage>(
        //     &user_one,
        //     &assignment_message,
        // );

        // player two
        let player_two_entity = commands
            .spawn((PlayerTwo, PlayerTwo::name(), instance.clone(), ShouldTick))
            .id();
        user_entities.insert(user_two, player_two_entity);
        // server
        //     .room_mut(&room_key)
        //     .add_entity(&player_two_entity)
        //     .add_user(&user_two);
        // let mut assignment_message = PlayerAssignmentMessage::new();
        // assignment_message.entity.set(&server, &player_two_entity);
        // server.send_message::<PlayerAssignmentChannel, PlayerAssignmentMessage>(
        //     &user_two,
        //     &assignment_message,
        // );

        // spawn lobby
        commands.spawn((
            GameLobby::default(),
            GameLobby::name(),
            instance,
            ShouldTick,
        ));
    }
}
