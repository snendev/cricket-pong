use bevy_ecs::{
    prelude::{IntoSystemConfigs, IntoSystemSetConfigs, Schedule, SystemSet},
    query::With,
    system::{Query, SystemParam},
};

use bevy_rapier2d::prelude::{
    BevyPhysicsHooks, PairFilterContextView, PhysicsSet as RapierPhysicsSet, RapierPhysicsPlugin,
    SolverFlags,
};

use cricket_pong_base::lobby::components::GameInstance;

use crate::ShouldTick;

#[derive(SystemParam)]
pub(crate) struct InstanceFilter<'w, 's> {
    instance: Query<'w, 's, &'static GameInstance, With<ShouldTick>>,
}

impl BevyPhysicsHooks for InstanceFilter<'_, '_> {
    fn filter_contact_pair(&self, context: PairFilterContextView) -> Option<SolverFlags> {
        let Ok(instance1) = self.instance.get(context.collider1()) else { return None };
        let Ok(instance2) = self.instance.get(context.collider2()) else { return None };
        if *instance1.id == *instance2.id {
            Some(SolverFlags::COMPUTE_IMPULSES)
        } else {
            None
        }
    }
}

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
            RapierPhysicsPlugin::<InstanceFilter>::get_systems(RapierPhysicsSet::SyncBackend)
                .in_set(RapierPhysicsSet::SyncBackend),
        )
        .add_systems(
            RapierPhysicsPlugin::<InstanceFilter>::get_systems(RapierPhysicsSet::SyncBackendFlush)
                .in_set(RapierPhysicsSet::SyncBackendFlush),
        )
        .add_systems(
            RapierPhysicsPlugin::<InstanceFilter>::get_systems(RapierPhysicsSet::StepSimulation)
                .in_set(RapierPhysicsSet::StepSimulation),
        )
        .add_systems(
            RapierPhysicsPlugin::<InstanceFilter>::get_systems(RapierPhysicsSet::Writeback)
                .in_set(RapierPhysicsSet::Writeback),
        );
    schedule
}
