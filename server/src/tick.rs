use bevy_ecs::prelude::EventReader;
use bevy_replicon::prelude::FromClient;

use cricket_pong_game::base::actions::Actions;
use network_base::messages::ActionMessageEvent;

pub fn handle_actions(
    mut tick_reader: EventReader<FromClient<ActionMessageEvent>>,
) -> Vec<(u16, Actions)> {
    vec![(
        0,
        Actions(
            tick_reader
                .iter()
                .flat_map(
                    |FromClient {
                         client_id: _,
                         event,
                     }| {
                        event
                            .action
                            .as_ref()
                            .map(|action| (event.entity, action.clone()))
                    },
                )
                .collect(),
        ),
    )]
}
