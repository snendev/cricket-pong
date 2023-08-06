use bevy_ecs::{
    prelude::{Component, Query},
    query::ReadOnlyWorldQuery,
};
use naia_bevy_shared::Replicate;

pub fn sync_component<Source, Target, Filter>(mut query: Query<(&Source, &mut Target)>)
where
    Source: Component + std::fmt::Debug,
    Target: Component + std::fmt::Debug + for<'a> From<&'a Source>,
    Filter: ReadOnlyWorldQuery,
{
    for (source, mut target) in &mut query {
        *target = source.into();
    }
}

pub fn sync_replicated<Source, Target, Filter>(mut query: Query<(&Source, &mut Target)>)
where
    Source: Component + std::fmt::Debug,
    Target: Component + Replicate + std::fmt::Debug + for<'a> From<&'a Source>,
    Filter: ReadOnlyWorldQuery,
{
    for (source, mut target) in &mut query {
        let update: Target = source.into();
        target.mirror(&update);
    }
}
