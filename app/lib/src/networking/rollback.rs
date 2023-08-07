use bevy::prelude::{Component, EventReader, Query, ResMut, With, Without};

use naia_bevy_client::{events::UpdateComponentEvents, sequence_greater_than, Replicate, Tick};

use cricket_pong_game::base::{
    actions::Actions,
    components::{
        batter::Batter,
        physics::{ExternalImpulse, Rotation, Translation, Velocity},
    },
    rapier::prelude::RapierConfiguration,
};

use crate::networking::{
    components::{PredictionOf, SourceOf},
    resources::TickHistory,
};

fn get_latest_tick(mut event_reader: EventReader<UpdateComponentEvents>) -> Option<Tick> {
    let mut latest_tick: Option<Tick> = None;
    for events in event_reader.iter() {
        for (server_tick, _entity) in events
            .read::<Translation>()
            .into_iter()
            .chain(events.read::<Rotation>())
            .chain(events.read::<Velocity>())
            .chain(events.read::<ExternalImpulse>())
            .chain(events.read::<Batter>())
        {
            if let Some(last_tick) = latest_tick {
                if sequence_greater_than(server_tick, last_tick) {
                    latest_tick = Some(server_tick);
                }
            } else {
                latest_tick = Some(server_tick);
            }
        }
    }
    latest_tick
}

// A helper system that iterates through all SourceOf entities with some component T and mirrors
// its values to the corresponding PredictionOf component
fn mirror_source_components<T: Component + Replicate>(
    mut prediction_query: Query<&mut T, (With<PredictionOf>, Without<SourceOf>)>,
    source_query: Query<(&T, &SourceOf)>,
) {
    for (source, SourceOf(prediction)) in source_query.iter() {
        if let Ok(mut prediction) = prediction_query.get_mut(*prediction) {
            prediction.mirror(source);
        }
    }
}

// N.B. Be sure to run this system on all stateful replicated components before receive_update_component_events!
pub fn rollback_component<T: Component + Replicate>(
    event_reader: EventReader<UpdateComponentEvents>,
    sources_query: Query<(&T, &SourceOf)>,
    predictions_query: Query<&mut T, (With<PredictionOf>, Without<SourceOf>)>,
) {
    let latest_tick = get_latest_tick(event_reader);
    if latest_tick.is_some() {
        // Reset all expected entities to their source states
        mirror_source_components(predictions_query, sources_query);
    }
}

pub fn replay_ticks(
    event_reader: EventReader<UpdateComponentEvents>,
    mut physics_config: ResMut<RapierConfiguration>,
    mut tick_history: ResMut<TickHistory>,
) -> Vec<(u16, Actions)> {
    // We only care about whatever the latest tick is
    // so we check the events for the latest tick count,
    // and use that to get the commands we need to replay
    let latest_tick = get_latest_tick(event_reader);
    if let Some(latest_tick) = latest_tick {
        physics_config.force_update_from_transform_changes = true;
        let mut replays = tick_history.0.replays(&latest_tick);
        replays.reverse();
        replays
    } else {
        Vec::new()
    }
}
