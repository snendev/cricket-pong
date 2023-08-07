use bevy_ecs::{
    prelude::{Component, Query},
    query::ReadOnlyWorldQuery,
};
use bevy_log::{debug, info};
use bevy_transform::prelude::Transform;

use naia_bevy_shared::Replicate;

use cricket_pong_base::components::physics::{Rotation, Translation};

pub fn sync_component<Source, Target, Filter>(mut query: Query<(&Source, &mut Target), Filter>)
where
    Source: Component + std::fmt::Debug,
    Target: Component + std::fmt::Debug + for<'a> From<&'a Source>,
    Filter: ReadOnlyWorldQuery,
{
    for (source, mut target) in query.iter_mut() {
        *target = source.into();
    }
}

pub fn sync_replicated<Source, Target, Filter>(mut query: Query<(&Source, &mut Target), Filter>)
where
    Source: Component + std::fmt::Debug,
    Target: Component + Replicate + std::fmt::Debug + for<'a> From<&'a Source>,
    Filter: ReadOnlyWorldQuery,
{
    for (source, mut target) in query.iter_mut() {
        let update: Target = source.into();
        target.mirror(&update);
    }
}

pub fn sync_transforms_from_replicated<Filter>(
    mut query: Query<(&Translation, &Rotation, &mut Transform), Filter>,
) where
    Filter: ReadOnlyWorldQuery,
{
    for (translation, rotation, mut transform) in query.iter_mut() {
        transform.translation = translation.into();
        transform.rotation = rotation.into();
    }
}

pub fn sync_transforms_to_replicated<Filter>(
    mut query: Query<(&mut Translation, &mut Rotation, &Transform), Filter>,
) where
    Filter: ReadOnlyWorldQuery,
{
    for (mut translation, mut rotation, transform) in query.iter_mut() {
        translation.mirror(&Translation::from(transform.translation));
        rotation.mirror(&Rotation::from(transform.rotation));
    }
}
