use bevy::prelude::{debug, EventReader, Query, ResMut};

use naia_bevy_client::{events::ClientTickEvent, Client};

use cricket_pong_game::base::{
    actions::Actions,
    protocol::{channels::PlayerActionsChannel, messages::ActionMessage},
    rapier::prelude::RapierConfiguration,
};

use crate::networking::{components::SourceOf, resources::TickHistory};

pub fn send_and_prepare_inputs(
    mut client: Client,
    mut physics_config: ResMut<RapierConfiguration>,
    mut tick_reader: EventReader<ClientTickEvent>,
    mut tick_history: ResMut<TickHistory>,
    sources_query: Query<&SourceOf>,
    mut player_actions: ResMut<Actions>,
) -> Vec<(u16, Actions)> {
    let mut ticks = Vec::new();

    if tick_reader.len() > 0 {
        physics_config.force_update_from_transform_changes = false;
    }

    for ClientTickEvent(client_tick) in tick_reader.iter() {
        let mut predicted_actions = Actions::default();

        // N.B. need to ensure inputs are applied to all ticks registered this frame
        for (entity, action) in player_actions.0.clone().into_iter() {
            // Send each command to server
            let mut input_message = ActionMessage::new(Some(action.clone()));
            input_message.entity.set(&client, &entity);
            client.send_tick_buffer_message::<PlayerActionsChannel, ActionMessage>(
                client_tick,
                &input_message,
            );
            if let Ok(SourceOf(prediction)) = sources_query.get(entity) {
                predicted_actions.0.push((*prediction, action));
            } else {
                debug!("Warning: Input received for non-source entity")
            }
        }

        tick_history
            .0
            .insert(*client_tick, predicted_actions.clone());
        // Also proxy actions to TickPlugin
        ticks.push((*client_tick, predicted_actions));
    }

    player_actions.0.clear();

    ticks
}
