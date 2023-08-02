use bevy_ecs::{
    prelude::{Component, Query},
    query::ReadOnlyWorldQuery,
};

pub fn sync_components<Source, Target, Filter>(mut query: Query<(&mut Target, &Source)>)
where
    Source: Component + std::fmt::Debug,
    Target: Component + std::fmt::Debug + for<'a> From<&'a Source>,
    Filter: ReadOnlyWorldQuery,
{
    for (mut target, source) in &mut query {
        *target = source.into();
    }
}
