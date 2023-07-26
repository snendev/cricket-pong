use bevy_ecs::prelude::{Component, Query};

pub fn sync_components<Source, Target>(mut query: Query<(&mut Target, &Source)>)
where
    Source: Component + std::fmt::Debug,
    Target: Component + std::fmt::Debug + for<'a> From<&'a Source>,
{
    for (mut target, source) in &mut query {
        // info!("Syncing source {:?} to {:?}", source, target);
        *target = source.into();
    }
}
