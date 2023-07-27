use bevy::prelude::{info, EventReader, Query, ResMut};

use naia_bevy_client::{events::ClientTickEvent, Client};

use cricket_pong_game::base::{
    actions::Actions,
    protocol::{channels::PlayerActionsChannel, messages::ActionMessage},
};

use super::{components::SourceOf, resources::TickHistory};

pub fn send_and_prepare_inputs(
    mut client: Client,
    mut tick_reader: EventReader<ClientTickEvent>,
    mut tick_history: ResMut<TickHistory>,
    sources_query: Query<&SourceOf>,
    mut player_actions: ResMut<Actions>,
) -> Vec<(u16, Actions)> {
    let mut ticks = Vec::new();

    for ClientTickEvent(client_tick) in tick_reader.iter() {
        info!("Client tick {}", client_tick);
        let mut predicted_actions = Actions::default();

        for (entity, action) in player_actions.0.drain(..) {
            // Send each command to server
            let mut input_message = ActionMessage::new(Some(action.clone()));
            input_message.entity.set(&client, &entity);
            client.send_tick_buffer_message::<PlayerActionsChannel, ActionMessage>(
                client_tick,
                &input_message,
            );
            if let Ok(SourceOf(prediction)) = sources_query.get(entity) {
                predicted_actions.0.push((*prediction, action));
            }
        }

        tick_history
            .0
            .insert(*client_tick, predicted_actions.clone());
        // Also proxy actions to TickPlugin
        ticks.push((*client_tick, predicted_actions));
    }

    ticks
}
