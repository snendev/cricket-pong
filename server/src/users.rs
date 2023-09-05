use itertools::Itertools;

use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    prelude::{EventReader, ResMut, Resource},
    query::{Added, Or},
    system::{Commands, Local, Query},
};
use bevy_log::debug;

use bevy_replicon::{
    prelude::{SendMode, ToClients},
    renet::ServerEvent,
};

use cricket_pong_game::base::components::{
    instance::{GameInstance, GameLobby, PlayerID},
    player::{Identity, PlayerOne, PlayerTwo},
};
use network_base::messages::PlayerAssignmentMessageEvent;

#[derive(Clone, Debug, Default, Resource)]
pub(crate) struct QueuedUsers(Vec<u64>);

pub(crate) fn handle_user_connections(
    mut server_events: EventReader<ServerEvent>,
    mut queued_players: ResMut<QueuedUsers>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                debug!("Queued for matchmaking: New client {} connected", client_id);
                queued_players.0.push(*client_id);
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
    mut queued_players: ResMut<QueuedUsers>,
    mut next_room_key: Local<u64>,
) {
    let num_queued = queued_players.0.len();
    let num_matched = num_queued - (num_queued % 2);
    for (user_one, user_two) in queued_players.0.drain(0..num_matched).tuples() {
        let room_key = *next_room_key;
        *next_room_key += 1;

        debug!(
            "Game starting in room {} with users {} (one) and {} (two)",
            room_key, user_one, user_one,
        );
        let instance = GameInstance::new(room_key);

        // player one
        commands.spawn((
            PlayerOne,
            PlayerOne::name(),
            PlayerID::new(user_one),
            instance.clone(),
        ));
        commands.spawn((
            PlayerTwo,
            PlayerTwo::name(),
            PlayerID::new(user_two),
            instance.clone(),
        ));

        // spawn lobby
        commands.spawn((GameLobby::default(), GameLobby::name(), instance));
    }
}

type WithAddedPlayer = Or<(Added<PlayerID>, Added<PlayerOne>, Added<PlayerTwo>)>;

pub(crate) fn send_player_assignment_messages(
    query: Query<(&PlayerID, Option<&PlayerOne>, Option<&PlayerTwo>), WithAddedPlayer>,
    mut assignment_event_writer: EventWriter<ToClients<PlayerAssignmentMessageEvent>>,
) {
    for (player, maybe_p1, maybe_p2) in query.iter() {
        let identity = match (maybe_p1, maybe_p2) {
            (Some(_), _) => Identity::One,
            (_, Some(_)) => Identity::Two,
            _ => continue,
        };
        assignment_event_writer.send(ToClients {
            mode: SendMode::Direct(player.id),
            event: PlayerAssignmentMessageEvent::new(identity),
        });
    }
}
