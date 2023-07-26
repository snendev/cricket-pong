use bevy_ecs::prelude::{IntoSystemConfigs, IntoSystemSetConfigs, Schedule, SystemSet};
use bevy_rapier2d::prelude::{PhysicsSet as RapierPhysicsSet, RapierPhysicsPlugin};

// struct to group all RapierPhysicsSet sets so that other plugins can simply order
// relative to one set without info about RapierPhysicsSet
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct PhysicsSet;

pub(crate) fn build_physics_schedule(mut schedule: Schedule) -> Schedule {
    schedule
        .configure_sets(
            (
                RapierPhysicsSet::SyncBackend,
                RapierPhysicsSet::SyncBackendFlush,
                RapierPhysicsSet::StepSimulation,
                RapierPhysicsSet::Writeback,
            )
                .chain()
                .in_set(PhysicsSet),
        )
        .add_systems(
            RapierPhysicsPlugin::<()>::get_systems(RapierPhysicsSet::SyncBackend)
                .in_set(RapierPhysicsSet::SyncBackend),
        )
        .add_systems(
            RapierPhysicsPlugin::<()>::get_systems(RapierPhysicsSet::SyncBackendFlush)
                .in_set(RapierPhysicsSet::SyncBackendFlush),
        )
        .add_systems(
            RapierPhysicsPlugin::<()>::get_systems(RapierPhysicsSet::StepSimulation)
                .in_set(RapierPhysicsSet::StepSimulation),
        )
        .add_systems(
            RapierPhysicsPlugin::<()>::get_systems(RapierPhysicsSet::Writeback)
                .in_set(RapierPhysicsSet::Writeback),
        );
    schedule
}
