use bevy_ecs::{
    prelude::{In, IntoSystemConfigs, IntoSystemSetConfig, Schedule, SystemSet, World},
    query::With,
    schedule::ScheduleLabel,
};
use bevy_transform::prelude::Transform;

use bevy_rapier2d::prelude::{ExternalImpulse, Velocity};

use cricket_pong_base::{
    actions::Actions,
    components::physics::{
        ExternalImpulse as SyncImpulse, Transform as SyncTransform, Velocity as SyncVelocity,
    },
};

use crate::{
    systems::{scoring, sync, tick},
    ShouldTick,
};

mod physics;
pub(crate) use physics::InstanceFilter;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct SyncPhysicsSet;
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct SyncInternalsSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
pub struct CoreTickSchedule;

pub(crate) fn build_core_tick_schedule() -> (CoreTickSchedule, Schedule) {
    let schedule = Schedule::new();
    let mut schedule = physics::build_physics_schedule(schedule);
    schedule
        .configure_sets((
            tick::ActionsSet.before(physics::PhysicsSet),
            scoring::ScoringSet.after(physics::PhysicsSet),
            SyncPhysicsSet
                .after(tick::ActionsSet)
                .before(physics::PhysicsSet),
            SyncInternalsSet.before(tick::ActionsSet),
        ))
        .add_systems((
            (tick::track_bowler_transform, tick::consume_actions)
                .chain()
                .in_set(tick::ActionsSet),
            (
                sync::sync_components::<SyncTransform, Transform, With<ShouldTick>>,
                sync::sync_components::<SyncVelocity, Velocity, With<ShouldTick>>,
                sync::sync_components::<SyncImpulse, ExternalImpulse, With<ShouldTick>>,
            )
                .in_set(SyncPhysicsSet),
            (
                sync::sync_components::<Transform, SyncTransform, With<ShouldTick>>,
                sync::sync_components::<Velocity, SyncVelocity, With<ShouldTick>>,
                sync::sync_components::<ExternalImpulse, SyncImpulse, With<ShouldTick>>,
            )
                .in_set(SyncInternalsSet),
            scoring::register_goals.in_set(scoring::ScoringSet),
        ));
    (CoreTickSchedule, schedule)
}

pub(crate) fn run_core_game_loop(In(ticks): In<Vec<(u16, Actions)>>, world: &mut World) {
    for (_tick, tick_actions) in ticks {
        let mut inputs = world.resource_mut::<Actions>();
        inputs.0 = tick_actions.0;
        world.run_schedule(CoreTickSchedule);
    }
}
