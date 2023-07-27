use bevy_ecs::prelude::{Added, Commands, DetectChanges, Entity, EventReader, Query, Res, ResMut};
use bevy_log::info;

use naia_bevy_server::{
    events::{ConnectEvent, DisconnectEvent, MessageEvents},
    CommandsExt, RoomKey, Server, UserKey,
};
use naia_shared::BigMapKey;

use common_lobby_protocol::{
    bundles::{GameBundle, PlayerBundle},
    components::GameInstance,
    messages::{LobbyMessage, LobbyMessageChannel, LobbyMessageData},
};

use crate::resources::{LobbyState, LobbyStateMap, ReadiedUsers, UserEntityMap};

const ROOM_MAX_PLAYERS: usize = 2;

pub fn handle_user_join_room(
    mut reader: EventReader<ConnectEvent>,
    mut commands: Commands,
    mut server: Server,
    mut global: ResMut<UserEntityMap>,
    mut room_states: ResMut<LobbyStateMap>,
) {
    for ConnectEvent(user_key) in reader.iter() {
        let room_to_join = {
            let room_keys = server.room_keys();
            let existing_room = room_keys
                .iter()
                .find(|key| server.room(key).entities_count() < ROOM_MAX_PLAYERS);
            if let Some(&room_key) = existing_room {
                room_key
            } else {
                let room_key = server.make_room().key();
                global
                    .instance_to_room_map
                    .insert(room_key.to_u64(), room_key);
                room_states.insert(room_key, LobbyState::FindingUsers);
                room_key
            }
        };

        // join the room
        let address = server
            .user_mut(user_key)
            .enter_room(&room_to_join)
            .address();

        info!(
            "Client {} connected to room {}",
            address,
            room_to_join.to_u64()
        );

        // create Entity to represent new player
        let player_num = server.users_count() as u64;
        let PlayerBundle { instance, player } =
            PlayerBundle::new(room_to_join.to_u64(), player_num);

        let entity = commands
            .spawn((instance, player))
            .enable_replication(&mut server)
            .id();
        server.room_mut(&room_to_join).add_entity(&entity);

        // add data to maps
        global.user_to_entity_map.insert(*user_key, entity);
        global.user_to_room_map.insert(*user_key, room_to_join);
        global.entity_to_user_map.insert(entity, *user_key);
        global.entity_to_room_map.insert(entity, room_to_join);

        // Send an EntityAssignment message
        // TODO move to an on_add::<PlayerBundle>
        // let mut assignment_message = EntityAssignment::new();
        // assignment_message.entity.set(&server, &entity);
        // server.send_message::<EntityAssignmentChannel, EntityAssignment>(
        //     user_key,
        //     &assignment_message,
        // );
    }
}

pub fn flush_room_state_updates(server: Server, mut room_states: ResMut<LobbyStateMap>) {
    for room_key in server.room_keys() {
        if room_states.is_changed() {
            let Some(state) = room_states
                .as_mut()
                .0
                .get_mut(&room_key) else { continue; };
            state.flush();
        }
    }
}

pub fn handle_room_ready(server: Server, mut room_states: ResMut<LobbyStateMap>) {
    for room_key in server.room_keys() {
        let room = server.room(&room_key);
        let Some(lobby_state) = room_states.as_mut().0.get_mut(&room_key) else { continue };
        if lobby_state.inner() == LobbyState::FindingUsers && room.users_count() == ROOM_MAX_PLAYERS
        {
            info!(
                "Room {} meets conditions for Setup state.",
                room_key.to_u64()
            );
            lobby_state.set(LobbyState::Setup);
        }
    }
}

pub fn send_room_start_signal(
    mut commands: Commands,
    mut server: Server,
    room_states: Res<LobbyStateMap>,
) {
    for room_key in server.room_keys() {
        let Some(state) = room_states
            .as_ref()
            .0
            .get(&room_key) else { continue; };
        if state.just_changed_to(LobbyState::Setup) {
            info!("Room {} just entered the setup phase", room_key.to_u64());
            let GameBundle { instance, lobby } = GameBundle::new(room_key.to_u64());
            let entity = commands
                .spawn((instance, lobby))
                .enable_replication(&mut server)
                .id();
            server.room_mut(&room_key).add_entity(&entity);

            for user_key in server.user_keys() {
                server.send_message::<LobbyMessageChannel, LobbyMessage>(
                    &user_key,
                    &LobbyMessage::start(),
                );
            }
        }
    }
}

pub fn handle_room_cleanup(mut server: Server, mut room_states: ResMut<LobbyStateMap>) {
    for room_key in server.room_keys() {
        let Some(state) = room_states.as_ref().0.get(&room_key) else { continue; };
        if state.in_state(LobbyState::Gameover) && server.room(&room_key).users_count() == 0 {
            room_states.as_mut().0.remove(&room_key);
            server.room_mut(&room_key).destroy();
        }
    }
}

pub fn receive_state_update_message(
    mut event_reader: EventReader<MessageEvents>,
    mut server: Server,
    mut room_states: ResMut<LobbyStateMap>,
    global: Res<UserEntityMap>,
    mut readied_users: ResMut<ReadiedUsers>,
) {
    for events in event_reader.iter() {
        for (user_key, update) in events.read::<LobbyMessageChannel, LobbyMessage>() {
            let room = global.user_to_room_map.get(&user_key).unwrap();
            let room_state = room_states.0.get_mut(room).unwrap();
            let filter_users_in_room = |user: &UserKey| {
                let room = server.room(room);
                if room.has_user(user) {
                    Some(*user)
                } else {
                    None
                }
            };
            match update.inner() {
                LobbyMessageData::Pause => match room_state.inner() {
                    LobbyState::Paused | LobbyState::Active => {
                        room_state.set(LobbyState::Paused).unwrap();
                        for user in server
                            .user_keys()
                            .iter()
                            .filter_map(filter_users_in_room)
                            .collect::<Vec<UserKey>>()
                            .iter()
                        {
                            server.send_message::<LobbyMessageChannel, LobbyMessage>(
                                user,
                                &LobbyMessage::pause(),
                            );
                        }
                    }
                    _ => {}
                },
                LobbyMessageData::Start => {
                    readied_users.0.insert(user_key);
                    if room_state.inner() != LobbyState::Active
                        && server
                            .user_keys()
                            .iter()
                            .filter_map(filter_users_in_room)
                            .all(|user| readied_users.0.get(&user).is_some())
                    {
                        info!("Game is beginning!");
                        for user in server.user_keys().iter().filter_map(filter_users_in_room) {
                            readied_users.0.remove(&user);
                        }
                        room_state.set(LobbyState::Active);
                    }
                }
            }
        }
    }
}

pub fn remove_user(
    mut reader: EventReader<DisconnectEvent>,
    mut global: ResMut<UserEntityMap>,
    mut server: Server,
    mut commands: Commands,
) {
    for DisconnectEvent(user_key, _) in reader.iter() {
        let Some(entity) = global.user_to_entity_map.remove(user_key) else { continue; };
        let Some(room) = global.user_to_room_map.remove(user_key) else { continue; };
        global.entity_to_user_map.remove(&entity).unwrap();
        global.entity_to_room_map.remove(&entity).unwrap();
        server.room_mut(&room).remove_entity(&entity);
        commands.entity(entity).despawn();
    }
}

pub fn subscribe_rooms_to_game_instance(
    mut server: Server,
    query: Query<(Entity, &GameInstance), Added<GameInstance>>,
) {
    for (entity, instance) in query.iter() {
        server
            .room_mut(&RoomKey::from_u64(*instance.id))
            .add_entity(&entity);
    }
}
