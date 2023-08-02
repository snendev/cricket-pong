use itertools::Itertools;

use bevy_ecs::prelude::{Commands, ResMut};
use bevy_log::{debug, info};

use naia_bevy_server::{CommandsExt, Server};
use naia_shared::BigMapKey;

use common_lobby_server::resources::{QueuedUsers, UserEntities};

use cricket_pong_game::{
    base::{
        components::player::{PlayerOne, PlayerTwo},
        protocol::{channels::PlayerAssignmentChannel, messages::PlayerAssignmentMessage},
    },
    lobby::components::{GameInstance, GameLobby},
    ShouldTick,
};

pub fn pair_queued_users(
    mut commands: Commands,
    mut server: Server,
    mut user_entities: ResMut<UserEntities>,
    mut queued_players: ResMut<QueuedUsers>,
) {
    let num_queued = queued_players.users.len();
    let num_matched = num_queued - (num_queued % 2);
    for (user_one, user_two) in queued_players.users.drain(0..num_matched).tuples() {
        let room_key = server.make_room().key();
        debug!(
            "Game starting in room {} with users {} (one) and {} (two)",
            room_key.to_u64(),
            user_one.to_u64(),
            user_two.to_u64(),
        );
        let instance = GameInstance::new(room_key.to_u64());

        // player one
        let player_one_entity = commands
            .spawn((PlayerOne, PlayerOne::name(), instance.clone(), ShouldTick))
            .enable_replication(&mut server)
            .id();
        server
            .room_mut(&room_key)
            .add_user(&user_one)
            .add_entity(&player_one_entity);
        user_entities.insert(user_one, player_one_entity);
        let mut assignment_message = PlayerAssignmentMessage::new();
        assignment_message.entity.set(&server, &player_one_entity);
        server.send_message::<PlayerAssignmentChannel, PlayerAssignmentMessage>(
            &user_one,
            &assignment_message,
        );

        // player two
        let player_two_entity = commands
            .spawn((PlayerTwo, PlayerTwo::name(), instance.clone(), ShouldTick))
            .enable_replication(&mut server)
            .id();
        server
            .room_mut(&room_key)
            .add_entity(&player_two_entity)
            .add_user(&user_two);
        user_entities.insert(user_two, player_two_entity);
        let mut assignment_message = PlayerAssignmentMessage::new();
        assignment_message.entity.set(&server, &player_two_entity);
        server.send_message::<PlayerAssignmentChannel, PlayerAssignmentMessage>(
            &user_two,
            &assignment_message,
        );

        // spawn lobby
        let lobby_entity = commands
            .spawn((
                GameLobby::default(),
                GameLobby::name(),
                instance,
                ShouldTick,
            ))
            .enable_replication(&mut server)
            .id();
        server.room_mut(&room_key).add_entity(&lobby_entity);
    }
}
