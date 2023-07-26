use bevy::prelude::{Entity, EventReader, Query, ResMut};

use naia_bevy_client::{events::UpdateComponentEvents, sequence_greater_than, Replicate, Tick};

use cricket_pong_game::base::{
    actions::Actions,
    physics::{Position, Velocity},
};

use super::{components::SourceOf, resources::TickHistory};

pub fn receive_update_component_events(
    mut event_reader: EventReader<UpdateComponentEvents>,
    mut tick_history: ResMut<TickHistory>,
    sources_query: Query<(&Position, &Velocity, &SourceOf)>,
    mut predictions_query: Query<(Entity, &mut Position, &mut Velocity)>,
) -> Vec<(u16, Actions)> {
    // We only care about whatever the latest tick is
    // so we check the events for the latest tick count,
    // and use that to get the commands we need to replay
    let mut latest_tick: Option<Tick> = None;
    for events in event_reader.iter() {
        for (server_tick, _entity) in events.read::<Position>() {
            if let Some(last_tick) = latest_tick {
                if sequence_greater_than(server_tick, last_tick) {
                    latest_tick = Some(server_tick);
                }
            } else {
                latest_tick = Some(server_tick);
            }
        }
    }
    if let Some(latest_tick) = latest_tick {
        // Reset all expected entities to their source states
        for (source_position, source_velocity, SourceOf(prediction)) in sources_query.iter() {
            if let Ok((_, mut position, mut velocity)) = predictions_query.get_mut(*prediction) {
                position.mirror(source_position);
                velocity.mirror(source_velocity);
            }
        }
        // Then replay ticks
        let mut replays = tick_history.0.replays(&latest_tick);
        replays.reverse();
        replays
    } else {
        Vec::new()
    }
}
