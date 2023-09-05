use bevy::prelude::{EventWriter, Local, ResMut};

use cricket_pong_game::base::actions::Actions;
use network_base::messages::ActionMessageEvent;

pub fn send_and_prepare_inputs(
    mut player_actions: ResMut<Actions>,
    mut action_events: EventWriter<ActionMessageEvent>,
    mut tick_count: Local<u16>,
) -> Vec<(u16, Actions)> {
    let mut predicted_actions = Actions::default();

    // N.B. need to ensure inputs are applied to all ticks registered this frame
    for (entity, action) in player_actions.0.clone().into_iter() {
        // Send each command to server
        let input_message = ActionMessageEvent {
            entity,
            action: Some(action.clone()),
        };
        action_events.send(input_message);
        predicted_actions.0.push((entity, action));
    }

    player_actions.0.clear();

    let actions = (*tick_count, predicted_actions);
    *tick_count += 1;

    vec![actions]
}
