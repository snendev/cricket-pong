use bevy_ecs::{
    event::EventReader,
    query::Changed,
    system::{Local, Query},
};
use bevy_utils::HashMap;

use cricket_pong_game::{
    base::{
        actions::Actions,
        components::scoreboard::Scoreboard,
        protocol::{
            channels::{PlayerActionsChannel, ScoreMessageChannel},
            messages::{ActionMessage, ScoreMessage},
        },
    },
    GameInstance,
};

pub fn tick_events(
    mut server: Server,
    mut tick_reader: EventReader<TickEvent>,
) -> Vec<(u16, Actions)> {
    let mut tick_actions = Vec::new();

    for TickEvent(server_tick) in tick_reader.iter() {
        let mut actions = Actions(Vec::new());
        let mut messages = server.receive_tick_buffer_messages(server_tick);
        for (_user_key, command) in messages.read::<PlayerActionsChannel, ActionMessage>() {
            let Some(entity) = command.entity.get(&server) else { continue };
            if let Some(action) = command.action {
                actions.0.push((entity, action));
            }
        }
        tick_actions.push((*server_tick, actions));
    }

    tick_actions
}

pub fn send_score_mesasges(
    mut server: Server,
    updated_scores_query: Query<(&GameInstance, &Scoreboard), Changed<Scoreboard>>,
    mut last_score_index_by_instance_id: Local<HashMap<u64, usize>>,
) {
    for (instance, scoreboard) in updated_scores_query.iter() {
        let users_in_room = {
            let room = server.room(&RoomKey::from_u64(*instance.id));
            room.user_keys().cloned().collect::<Vec<_>>()
        };

        if let Some(last_score_index) = last_score_index_by_instance_id.get_mut(&*instance.id) {
            for index in *last_score_index..scoreboard.len() {
                let score = scoreboard.get(index).unwrap();
                for user_key in users_in_room.iter() {
                    server.send_message::<ScoreMessageChannel, ScoreMessage>(
                        user_key,
                        &ScoreMessage::new(*score, index),
                    );
                }
            }
            *last_score_index = scoreboard.len();
        } else {
            last_score_index_by_instance_id.insert(*instance.id, scoreboard.len());
        }
    }
}

pub fn update_entity_scopes(mut server: Server, mut tick_reader: EventReader<TickEvent>) {
    if !tick_reader.iter().count() != 0 {
        // Update entity scopes
        for (_room_key, user_key, entity) in server.scope_checks() {
            server.user_scope(&user_key).include(&entity);
        }
    }
}
