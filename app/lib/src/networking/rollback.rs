use bevy::prelude::{info, EventReader, Query, ResMut, Without};

use naia_bevy_client::{events::UpdateComponentEvents, sequence_greater_than, Replicate, Tick};

use cricket_pong_game::base::{
    actions::Actions,
    components::physics::{ExternalImpulse, Transform, Velocity},
};

use super::{components::SourceOf, resources::TickHistory};

pub fn receive_update_component_events(
    mut event_reader: EventReader<UpdateComponentEvents>,
    mut tick_history: ResMut<TickHistory>,
    transform_sources_query: Query<(&Transform, &SourceOf)>,
    mut transform_predictions_query: Query<&mut Transform, Without<SourceOf>>,
    velocity_sources_query: Query<(&Velocity, &SourceOf)>,
    mut velocity_predictions_query: Query<&mut Velocity, Without<SourceOf>>,
    impulse_sources_query: Query<(&ExternalImpulse, &SourceOf)>,
    mut impulse_predictions_query: Query<&mut ExternalImpulse, Without<SourceOf>>,
) -> Vec<(u16, Actions)> {
    // We only care about whatever the latest tick is
    // so we check the events for the latest tick count,
    // and use that to get the commands we need to replay
    let mut latest_tick: Option<Tick> = None;
    for events in event_reader.iter() {
        for (server_tick, _entity) in events.read::<Velocity>() {
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
        info!("Rolling back from tick {}", latest_tick);
        // Reset all expected entities to their source states
        for (source_transform, SourceOf(prediction)) in transform_sources_query.iter() {
            if let Ok(mut transform) = transform_predictions_query.get_mut(*prediction) {
                transform.mirror(source_transform);
            }
        }
        for (source_velocity, SourceOf(prediction)) in velocity_sources_query.iter() {
            if let Ok(mut velocity) = velocity_predictions_query.get_mut(*prediction) {
                velocity.mirror(source_velocity);
            }
        }
        for (source_impulse, SourceOf(prediction)) in impulse_sources_query.iter() {
            if let Ok(mut impulse) = impulse_predictions_query.get_mut(*prediction) {
                impulse.mirror(source_impulse);
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
